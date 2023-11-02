extern  crate regex;
use regex::Regex;

pub struct StringHelper;
impl StringHelper{
    pub fn cleanse(text:&str)->Result<String, Box<dyn std::error::Error>> {
        let mut string : String; 

        // Convert any HTML line breaks to newlines
        // <br /> to "/n"
        let re = Regex::new(r"<br\s*/?>").unwrap();
        string = re.replace_all(text, "\n").to_string();

        // Convert non-breaking spaces to spaces
        string = string.replace("\u{00a0}", " ");
        
        // Strip any remaining HTML tags
        // string = html.unescape(string)  // Decode HTML entities

        // Use regex
        let re = Regex::new(r"<[^>]*>").unwrap();
        string = re.replace_all(&string, "").to_string();

        // Remove newlines at the end
        string  = string.trim_end_matches("\n").to_string();

        // Remove whitespace
        string = string.trim().to_string();

        return Ok(string);
    }
}