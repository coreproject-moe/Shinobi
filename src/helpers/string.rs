extern crate html_escape;
extern crate regex;

use html_escape::decode_html_entities_to_string;
use regex::Regex;
use std::error::Error;

pub struct StringHelper;
impl StringHelper {
    pub fn cleanse(text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut string: String;

        // Convert any HTML line breaks to newlines
        // <br /> to "/n"
        let re = Regex::new(r"<br\s*/?>").unwrap();
        string = re.replace_all(text, "\n").to_string();

        // Strip any remaining HTML tags
        let mut unencoded_string = String::from("");
        decode_html_entities_to_string(string, &mut unencoded_string); // Decode HTML entities
        string = unencoded_string.to_string();

        // Use regex
        let re = Regex::new(r"<[^>]*>").unwrap();
        string = re.replace_all(&string, "").to_string();

        // Remove newlines at the end
        string = string.trim_end_matches("\n").to_string();

        // Remove whitespace
        string = string.trim().to_string();

        // Convert non-breaking spaces to spaces
        string = string.replace("\u{00a0}", " ");

        return Ok(string);
    }
    pub fn add_my_animelist_if_not_there(text: &str) -> Result<String, Box<dyn Error>> {
        if !text.contains("myanimelist.net") {
            Ok("https://myanimelist.net".to_owned() + text);
        } else {
            Ok(text.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_cleanse() {
        assert_eq!(
            StringHelper::cleanse("This is a text with <br/> HTML line breaks.<br>Testing.")
                .unwrap(),
            "This is a text with \n HTML line breaks.\nTesting."
        );

        assert_eq!(
            StringHelper::cleanse("Non-breaking spaces:&nbsp;Test").unwrap(),
            "Non-breaking spaces: Test"
        );

        assert_eq!(
            StringHelper::cleanse("This is <b>bold</b> and <i>italic</i> text.").unwrap(),
            "This is bold and italic text."
        )
    }
}
