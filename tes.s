mov rva 2
mov rvb 12
mov rvc 63
mov rvd 12
mov irg 746
draw rva rvb 6
draw rvc rvd 6
mov rve 0
call .label1
mov rv6 3
mov rv8 2
mov rv0 96
mov rdt rv0
mov rv0 rdt
:label2
se rv0 0
jump .label2
getrand rv7 23
add rv7 8
mov rv9 255
mov irg 752
draw rv6 rv7 1
mov irg 746
:label13
draw rva rvb 6
draw rvc rvd 6
:label5
mov rv0 1
sknp rv0
add rvb 254
mov rv0 4
sknp rv0
add rvb 2
mov rv0 31
and rvb rv0
draw rva rvb 6
mov rv0 12
sknp rv0
add rvd 254
mov rv0 13
sknp rv0
add rvd 2
mov rv0 31
and rvd rv0
draw rvc rvd 6
mov irg 752
draw rv6 rv7 1
add rv6 rv8
add rv7 rv9
mov rv0 63
and rv6 rv0
mov rv1 31
and rv7 rv1
sne rv6 2
jump .label3
sne rv6 63
jump .label4
sne rv7 31
mov rv9 255
sne rv7 0
mov rv9 1
draw rv6 rv7 1
jump .label5
mov rv8 2
:label3
mov rv3 1
mov rv0 rv7
sub rv0 rvb
:label15
jump .label6
mov rv8 254
:label4
mov rv3 10
mov rv0 rv7
sub rv0 rvd
se rvf 1
:label6
jump .label7
mov rv1 2
sub rv0 rv1
se rvf 1
jump .label8
sub rv0 rv1
se rvf 1
jump .label9
sub rv0 rv1
se rvf 1
jump .label10
mov rv0 32
:label7
mov rst rv0
call .label11
add rve rv3
call .label12
mov rv6 62
se rv3 1
mov rv6 3
mov rv8 254
se rv3 1
mov rv8 2
jump .label13
add rv9 255
:label8
sne rv9 254
mov rv9 255
jump .label14
add rv9 1
sne rv9 2
:label10
mov rv9 1
mov rv0 4
:label9
mov rst rv0
add rv6 1
:label14
sne rv6 64
add rv6 254
jump .label15
:label1
:label11
:label12
mov irg 754
bcd rve
store rv2
ldfadr rv1
mov rv4 20
mov rv5 0
draw rv4 rv5 5
add rv4 21
ldfadr rv2
draw rv4 rv5 5
ret
mov rv0 rv8
mov rv0 rv8
mov rv0 rv8
mov rv0 rv0

