use shinobi::helpers::regex;

fn main() {
    let res = regex::get_id_from_url("https://myanimelist.net/anime/12189/");
    println!("{:#?}",res);
    println!("Hello, world!");
}
