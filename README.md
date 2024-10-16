# While-Lang

## Statements

Syntax should be like the one discussed [in class](https://hpi.de/friedrich/teaching/units/while-language.html).
The only addition is the **print-statement** in the last line.
You can use `?VAR` to print a variable, where `VAR` is the variable in question.

```while
x0 := x1 + 0;

x2 := x2 + 1;
x2 := x2 + 1;

WHILE x2 != 0 DO
    x0 := x0 + 1;
    x2 := x2 - 1;
END;

?x0;
```

## Execution

- Clone the project
- Execute: `cargo r <filename>`
