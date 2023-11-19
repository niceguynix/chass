mov rv0 1
mov rv1 2

:loop
    ldfadr rv1
    draw rv0 rv0 5
    jump .loop