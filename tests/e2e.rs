use assert_cmd::Command;

#[test]
fn test_success() {
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
