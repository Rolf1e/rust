pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return phrase.to_string();
    }
    let mut response = String::new();

    println!("{:?}", &phrase);

    phrase
        .split(|c:char| c.is_whitespace() || c == '_' || c == '-')
        .flat_map(|word| split_condition(word))
        .for_each(|s| response.push_str(&s[..1].to_uppercase()));

    println!("Response :{:?}", &response);
    response
}


fn split_condition(word: &str) -> Vec<String> {
    let mut words : Vec<String> = Vec::new();
    let mut begining : usize = 0;
    let chars: Vec<char> = word.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        let next :usize = i+1;
        if i == word.len()-1 || c.is_lowercase() && chars[next].is_uppercase() {
            words.push(word[begining .. next].to_string());
            begining = next;
        }
    }
    println!("words => {:?}", words);
    words
}
