# Specs

## Basic

```ebnf
Identifier = (Letter, {Letter, Digit});
Letter = "A" | "B" | "C" | "D" | "E" | "F" | "G"
           | "H" | "I" | "J" | "K" | "L" | "M" | "N"
           | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
           | "V" | "W" | "X" | "Y" | "Z" ;
Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
String = "'", { AllLetters }, "'";
AllLetters = ? All letters ?
Type = "Integer" | "string" | "boolean" | "float" | "double" | Identifier;
```

## File

```ebnf
File = FileHeader, 

    (
        "interface",

        {Imports |  ConstantSection | DefinitionSection | VariableSection}
    )

    (
        "implementation",

        [Imports]

        [ResourceFiles]

        {ConstantSection | VariableSection | }
    )

    ["initialization"]

    ["finalisation"]

    "end.";


FileHeader = "unit", Identifier, ";";
ImportDirectives = "{$IFDEF", Identifier, "}", Identifier, {",", Indefifier}, "{$ENDIF}";
Imports = "uses", ((Identifier, {",", ( Indefifier | ImportDirectives )}) | ImportDirectives), ";";
VisibilityKw = "public" |"protected" | "private";

ConstantSection = "const", 
                    {ConstAssignment}

ConstAssignment = Identifier, "=", (Digit | String | Identifier);
Assignment = Identifier, ":=", (Digit | String | Identifier | Condition);
Condition = 

VariableSection = "var", {Variable};
Variable = Identifier, ":", Type;

ResourceFiles = "{$R" (Letter | Regex), ".", (Letter | Regex)}
```

### Interface section

```ebnf
DefinitionSection = ["type", class] | MethodHead;

Class = Idenifier, "=", ("class" | "interface")
            (VisibilityKw | Variable | MethodHead)
        "end;";

MethodHead = (Functionhead | ProcedureHead), ["override;"];
FunctionHead = "function", Identifier, ["(", {["var"], Variable}, ")"], ":", Type, ";";
ProcedureHead = ("procedure" | "destructor" | "constructor"), Identifier, ["(", {["var"], Variable}, ")"], ";";
```

### Implementation section

```ebnf
Method = MethodHead, 
            [VariableSection],
            Body;

MethodHead = (Functionhead | ProcedureHead);
FunctionHead = "function", (Identifier | (Identifier, ".", Indentifier)), ["(", {["var"], Variable}, ")"], ":", Type, ";";
ProcedureHead = ("procedure" | "destructor" | "constructor"), (Identifier | (Identifier, ".", Indentifier)), ["(", {["var"], Variable}, ")"], ";";

Body = "begin",
        {Assignment | }
        "end;";

If = 
```