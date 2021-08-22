use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// 学号
    student_no: String,
    /// 姓名
    name: String,
    /// 英文姓名
    name_eng: String,
    /// 性别
    sex: String,
    /// 证件类型
    credential_type: String,
    /// 证件号码
    credential_id: String,
    /// 出生日期
    birth_date: String,
    /// 民族
    ethnicity: String,
    /// 籍贯
    hometown: String,
    /// 入学日期
    enrollment_date: String,
    /// 学生类型
    types: String,
}

static ELEMENTS: [(&str, &str); 11] = [
    ("student_no", "#col_xh > p:nth-child(1)"),
    ("name", "#col_xm > p:nth-child(1)"),
    ("name_eng", "#col_ywxm > p:nth-child(1)"),
    ("sex", "#col_xbm > p:nth-child(1)"),
    ("credential_type", "#col_zjlxm > p:nth-child(1)"),
    ("credential_id", "#col_zjhm > p:nth-child(1)"),
    ("birth_date", "#col_csrq > p:nth-child(1)"),
    ("ethnicity", "#col_mzm > p:nth-child(1)"),
    ("hometown", "#col_jg > p:nth-child(1)"),
    ("enrollment_date", "#col_rxrq > p:nth-child(1)"),
    ("type", "#col_xslxdm > p:nth-child(1)"),
];

pub fn parse_profile_page(text: &str) -> Result<Profile> {
    use scraper::{Html, Selector};
    let pages = Html::parse_document(text);
    let mut element = Vec::new();
    for (field, selector) in ELEMENTS {
        let selectors = Selector::parse(selector).unwrap();
        let value = pages
            .select(&selectors)
            .next()
            .map(|x| x.inner_html())
            .unwrap_or_default();
        element.push(value);
    }
    if element.len() == 11 {
        let result = Profile {
            student_no: String::from(element.get(0).unwrap_or(&String::from(""))),
            name: String::from(element.get(1).unwrap_or(&"".to_string())),
            name_eng: String::from(element.get(2).unwrap_or(&"".to_string())),
            sex: String::from(element.get(3).unwrap_or(&"".to_string())),
            credential_type: String::from(element.get(4).unwrap_or(&"".to_string())),
            credential_id: String::from(element.get(5).unwrap_or(&"".to_string())),
            birth_date: String::from(element.get(6).unwrap_or(&"".to_string())),
            ethnicity: String::from(element.get(7).unwrap_or(&"".to_string())),
            hometown: String::from(element.get(8).unwrap_or(&"".to_string())),
            enrollment_date: String::from(element.get(9).unwrap_or(&"".to_string())),
            types: String::from(element.get(10).unwrap_or(&"".to_string())),
        };
        return Ok(result);
    }
    Err(anyhow::anyhow!(
        "Profile element error!!, see user_profile.rs"
    ))
}
