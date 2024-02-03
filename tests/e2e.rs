#[test]
fn test_ok_single_int() {
    let res = std::process::Command::new("bin/run_arg")
        .arg("42")
        .output()
        .unwrap();

    assert_eq!(res.status.code(), Some(42));
}

#[test]
fn test_ok_simple_formula() {
    let res = std::process::Command::new("bin/run_arg")
        .arg("5+20-4")
        .output()
        .unwrap();

    assert_eq!(res.status.code(), Some(21));
}

#[test]
fn test_ok_complex_formula() {
    let res = std::process::Command::new("bin/run_arg")
        .arg("(+3 + -2) * 3 - 5 / 5")
        .output()
        .unwrap();

    assert_eq!(res.status.code(), Some(2));
}

#[test]
fn test_ok_cmp_true() {
    let res = std::process::Command::new("bin/run_arg")
        .arg("(1 < 2 * 3 + 4) == (5 * 6 - 7 >= 8)")
        .output()
        .unwrap();

    assert_eq!(res.status.code(), Some(1));
}

#[test]
fn test_ok_cmp_false() {
    let res = std::process::Command::new("bin/run_arg")
        .arg("(1 < 2 * 3 + 4) == (5 * 6 - 7 <= 8)")
        .output()
        .unwrap();

    assert_eq!(res.status.code(), Some(0));
}

#[test]
fn test_ng_only_symbol() {
    let res = std::process::Command::new("target/debug/kanic")
        .arg("10 + 2 + moge")
        .output()
        .unwrap();

    assert_eq!(
        String::from_utf8(res.stderr).unwrap(),
        "\
10 + 2 + moge
         ^ Invalid token

"
    );
}
