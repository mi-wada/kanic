use assert_cmd::Command;

#[test]
fn test_success_single_int() {
    let mut cmd = Command::cargo_bin("kanic").unwrap();
    let assert = cmd.arg("42").assert();

    assert.success().stdout(
        "\
.intel_syntax noprefix
.globl main

main:
        mov rax, 42
        ret

",
    );
}

#[test]
fn test_success_formula() {
    let mut cmd = Command::cargo_bin("kanic").unwrap();
    let assert = cmd.arg("5+20-4").assert();

    assert.success().stdout(
        "\
.intel_syntax noprefix
.globl main

main:
        mov rax, 5
        add rax, 20
        sub rax, 4
        ret

",
    );
}
