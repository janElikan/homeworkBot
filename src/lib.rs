use chrono::NaiveDate;

#[allow(dead_code)]
pub struct App {
    current_assignments: Vec<String>,
    schedule: Vec<Vec<Period>>,
    overwrite_schedule: Option<Vec<Period>>,
    admins: Vec<String>,
}

#[allow(dead_code)]
pub struct Period {
    start: NaiveDate,
    end: NaiveDate,
    name: String,
}
