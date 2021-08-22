use crate::global_config::*;
use crate::parsers::*;
use crate::session::Session;
use crate::Result;
use async_trait::async_trait;
use reqwest::header::{COOKIE, USER_AGENT};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OwnClient {
    pub(crate) user: String,
    pub(crate) session: Session,
}

#[async_trait]
pub trait Environment {
    async fn get_major_list(&self, entrance_year: SchoolYear) -> Result<Vec<Major>>;

    async fn get_class_list(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Class>>;

    async fn get_suggested_course_list(
        &self,
        school_year: SchoolYear,
        semester: Semester,
        major_id: &str,
        class_id: &str,
        entrance_year: Option<&str>,
    ) -> Result<Vec<Course>>;
}

#[async_trait]
impl Environment for OwnClient {
    async fn get_major_list(&self, entrance_year: SchoolYear) -> Result<Vec<Major>> {
        let param = [("njdm_id", entrance_year.to_string())];
        let page = self
            .session
            .client
            .get(url::MAJOR_LIST)
            .form(&param)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        parse_major_list_page(&*text)
    }

    async fn get_class_list(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Class>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("queryModel.showCount", 10000.to_string()),
        ];
        let page = self
            .session
            .client
            .post(url::CLASS_LIST)
            .form(&data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        parse_class_list_page(&*text)
    }

    async fn get_suggested_course_list(
        &self,
        school_year: SchoolYear,
        semester: Semester,
        major_id: &str,
        class_id: &str,
        mut entrance_year: Option<&str>,
    ) -> Result<Vec<Course>> {
        let mut year;
        match entrance_year {
            Some(x) => {
                year = x.to_string();
            }
            None => {
                year = "20".to_string();
                let classes = class_id.chars();
                let mut count = 0;
                for text in classes {
                    year += &text.to_string();
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }
        }
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("njdm_id", year),
            ("zyh_id", major_id.to_string()),
            ("bh_id", class_id.to_string()),
            ("tjkbzdm", "1".to_string()),
            ("tjkbzxsdm", "0".to_string()),
        ];
        let page = self
            .session
            .client
            .post(url::SUGGESTED_COURSE)
            .form(&data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        parse_timetable_page(&*text)
    }
}

#[async_trait]
pub trait User {
    async fn get_profile(&self) -> Result<Profile>;

    async fn get_timetable(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Course>>;

    fn group_timetable(course_list: Vec<Course>) -> HashMap<String, Vec<Course>>;

    async fn get_group_timetable(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<HashMap<String, Vec<Course>>>;

    async fn get_score_list(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Score>>;

    fn calculate_gpa(score_list: Vec<Score>) -> Result<f32>;

    async fn get_gpa(&self, school_year: SchoolYear, semester: Semester) -> Result<f32>;
}
#[async_trait]
impl User for OwnClient {
    async fn get_profile(&self) -> Result<Profile> {
        let page = self
            .session
            .client
            .get(url::PROFILE)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        return parse_profile_page(&*text);
    }

    async fn get_timetable(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Course>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
        ];
        let page = self
            .session
            .client
            .post(url::TIME_TABLE)
            .form(&data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        return parse_timetable_page(&*text);
    }

    fn group_timetable(course_list: Vec<Course>) -> HashMap<String, Vec<Course>> {
        let mut result: HashMap<String, Vec<Course>> = HashMap::new();
        for course in course_list {
            let course_name = course.course_name.clone();
            if result.contains_key(&course_name) {
                let mut v = result.remove(&course_name).unwrap();
                v.push(course.clone());
                result.insert(course_name, v);
            } else {
                result.insert(course_name, vec![course]);
            }
        }
        result
    }

    async fn get_group_timetable(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<HashMap<String, Vec<Course>>> {
        let time_table = self.get_timetable(school_year, semester).await?;
        return Ok(OwnClient::group_timetable(time_table));
    }

    async fn get_score_list(
        &self,
        school_year: SchoolYear,
        semester: Semester,
    ) -> Result<Vec<Score>> {
        let data = [
            ("xnm", school_year.to_string()),
            ("xqm", semester.to_raw().to_string()),
            ("queryModel.showCount", "5000".to_string()),
        ];
        let page = self
            .session
            .client
            .post(url::SCORE_LIST)
            .form(&data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        let text = page.text().await?;
        return parse_score_list_page(&*text);
    }

    fn calculate_gpa(score_list: Vec<Score>) -> Result<f32> {
        Ok(calculate_gpa(score_list))
    }

    async fn get_gpa(&self, school_year: SchoolYear, semester: Semester) -> Result<f32> {
        let score_list = self.get_score_list(school_year, semester).await?;
        return OwnClient::calculate_gpa(score_list);
    }
}
