use std::collections::HashMap;

pub fn parse_body(body: &str) -> HashMap<String, String> {
    let mut parsed_data = HashMap::new();
    if body.contains("=") && body.contains("&") {
        let props = body.split("&");
        for prop in props {
            let mut prop_parts = prop.split("=");
            if let (Some(prop_key), Some(prop_value)) = (prop_parts.next(), prop_parts.next()) {
                let key = urlencoding::decode(prop_key).unwrap_or_else(|_| prop_key.into());
                let value = urlencoding::decode(prop_value).unwrap_or_else(|_| prop_value.into());
                parsed_data.insert(key.to_string(), value.to_string());
            }
        }
    } else if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(object) = json_value.as_object() {
            for (key, value) in object {
                parsed_data.insert(key.clone(), value.to_string());
            }
        }
    } else {
        parsed_data.insert("body".to_string(), body.to_string());
    }

    parsed_data
}


pub fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut parsed_data = HashMap::new();
    if !query.is_empty() {
        for param in query.split('&') {
            let mut param_parts = param.split('=');
            if let (Some(key), Some(value)) = (param_parts.next(), param_parts.next()) {
                let decoded_key = urlencoding::decode(key).unwrap_or_else(|_| key.into());
                let decoded_value = urlencoding::decode(value).unwrap_or_else(|_| value.into());
                parsed_data.insert(decoded_key.to_string(), decoded_value.to_string());
            }
        }
    }
    parsed_data
}


pub fn parse_dynamic_url_params(route: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let route_segments: Vec<&str> = route.split('/').collect();

    for (i, segment) in route_segments.iter().enumerate() {
        if segment.starts_with(':') && i < route_segments.len() - 1 {
            let param_name = segment.trim_start_matches(':');
            let param_value = route_segments[i + 1];
            params.insert(param_name.to_string(), param_value.to_string());
        }
    }
    
    params
}
