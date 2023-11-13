:main 
    mov rv0 1
draw rv0 rv0 5

mov rv1 6
mov irg 5
draw rv1 rv0 5

mov irg 10
mov rv1 11
draw rv1 rv0 5

mov irg 15
mov rv1 16
draw rv1 rv0 5

mov irg 20
mov rv1 21
draw rv1 rv0 5

:loop
    jump .loop