use crate::parsers::ParserError;
use crate::Result;
use serde_json::Value;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


static ELEMENTS_CLASS: [(&str, &str); 5] = [
    ("grade", "njmc"),      // 年级
    ("college", "jgmc"),    // 学院
    ("major_name", "zymc"), // 专业名称
    ("major_id", "zyh_id"), // 专业代码
    ("class_id", "bh"),     // 班级
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Major {
    #[serde(skip_deserializing)]
    entrance_year: i32,
    #[serde(skip_serializing, rename(deserialize = "njdm"))]
    /// 入学年份
    _entrance_year: String,
    #[serde(rename(deserialize = "zyh"))]
    /// 专业代码
    id: String,
    #[serde(rename(deserialize = "zymc"))]
    /// 专业名称
    name: String,
    #[serde(rename(deserialize = "zyh_id"))]
    /// 专业内部标识
    inner_id: String,
    #[serde(rename(deserialize = "zyfx_id"))]
    /// 专业方向内部表示
    direction_id: String,
    #[serde(rename(deserialize = "zyfxmc"))]
    /// 专业方向
    direction: String,
}

#[derive(Clone)]
pub struct Class {
    grade: String,
    college: String,
    major_name: String,
    major_id: String,
    class_id: String,
}

pub fn parse_major_list_page(page: &str) -> Result<Vec<Major>> {
    let json_page: Value = serde_json::from_str(page)?;

    if let Some(major_list) = json_page.as_array() {
        let result = major_list
            .iter()
            .map(|v| {
                let mut x = serde_json::from_value::<Major>(v.clone()).unwrap();
                x.entrance_year = x._entrance_year.parse().unwrap();
                x
            })
            .collect();
        return Ok(result);
    }
    Ok(vec![])
}

#[test]
fn test_parse_major_list_page() {
    let page =
    r#"
[{
	"jgpxzd": "1",
	"listnav": "false",
	"localeKey": "zh_CN",
	"njdm": "2018",
	"njdm_id": "2018",
	"njmc": "2018",
	"pageable": true,
	"queryModel": {
		"currentPage": 1,
		"currentResult": 0,
		"entityOrField": false,
		"limit": 15,
		"offset": 0,
		"pageNo": 0,
		"pageSize": 15,
		"showCount": 10,
		"sorts": [],
		"totalCount": 0,
		"totalPage": 0,
		"totalResult": 0
	},
	"rangeable": true,
	"totalResult": "0",
	"userModel": {
		"monitor": false,
		"roleCount": 0,
		"roleKeys": "",
		"roleValues": "",
		"status": 0,
		"usable": false
	},
	"zyfx_id": "2018Y240101",
	"zyfxdm": "2018Y240101",
	"zyfxmc": "本科预科班",
	"zyh": "Y2401",
	"zyh_id": "Y2401",
	"zymc": "本科预科班"
}, {
	"jgpxzd": "1",
	"listnav": "false",
	"localeKey": "zh_CN",
	"njdm": "2018",
	"njdm_id": "2018",
	"njmc": "2018",
	"pageable": true,
	"queryModel": {
		"currentPage": 1,
		"currentResult": 0,
		"entityOrField": false,
		"limit": 15,
		"offset": 0,
		"pageNo": 0,
		"pageSize": 15,
		"showCount": 10,
		"sorts": [],
		"totalCount": 0,
		"totalPage": 0,
		"totalResult": 0
	},
	"rangeable": true,
	"totalResult": "0",
	"userModel": {
		"monitor": false,
		"roleCount": 0,
		"roleKeys": "",
		"roleValues": "",
		"status": 0,
		"usable": false
	},
	"zyfx_id": "2018B210000",
	"zyfxdm": "2018B210000",
	"zyfxmc": "人文学院大类(公共管理类、社会学类)",
	"zyh": "B2100",
	"zyh_id": "B2100",
	"zymc": "人文学院大类"
}]"#;

    let parsed_major_list = parse_major_list_page(page);
    println!("{:#?}", parsed_major_list);
}
