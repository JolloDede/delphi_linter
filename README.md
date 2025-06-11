# DelphiLinter

## Caviats

Multilinestrings in Delphi have some wierd behavoir. They are not implemented correctly in this Lexer implementation.

### Example
```
'''
cool
'''
```

Should be `cool` but is `cool\n`