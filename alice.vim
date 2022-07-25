" Vim syntax file
" Language: alice
" Maintainer: Malte Dostal

if exists("b:current_syntax")
    finish
endif

set nospell

syn keyword alice_statement print println swap clear dup drop over rot
syn keyword alice_statement let fun
syn keyword alice_statement if else

syn match alice_ident '.*:(:?.*)'

syn match alice_num '\v\c<\d%(\d|_*\d)*L=>'
syn match alice_float '\v\c<\d%(\d|_*\d)*%(E[+-]=\d%(\d|_*\d)*[FD]=|[FD])>'
syn match alice_float '\v\c<\d%(\d|_*\d)*\.%(\d%(\d|_*\d)*)=%(E[+-]=\d%(\d|_*\d)*)=[FD]='
syn match alice_float '\v\c\.\d%(\d|_*\d)*%(E[+-]=\d%(\d|_*\d)*)=[FD]='
syn region alice_string start='"' end='"' skip="\\\""

syn region alice_comment start=/#/ end=/$/

syn keyword alice_type int float string bool any
syn keyword alice_const true false

hi def link alice_statement     Keyword
hi def link alice_string        String
hi def link alice_num           Number
hi def link alice_float         Number
hi def link alice_comment       Comment
hi def link alice_ident         Identifier
hi def link alice_type          Type
hi def link alice_const         Constant
