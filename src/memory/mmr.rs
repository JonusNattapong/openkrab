use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMRItem {
    pub id: String,
    pub score: f64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMRConfig {
    pub enabled: bool,
    pub lambda: f64,
}

impl Default for MMRConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            lambda: 0.7,
        }
    }
}

pub fn tokenize(text: &str) -> HashSet<String> {
    let re = regex::Regex::new(r"[a-z0-9_]+").unwrap();
    re.find_iter(&text.to_lowercase())
        .map(|m| m.as_str().to_string())
        .collect()
}

pub fn jaccard_similarity(set_a: &HashSet<String>, set_b: &HashSet<String>) -> f64 {
    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }
    if set_a.is_empty() || set_b.is_empty() {
        return 0.0;
    }

    let (smaller, larger) = if set_a.len() <= set_b.len() {
        (set_a, set_b)
    } else {
        (set_b, set_a)
    };

    let intersection_size = smaller.iter().filter(|item| larger.contains(*item)).count();
    let union_size = set_a.len() + set_b.len() - intersection_size;

    if union_size == 0 {
        0.0
    } else {
        intersection_size as f64 / union_size as f64
    }
}

pub fn text_similarity(content_a: &str, content_b: &str) -> f64 {
    jaccard_similarity(&tokenize(content_a), &tokenize(content_b))
}

fn max_similarity_to_selected(
    item: &MMRItem,
    selected_items: &[MMRItem],
    token_cache: &mut HashMap<String, HashSet<String>>,
) -> f64 {
    if selected_items.is_empty() {
        return 0.0;
    }

    let item_tokens = token_cache
        .entry(item.id.clone())
        .or_insert_with(|| tokenize(&item.content))
        .clone();

    let mut max_sim = 0.0;
    for selected in selected_items {
        let selected_tokens = token_cache
            .entry(selected.id.clone())
            .or_insert_with(|| tokenize(&selected.content));
        let sim = jaccard_similarity(&item_tokens, selected_tokens);
        if sim > max_sim {
            max_sim = sim;
        }
    }
    max_sim
}

pub fn compute_mmr_score(relevance: f64, max_similarity: f64, lambda: f64) -> f64 {
    lambda * relevance - (1.0 - lambda) * max_similarity
}

pub fn mmr_rerank<T: Into<MMRItem> + Clone>(items: Vec<T>, config: Option<MMRConfig>) -> Vec<T>
where
    T: Clone,
{
    let cfg = config.unwrap_or_default();

    let mmr_items: Vec<MMRItem> = items.iter().map(|item| item.clone().into()).collect();

    if !cfg.enabled || items.len() <= 1 {
        return items;
    }

    let clamped_lambda = cfg.lambda.clamp(0.0, 1.0);

    if (clamped_lambda - 1.0).abs() < f64::EPSILON {
        let mut sorted = items;
        sorted.sort_by(|a, b| {
            let a_score = a.clone().into().score;
            let b_score = b.clone().into().score;
            b_score.partial_cmp(&a_score).unwrap()
        });
        return sorted;
    }

    let mut token_cache: HashMap<String, HashSet<String>> = HashMap::new();
    for item in &mmr_items {
        token_cache.insert(item.id.clone(), tokenize(&item.content));
    }

    let scores: Vec<f64> = mmr_items.iter().map(|i| i.score).collect();
    let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
    let score_range = max_score - min_score;

    let normalize_score = |score: f64| -> f64 {
        if score_range.abs() < f64::EPSILON {
            1.0
        } else {
            (score - min_score) / score_range
        }
    };

    let mut selected_indices: Vec<usize> = Vec::new();
    let mut remaining: HashSet<usize> = (0..items.len()).collect();

    while !remaining.is_empty() {
        let mut best_idx: Option<usize> = None;
        let mut best_mmr_score = f64::NEG_INFINITY;

        for &idx in &remaining {
            let item = &mmr_items[idx];
            let normalized_relevance = normalize_score(item.score);
            let selected_items: Vec<MMRItem> = selected_indices
                .iter()
                .map(|&i| mmr_items[i].clone())
                .collect();
            let max_sim = max_similarity_to_selected(item, &selected_items, &mut token_cache);
            let mmr_score = compute_mmr_score(normalized_relevance, max_sim, clamped_lambda);

            let is_better = mmr_score > best_mmr_score
                || (mmr_score - best_mmr_score).abs() < f64::EPSILON
                    && item.score
                        > mmr_items
                            .get(best_idx.unwrap_or(0))
                            .map(|i| i.score)
                            .unwrap_or(f64::NEG_INFINITY);

            if is_better {
                best_mmr_score = mmr_score;
                best_idx = Some(idx);
            }
        }

        if let Some(idx) = best_idx {
            selected_indices.push(idx);
            remaining.remove(&idx);
        } else {
            break;
        }
    }

    selected_indices
        .into_iter()
        .map(|i| items[i].clone())
        .collect()
}

