#!/bin/bash
mkdir -p ~/.vim/syntax
mkdir -p ~/.vim/ftdetect
cp alice.vim ~/.vim/syntax/alice.vim
echo "au BufRead,BufNewFile *.alice set filetype=alice" > ~/.vim/ftdetect/alice.vim 
