pub fn extract_error<F>(input: &str, mut f: F)
where
    F: FnMut(String, String),
{
    let lines = input.lines();

    lines.for_each(|line| {
        // email: invalid email
        if let Some((first, second)) = line.split_once(": ") {
            f(first.to_string(), second.to_string());
        };
    });
}
