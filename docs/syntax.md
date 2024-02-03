# Syntax

Syntax of the C language for which this repository is compiled, written in [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

```ebnf
expr    = mul ("+" mul | "-" mul)*
mul     = unary ("*" unary | "/" unary)*
unary   = ("+" | "-")? primary
primary = num | "(" expr ")"
num     = digit digit*
digit   = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```
