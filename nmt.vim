syntax match Types /@\[.\+\]\|@[_a-zA-Z]\+/
syntax match Numbers /[0-9]\+/
syntax match Comments /[~].\+/
syntax match StringLiterals /".\+"/
syntax match DeclearKeywords /var\|return\|func/
syntax match ConditionalKeywords /if\|else\|while\|include/
syntax match Semicolon /[;]/
syntax match Print /print/

highlight Types guifg=#7dcfff
highlight Numbers guifg=#d08770
highlight Semicolon guifg=#b4f9f8
highlight Comments guifg=#565f89
highlight Print guifg=#0db9d7
highlight StringLiterals guifg=#9ece6a
highlight DeclearKeywords guifg=#9d7cd8
highlight ConditionalKeywords guifg=#e0af68

