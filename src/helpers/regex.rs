use regex::Regex;


pub fn get_id_from_url(url: &str) -> Result<i64, String> {
    let pattern = Regex::new(r"/(\d+)/").unwrap();
    if let Some(captures) = pattern.captures(url) {
        if let Some(id) = captures.get(1) {
            if let Ok(id_val) = id.as_str().parse::<i64>() {
                return Ok(id_val);
            }
        }
    }
    
    return Err(String::from("No capture found"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_regex() {
        let id = get_id_from_url("https://myanimelist.net/anime/12189/");
        assert_eq!(id.unwrap(), 12189);
    }
    
    #[test]
    fn test_bad_regex(){
        let id = get_id_from_url("https://myanimelist.net/anime/38101/5-toubun_no_Hanayome");
        assert_eq!(id.
            unwrap(), 38101);
    }

    #[test]
    fn test_no_match(){
        let id  = get_id_from_url("https://myanimelist.net/anime");
        assert!(id.is_err());
        assert_eq!(id.unwrap_err(), "No capture found");
    }

}