use crate::parsers::Semester;
use crate::Result;
use serde_json::Value;
use std::str::FromStr;

#[derive(Clone)]
pub struct Score {
    /// 成绩
    score: f32,
    /// 课程
    course: String,
    /// 课程代码
    course_id: String,
    /// 班级
    class_id: String,
    /// 学年
    school_year: String,
    /// 学期
    semester: Semester,
    /// 学分
    credit: f32,
}

pub fn parse_score_list_page(page: &str) -> Result<Vec<Score>> {
    let json_page: Value = serde_json::from_str(page)?;
    let course_list = json_page["items"].clone();
    if let Some(course) = course_list.as_array() {
        let mut result = Vec::new();
        for each_course in course {
            let scores = f32::from_str(&*each_course["cj"].to_string()).unwrap_or(0.0);
            let sem = Semester::from_raw(&*each_course["xqm"].to_string()).unwrap();
            let credits = f32::from_str(&*each_course["xf"].to_string()).unwrap_or(0.0);
            result.push(Score {
                score: scores,
                course: each_course["kcmc"].as_str().unwrap_or("空").to_string(),
                course_id: each_course["kch"].as_str().unwrap_or("空").to_string(),
                class_id: each_course["jxb_id"].as_str().unwrap_or("空").to_string(),
                school_year: each_course["xnmmc"].as_str().unwrap_or("空").to_string(),
                semester: sem,
                credit: credits,
            })
        }
        return Ok(result);
    }
    Ok(vec![])
}

pub fn calculate_gpa(scores: Vec<Score>) -> f32 {
    let mut total_credits = 0.0;
    let mut t = 0.0;
    for s in scores {
        t += s.credit * s.score;
        total_credits += s.credit;
    }
    let result = (t / total_credits / 10.0) - 5.0;
    return result;
}
