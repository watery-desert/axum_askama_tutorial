pub fn extract_error(input: &str,) {
    let lines = input.lines();

    lines.for_each(|line| {
        // email: Invalid email
        if let Some((first, second)) = line.split_once(": ") {

        }
    });
}
