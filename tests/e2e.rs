fn assert_exit_code(c_code: &str, expected: i32) {
    let res = std::process::Command::new("bin/run_arg")
        .arg(c_code)
        .output()
        .unwrap();

    assert_eq!(res.status.code(), Some(expected));
}

#[test]
fn test_ok_single_int() {
    assert_exit_code("return 42;", 42);
}

#[test]
fn test_ok_simple_formula() {
    assert_exit_code("return 5+20-4;", 21)
}

#[test]
fn test_ok_complex_formula() {
    assert_exit_code("return (+3 + -2) * 3 - 5 / 5;", 2);
}

#[test]
fn test_ok_cmp_true() {
    assert_exit_code("return (1 < 2 * 3 + 4) == (5 * 6 - 7 >= 8);", 1);
}

#[test]
fn test_ok_cmp_false() {
    assert_exit_code("return (1 < 2 * 3 + 4) == (5 * 6 - 7 <= 8);", 0);
}

#[test]
fn test_local_var() {
    assert_exit_code("a = 3; bar = 10; return 3 * a + bar;", 19);
}

#[test]
fn test_double_return() {
    assert_exit_code("return 10; return 12;", 10);
}

#[test]
fn test_if_eval_if() {
    assert_exit_code("a = 20; b = 10; if (a > b) return a; else return b;", 20);
}

#[test]
fn test_if_eval_else() {
    assert_exit_code("a = 20; b = 10; if (a < b) return a; else return b;", 10);
}

#[test]
fn test_while() {
    assert_exit_code("a = 0; while (a < 10) a = a + 1; return a;", 10);
}

#[test]
fn test_for() {
    assert_exit_code(
        "a = 0; for (b = 0; b < 10; b = b + 1) a = a + 1; return a;",
        10,
    );
}

#[test]
fn test_ng_only_symbol() {
    let res = std::process::Command::new("target/debug/kanic")
        .arg("10 + 2 == == 2")
        .output()
        .unwrap();

    assert_eq!(
        String::from_utf8(res.stderr).unwrap(),
        "\
10 + 2 == == 2
          ^ Invalid token

"
    );
}
