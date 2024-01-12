use assert_cmd::Command;

#[test]
fn test_ok_single_int() {
    let mut cmd = Command::cargo_bin("kanic").unwrap();
    let assert = cmd.arg("42").assert();

    assert.success().stdout(
        "\
.intel_syntax noprefix
.globl main

main:
        push 42
        pop rax
        ret

",
    );
}

#[test]
fn test_ok_simple_formula() {
    let mut cmd = Command::cargo_bin("kanic").unwrap();
    let assert = cmd.arg("5+20-4").assert();

    assert.success().stdout(
        "\
.intel_syntax noprefix
.globl main

main:
        push 5
        push 20
        pop rdi
        pop rax
        add rax, rdi
        push rax
        push 4
        pop rdi
        pop rax
        sub rax, rdi
        push rax
        pop rax
        ret

",
    );
}

#[test]
fn test_ok_complex_formula() {
    let mut cmd = Command::cargo_bin("kanic").unwrap();
    let assert = cmd.arg("(1 + 2) * 3 - 4 / 5").assert();

    assert.success().stdout(
        "\
.intel_syntax noprefix
.globl main

main:
        push 1
        push 2
        pop rdi
        pop rax
        add rax, rdi
        push rax
        push 3
        pop rdi
        pop rax
        imul rax, rdi
        push rax
        push 4
        push 5
        pop rdi
        pop rax
        cqo
        idiv rdi
        push rax
        pop rdi
        pop rax
        sub rax, rdi
        push rax
        pop rax
        ret

",
    );
}

#[test]
fn test_ng_only_symbol() {
    let mut cmd = Command::cargo_bin("kanic").unwrap();
    let assert = cmd.arg("10 + 2 + moge").assert();

    assert.failure().stderr(
        "\
10 + 2 + moge
         ^ Invalid token

",
    );
}
