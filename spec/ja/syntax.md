Syntax
======

文字の種類
---
`graphic_character`は任意の表示可能文字

一文字の記号
---
`'('`, `')'`, `'['`, `']'`, `'{'`, `'}'`, `';'`, `','`は1文字でlex

記号
---
```
symbol_char
    <- '!' / '#' / '$' / '%' / '&' / '*' / '+' / '-' / '.' / '/' /
       ':' / '<' / '=' / '>' / '?' /
       '@' /
       '\\' / '^' /
       '|' / '~'

symbol
    <- symbol_char*
```

識別子
---
```
identifier
    <- ([a-z] / [A-Z] / '_') ([a-z] / [A-Z] / [0-9] / '_')*
```

数値
---
```
numerical_value
    <- [0-9] ([0-9] / [a-z] / [A-Z] / '_' / '.')*
```

文字
---
```
character
    <- ''' '\'? graphic_character '''
```

文字列
---
```
string
    <- '"' (('{' expression '}') / ('\'? graphic_character))* '"'
```

primary expression
---
```
primary_expression
    <- identifier /
       numerical_value /
       character /
       string /
       '(' expression ')' /
       symbol primary_expression       
```

expression
---
```
expression
    <- expression symbol expression
```

program
---
```
program
    <- (string ';')*
```
