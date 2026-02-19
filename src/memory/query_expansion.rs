use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

static STOP_WORDS_EN: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    for word in &[
        "a",
        "an",
        "the",
        "this",
        "that",
        "these",
        "those",
        "i",
        "me",
        "my",
        "we",
        "our",
        "you",
        "your",
        "he",
        "she",
        "it",
        "they",
        "them",
        "is",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "do",
        "does",
        "did",
        "will",
        "would",
        "could",
        "should",
        "can",
        "may",
        "might",
        "in",
        "on",
        "at",
        "to",
        "for",
        "of",
        "with",
        "by",
        "from",
        "about",
        "into",
        "through",
        "during",
        "before",
        "after",
        "above",
        "below",
        "between",
        "under",
        "over",
        "and",
        "or",
        "but",
        "if",
        "then",
        "because",
        "as",
        "while",
        "when",
        "where",
        "what",
        "which",
        "who",
        "how",
        "why",
        "yesterday",
        "today",
        "tomorrow",
        "earlier",
        "later",
        "recently",
        "before",
        "ago",
        "just",
        "now",
        "thing",
        "things",
        "stuff",
        "something",
        "anything",
        "everything",
        "nothing",
        "please",
        "help",
        "find",
        "show",
        "get",
        "tell",
        "give",
    ] {
        set.insert(*word);
    }
    set
});

static STOP_WORDS_ZH: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    for word in &[
        "我",
        "我们",
        "你",
        "你们",
        "他",
        "她",
        "它",
        "他们",
        "这",
        "那",
        "这个",
        "那个",
        "这些",
        "那些",
        "的",
        "了",
        "着",
        "过",
        "得",
        "地",
        "吗",
        "呢",
        "吧",
        "啊",
        "呀",
        "嘛",
        "啦",
        "是",
        "有",
        "在",
        "被",
        "把",
        "给",
        "让",
        "用",
        "到",
        "去",
        "来",
        "做",
        "说",
        "看",
        "找",
        "想",
        "要",
        "能",
        "会",
        "可以",
        "和",
        "与",
        "或",
        "但",
        "但是",
        "因为",
        "所以",
        "如果",
        "虽然",
        "而",
        "也",
        "都",
        "就",
        "还",
        "又",
        "再",
        "才",
        "只",
        "之前",
        "以前",
        "之后",
        "以后",
        "刚才",
        "现在",
        "昨天",
        "今天",
        "明天",
        "最近",
        "东西",
        "事情",
        "事",
        "什么",
        "哪个",
        "哪些",
        "怎么",
        "为什么",
        "多少",
        "请",
        "帮",
        "帮忙",
        "告诉",
    ] {
        set.insert(*word);
    }
    set
});

static PUNCTUATION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[\p{P}\p{S}]+$").unwrap());
static CJK_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\u4e00-\u9fff]").unwrap());

fn is_valid_keyword(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    if token.chars().all(|c| c.is_ascii_alphabetic()) && token.len() < 3 {
        return false;
    }

    if token.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if PUNCTUATION_RE.is_match(token) {
        return false;
    }

    true
}

fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let normalized = text.to_lowercase().trim().to_string();

    let segments: Vec<&str> = normalized
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|s| !s.is_empty())
        .collect();

    for segment in segments {
        if CJK_RE.is_match(segment) {
            let chars: Vec<char> = segment
                .chars()
                .filter(|c| CJK_RE.is_match(&c.to_string()))
                .collect();

            for c in &chars {
                tokens.push(c.to_string());
            }

            for i in 0..chars.len().saturating_sub(1) {
                tokens.push(format!("{}{}", chars[i], chars[i + 1]));
            }
        } else {
            tokens.push(segment.to_string());
        }
    }

    tokens
}

pub fn extract_keywords(query: &str) -> Vec<String> {
    let tokens = tokenize(query);
    let mut keywords = Vec::new();
    let mut seen = HashSet::new();

    for token in tokens {
        if STOP_WORDS_EN.contains(token.as_str()) || STOP_WORDS_ZH.contains(token.as_str()) {
            continue;
        }

        if !is_valid_keyword(&token) {
            continue;
        }

        if seen.contains(&token) {
            continue;
        }

        seen.insert(token.clone());
        keywords.push(token);
    }

    keywords
}

pub fn expand_query_for_fts(query: &str) -> (String, Vec<String>, String) {
    let original = query.trim().to_string();
    let keywords = extract_keywords(&original);

    let expanded = if !keywords.is_empty() {
        format!("{} OR {}", original, keywords.join(" OR "))
    } else {
        original.clone()
    };

    (original, keywords, expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords_english() {
        let keywords = extract_keywords("that thing we discussed about the API");
        assert!(keywords.contains(&"discussed".to_string()));
        assert!(keywords.contains(&"api".to_string()));
        assert!(!keywords.contains(&"the".to_string()));
    }

    #[test]
    fn test_extract_keywords_chinese() {
        let keywords = extract_keywords("之前讨论的那个方案");
        assert!(keywords.contains(&"讨论".to_string()));
        assert!(keywords.contains(&"方案".to_string()));
        assert!(!keywords.contains(&"的".to_string()));
    }

    #[test]
    fn test_expand_query_for_fts() {
        let (original, keywords, expanded) =
            expand_query_for_fts("what was the solution for the bug");
        assert_eq!(original, "what was the solution for the bug");
        assert!(keywords.contains(&"solution".to_string()));
        assert!(keywords.contains(&"bug".to_string()));
        assert!(expanded.contains("OR"));
    }
}