pub fn apply_mmr_to_results<T: Clone + Into<MMRItem> + WithScoreAndSnippet>(
    results: Vec<T>,
    config: Option<MMRConfig>,
) -> Vec<T> {
    if results.is_empty() {
        return results;
    }

    let mmr_items: Vec<MMRItem> = results
        .iter()
        .enumerate()
        .map(|(index, r)| MMRItem {
            id: format!("{}:{}:{}", r.path(), r.start_line(), index),
            score: r.score(),
            content: r.snippet().to_string(),
        })
        .collect();

    let reranked_indices = {
        let cfg = config.unwrap_or_default();
        if !cfg.enabled || mmr_items.len() <= 1 {
            return results;
        }

        let mut token_cache: HashMap<String, HashSet<String>> = HashMap::new();
        for item in &mmr_items {
            token_cache.insert(item.id.clone(), tokenize(&item.content));
        }

        let scores: Vec<f64> = mmr_items.iter().map(|i| i.score).collect();
        let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let score_range = max_score - min_score;

        let normalize_score = |score: f64| -> f64 {
            if score_range.abs() < f64::EPSILON {
                1.0
            } else {
                (score - min_score) / score_range
            }
        };

        let mut selected: Vec<usize> = Vec::new();
        let mut remaining: HashSet<usize> = (0..mmr_items.len()).collect();

        while !remaining.is_empty() {
            let mut best_idx: Option<usize> = None;
            let mut best_mmr_score = f64::NEG_INFINITY;

            for &idx in &remaining {
                let item = &mmr_items[idx];
                let normalized_relevance = normalize_score(item.score);
                let selected_items: Vec<MMRItem> =
                    selected.iter().map(|&i| mmr_items[i].clone()).collect();
                let max_sim = max_similarity_to_selected(item, &selected_items, &mut token_cache);
                let mmr_score = compute_mmr_score(normalized_relevance, max_sim, cfg.lambda);

                if mmr_score > best_mmr_score {
                    best_mmr_score = mmr_score;
                    best_idx = Some(idx);
                }
            }

            if let Some(idx) = best_idx {
                selected.push(idx);
                remaining.remove(&idx);
            } else {
                break;
            }
        }

        selected
    };

    reranked_indices
        .into_iter()
        .map(|i| results[i].clone())
        .collect()
}

pub trait WithScoreAndSnippet {
    fn score(&self) -> f64;
    fn snippet(&self) -> &str;
    fn path(&self) -> &str;
    fn start_line(&self) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello World Test");
        assert!(tokens.contains("hello"));
        assert!(tokens.contains("world"));
        assert!(tokens.contains("test"));
    }

    #[test]
    fn test_jaccard_similarity() {
        let a: HashSet<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        let b: HashSet<String> = ["b", "c", "d"].iter().map(|s| s.to_string()).collect();
        let sim = jaccard_similarity(&a, &b);
        assert!((sim - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_compute_mmr_score() {
        let score = compute_mmr_score(1.0, 0.5, 0.7);
        assert!((score - 0.55).abs() < 0.001);
    }
}
