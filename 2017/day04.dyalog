x←' '(≠⊆⊢)¨⊃⎕NGET 'inputs/day04.txt' 1
⎕←+/(∪≡⊢)¨x
⎕←+/(∪≡⊢)¨{⍵[⍋⍵]}¨¨x
