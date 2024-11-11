use chrono::Utc;

pub fn previous_page(page: i32) -> i32 {
    page - 1
}

pub fn next_page(page: i32) -> i32 {
    page + 1
}

pub fn todo_date(date: chrono::DateTime<Utc>) -> String {
    date.format("%d %b %Y at %I:%M%P").to_string()
}