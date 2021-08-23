mod classes;
mod score;
mod select_course;
mod timetable;
mod user_profile;

pub use classes::{parse_class_list_page, parse_major_list_page, Class, Major};
use reqwest::Error as ReError;
pub use score::{calculate_gpa, parse_score_list_page, Score};
pub use select_course::{parse_available_course_page, SelectCourse};
pub use timetable::{parse_timetable_page, Course};
pub use user_profile::{parse_profile_page, Profile};
use serde_json::Value;

#[derive(Clone)]
pub enum SchoolYear {
    AllYear,
    SomeYear(i32),
}

impl SchoolYear {
    fn new(year: i32) -> Self {
        Self::SomeYear(year)
    }
}

impl ToString for SchoolYear {
    fn to_string(&self) -> String {
        match self {
            SchoolYear::SomeYear(year) => year.to_string(),
            SchoolYear::AllYear => String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Semester {
    All = 0,
    FirstTerm = 1,
    SecondTerm = 2,
    MidTerm = 3,
}

impl Semester {
    pub(crate) fn to_raw(&self) -> &str {
        return match self {
            Semester::All => "",
            Semester::FirstTerm => "3",
            Semester::SecondTerm => "12",
            Semester::MidTerm => "16",
        };
    }

    fn from_raw(raw: &str) -> Result<Semester, ParserError> {
        return match raw {
            "" => Ok(Semester::All),
            "3" => Ok(Semester::FirstTerm),
            "12" => Ok(Semester::SecondTerm),
            "16" => Ok(Semester::MidTerm),
            _ => Err(ParserError::SemesterError),
        };
    }
}

pub fn get_str(x: Option<&Value>) -> String {
    String::from(x.map(|m| m.as_str().unwrap()).unwrap_or_default())
}

pub fn get_f32(x: Option<&Value>) -> f32 {
    get_str(x).parse().unwrap()
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Profile element is wrong!!")]
    MissingField,
    #[error("Invalid semester valid given.")]
    SemesterError,
    #[error("Other Error {}", 0)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

#[macro_export]
macro_rules! convert_inner_errors {
    ($src_err_type: ident) => {
        impl From<$src_err_type> for ParserError {
            fn from(sub_err: $src_err_type) -> Self {
                return Self::Other(Box::from(sub_err));
            }
        }
    };
}

convert_inner_errors!(ReError);
