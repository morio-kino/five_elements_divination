use regex::Regex;

pub struct TextTools {}

impl TextTools {
    pub fn parse_date(date_str: &str) -> Option<(String, String, String)> {
        // yyyy/mm/dd 形式の正規表現: 4桁/2桁/2桁
        match Regex::new(r"^(\d{4})/(\d{2})/(\d{2})$") {
            Ok(re) => {
                if let Some(caps) = re.captures(date_str.trim()) {
                    let year = caps.get(1).map_or("", |m| m.as_str());
                    let month = caps.get(2).map_or("", |m| m.as_str());
                    let day = caps.get(3).map_or("", |m| m.as_str());

                    Some((year.to_owned(), month.to_owned(), day.to_owned()))
                } else {
                    None // 形式が違う
                }
            }
            Err(s) => panic!("yyyy/mm/dd のRegex::newでエラー [{s}]"),
        }
    }

    pub fn parse_time(time_str: &str) -> Option<(String, String, String)> {
        // hh:mm:ss 形式の正規表現: 2桁/2桁/2桁
        match Regex::new(r"^(\d{2}):(\d{2}):(\d{2})$") {
            Ok(re) => {
                if let Some(caps) = re.captures(time_str.trim()) {
                    let hh = caps.get(1).map_or("", |m| m.as_str());
                    let mm = caps.get(2).map_or("", |m| m.as_str());
                    let ss = caps.get(3).map_or("", |m| m.as_str());

                    Some((hh.to_owned(), mm.to_owned(), ss.to_owned()))
                } else {
                    None // 形式が違う
                }
            }
            Err(s) => panic!("hh:mm:ss のRegex::newでエラー [{s}]"),
        }
    }
}
