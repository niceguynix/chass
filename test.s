mov rv0 1
mov rv1 1
mov rv2 5
mov rv3 0
:loop
    se rv3 12
    jump .continue
    jump .end
:continue
    draw rv0 rv1 5
    add rv0 5
    add irg rv2
    add rv3 1
    jump .loop
:end
    jump .end