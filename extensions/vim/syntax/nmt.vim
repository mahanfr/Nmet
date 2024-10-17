if exists('b:current_syntax') | finish | endif

syntax match   nmetType         /@\[.\+\]\|@[_a-zA-Z]\+/
syntax match   nmetNumbers      transparent "\<\d\|\.\d" contains=nmetNumber,nmetFloat,nmetOctal,nmetBin
syntax match   nmetNumber       contained "\d\+\>"
syntax match   nmetOctal		contained "0x[a-fA-F0-9]\+\>"
syntax match   nmetBin	     	contained "0b[0-1]\+\>"
syntax match   nmetFloat	    contained "\d\+\.\d*\%(e[-+]\=\d\+\)\=[fl]\="
syntax match   nmetFloat		contained "\.\d\+\%(e[-+]\=\d\+\)\=[fl]\=\>"
syntax match   nmetFloat		contained "\d\+e[-+]\=\d\+[fl]\=\>"
syntax match   nmetComment      "[~].\+$"
syntax match   nmetString       /".*"/
syntax match   nmetChar         /'.'/
syntax match   nmetOperator     "+\|-\|*\|/\|[=][=]\|[&]\|[|]\|[>]\|[<]\|[<=]\|[>=]\|[!]"
syntax keyword nmetStatement    asm break return continue
syntax keyword nmetFunction     func struct
syntax keyword nmetLabel        defer
syntax keyword nmetConditional  if else
syntax keyword nmetLoops        while for
syntax keyword nmetKeyword      to
syntax keyword nmetMacro        print ffi
syntax keyword nmetImport       import
syntax keyword nmetStorageClass var static
syntax keyword nmetBoolean      true false

highlight link nmetStatement Statement
highlight link nmetType Type
highlight link nmetLabel Label
highlight link nmetFunction Function
highlight link nmetStorageClass StorageClass
highlight link nmetLoops Repeat
highlight link nmetImport Include
highlight link nmetMacro Macro
highlight link nmetBoolean Boolean
highlight link nmetString String
highlight link nmetChar Character
highlight link nmetConditional Conditional
highlight link nmetKeyword Keyword
highlight link nmetOperator Operator
highlight link nmetComment Comment

highlight link nmetNumber Number
highlight link nmetNumbers Number
highlight link nmetOctal Number
highlight link nmetBin Number
highlight link nmetFloat Number
