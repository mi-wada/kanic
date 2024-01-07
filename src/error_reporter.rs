use std::{fmt::format, process::exit};

pub fn report_error(user_input: &str, error_location: usize, message: &str) {
    eprintln!("{}", error_message(user_input, error_location, message));
    exit(1);
}

fn error_message(user_input: &str, error_location: usize, message: &str) -> String {
    let (error_line, cumulative_bytes_count) = {
        let mut cumulative_bytes_count = 0;
        let mut res = "";
        for line in user_input.lines() {
            // + 1 is for \n.
            cumulative_bytes_count += line.as_bytes().len() + 1;
            if error_location <= cumulative_bytes_count {
                res = line;
                cumulative_bytes_count -= line.as_bytes().len() + 1;
                break;
            }
            res = line;
        }
        (res, cumulative_bytes_count)
    };

    let error_column_number = error_location - cumulative_bytes_count;

    format!(
        "{}
{}^ {}
",
        error_line,
        " ".repeat(error_column_number),
        message
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message_single_line() {
        let actual = error_message("1 + 2 + hoge", 8, "Invalid token");

        assert_eq!(
            actual,
            "\
1 + 2 + hoge
        ^ Invalid token
"
        );
    }

    #[test]
    fn test_error_message_multi_lines() {
        let actual = error_message(
            "1 + 2 + 2
1 + 3 + hoge",
            18,
            "Invalid token",
        );

        assert_eq!(
            actual,
            "\
1 + 3 + hoge
        ^ Invalid token
"
        );
    }
}
