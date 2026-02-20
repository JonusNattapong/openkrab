#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocationSource {
    Pin,
    Place,
    Live,
}

#[derive(Debug, Clone)]
pub struct NormalizedLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: Option<f64>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub is_live: Option<bool>,
    pub source: Option<LocationSource>,
    pub caption: Option<String>,
}

#[derive(Debug, Clone)]
struct ResolvedLocation {
    latitude: f64,
    longitude: f64,
    accuracy: Option<f64>,
    name: Option<String>,
    address: Option<String>,
    is_live: bool,
    source: LocationSource,
    caption: Option<String>,
}

fn resolve_location(location: &NormalizedLocation) -> ResolvedLocation {
    use LocationSource::*;
    let source = match &location.source {
        Some(LocationSource::Live) => Live,
        Some(LocationSource::Place) => Place,
        Some(LocationSource::Pin) => Pin,
        None => {
            if location.is_live.unwrap_or(false) {
                Live
            } else if location.name.is_some() || location.address.is_some() {
                Place
            } else {
                Pin
            }
        }
    };
    let is_live = location.is_live.unwrap_or(source == LocationSource::Live);
    ResolvedLocation {
        latitude: location.latitude,
        longitude: location.longitude,
        accuracy: location.accuracy,
        name: location.name.clone(),
        address: location.address.clone(),
        is_live,
        source,
        caption: location.caption.clone(),
    }
}

fn format_accuracy(accuracy: Option<f64>) -> String {
    match accuracy {
        Some(a) if a.is_finite() => format!(" ±{}m", a.round() as i64),
        _ => String::new(),
    }
}

fn format_coords(latitude: f64, longitude: f64) -> String {
    format!("{:.6}, {:.6}", latitude, longitude)
}

pub fn format_location_text(location: &NormalizedLocation) -> String {
    let resolved = resolve_location(location);
    let coords = format_coords(resolved.latitude, resolved.longitude);
    let accuracy = format_accuracy(resolved.accuracy);
    let caption = resolved
        .caption
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    let header = if resolved.source == LocationSource::Live || resolved.is_live {
        format!("🛰 Live location: {}{}", coords, accuracy)
    } else if resolved.name.is_some() || resolved.address.is_some() {
        let label = [resolved.name.as_deref(), resolved.address.as_deref()]
            .iter()
            .filter_map(|x| *x)
            .collect::<Vec<_>>()
            .join(" — ");
        format!("📍 {} ({}{})", label, coords, accuracy)
    } else {
        format!("📍 {}{}", coords, accuracy)
    };
    if let Some(c) = caption {
        format!("{}\n{}", header, c)
    } else {
        header
    }
}

pub fn to_location_context(location: &NormalizedLocation) -> LocationContext {
    let resolved = resolve_location(location);
    LocationContext {
        location_lat: resolved.latitude,
        location_lon: resolved.longitude,
        location_accuracy: resolved.accuracy,
        location_name: resolved.name,
        location_address: resolved.address,
        location_source: resolved.source,
        location_is_live: resolved.is_live,
    }
}

pub struct LocationContext {
    pub location_lat: f64,
    pub location_lon: f64,
    pub location_accuracy: Option<f64>,
    pub location_name: Option<String>,
    pub location_address: Option<String>,
    pub location_source: LocationSource,
    pub location_is_live: bool,
}
