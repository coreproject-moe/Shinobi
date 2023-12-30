extern crate regex;
use regex::Regex;

pub struct RegexHelper;
impl RegexHelper {
    pub fn get_id_from_url(url: &str) -> Result<i64, String> {
        let pattern = Regex::new(r"/(\d+)/").unwrap();
        if let Some(captures) = pattern.captures(url) {
            if let Some(id) = captures.get(1) {
                if let Ok(id_val) = id.as_str().parse::<i64>() {
                    return Ok(id_val);
                } else {
                    return Err("ID is not an integer".to_string());
                }
            } else {
                return Err("No ID found".to_string());
            }
        }

        Err("No capture found".to_string())
    }

    pub fn get_content_between_first_brackets(text: &str) -> Result<String, String> {
        let pattern = Regex::new(r"\((.*?)\)").unwrap();
        if let Some(captures) = pattern.captures(text) {
            if let Some(inner_text) = captures.get(1) {
                return Ok(inner_text.as_str().trim().to_string());
            } else {
                return Err("No Text found".to_string());
            }
        }

        Err("No capture found".to_string())
    }

    pub fn check_if_string_contains_integer(string: &str) -> Result<bool, regex::Error> {
        let pattern = Regex::new(r"\d+").unwrap();
        Ok(pattern.is_match(string))
    }

    pub fn check_if_string_contains_bracket(string: &str) -> Result<bool, regex::Error> {
        let pattern = Regex::new(r"\[\d+\]").unwrap();
        Ok(pattern.is_match(string))
    }

    pub fn replace_br_with_newline(string: &str) -> Result<String, regex::Error> {
        let re = Regex::new(r"<br\s*\/?>").unwrap();

        Ok(re.replace_all(string, "\n").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id_from_url_perfect_regex() {
        let id = RegexHelper::get_id_from_url("https://myanimelist.net/anime/12189/");
        assert_eq!(id.to_owned().unwrap(), 12189);

        let bad_id = RegexHelper::get_id_from_url(
            "https://myanimelist.net/anime/38101/5-toubun_no_Hanayome",
        );
        assert_eq!(bad_id.to_owned().unwrap(), 38101);

        let no_match_id = RegexHelper::get_id_from_url("https://myanimelist.net/anime");
        assert!(no_match_id.is_err());
        assert_eq!(no_match_id.to_owned().unwrap_err(), "No capture found");
    }

    #[test]
    fn test_capture_between_first_brackets() {
        let text = RegexHelper::get_content_between_first_brackets("Sora Amamiya ( 雨宮 天 )");
        assert_ne!(text.to_owned().unwrap(), " 雨宮 天 ");
        assert_eq!(text.to_owned().unwrap(), "雨宮 天");

        let bad_text = RegexHelper::get_content_between_first_brackets("Sora Amamiya");

        assert!(bad_text.to_owned().is_err());
        assert_eq!(bad_text.to_owned().unwrap_err(), "No capture found");
    }

    #[test]
    fn test_check_if_string_contains_integer() {
        let text = RegexHelper::check_if_string_contains_integer("hello 123");
        assert!(text.unwrap());

        let bad_text = RegexHelper::check_if_string_contains_integer("hello");
        assert!(!bad_text.unwrap());
    }

    #[test]
    fn test_check_if_string_contains_bracket() {
        let text = RegexHelper::check_if_string_contains_bracket("Sora [123]");
        assert!(text.unwrap());
    }

    #[test]
    fn test_replace_newline() {
        let text = "<br/>This is a sample text with <br> line breaks.<br />";
        assert_eq!(
            RegexHelper::replace_br_with_newline(text)
                .to_owned()
                .unwrap(),
            "\nThis is a sample text with \n line breaks.\n"
        );
    }
}
