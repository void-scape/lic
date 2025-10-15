/// Generate MIT license as seen at: https://en.wikipedia.org/wiki/MIT_License
pub fn generate_mit(year: usize, holders: &str) -> String {
    let mit = include_str!("mit");
    mit.replace("<year>", &year.to_string())
        .replace("<copyright holders>", holders)
}

/// Generate Apache license as seen at: https://www.apache.org/licenses/LICENSE-2.0.txt
pub fn generate_apache(year: usize, holders: &str) -> String {
    let apache = include_str!("apache");
    apache
        .replace("[yyyy]", &year.to_string())
        .replace("[name of copyright owner]", holders)
}
