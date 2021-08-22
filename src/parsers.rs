mod classes;
mod score;
mod select_course;
mod timetable;
mod user_profile;

pub use classes::{parse_class_list_page, parse_major_list_page, Class, Major};
pub use score::{calculate_gpa, parse_score_list_page, Score};
pub use select_course::{parse_available_course_page, SelectCourse};
pub use timetable::{parse_timetable_page, Course};
pub use user_profile::{parse_profile_page, Profile};

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

#[derive(Clone)]
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

    fn from_raw(raw: &str) -> anyhow::Result<Semester> {
        return match raw {
            "" => Ok(Semester::All),
            "3" => Ok(Semester::FirstTerm),
            "12" => Ok(Semester::SecondTerm),
            "16" => Ok(Semester::MidTerm),
            _ => Err(anyhow::anyhow!("Invalid semester valid given.")),
        };
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Missing field {} when parsing {}.", 0, 1)]
    MissingField(String, String),
}
