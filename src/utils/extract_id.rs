pub fn extract_video_id(url: &str) -> Option<String> {
    if let Some(pos) = url.find("v=") {
        return url[pos + 2..]
            .split('&')
            .next()
            .map(|s| s.to_string());
    } 
    else if let Some(pos) = url.find("youtu.be/") {
        return url[pos + 9..]
            .split(['?', '#'])
            .next()
            .map(|s| s.to_string());
    }
    else if let Some(pos) = url.find("/shorts/") {
        return url[pos + 8..]
            .split(['?', '#'])
            .next()
            .map(|s| s.to_string());
    }
    None
}