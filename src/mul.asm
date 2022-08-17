        LDM     4
        XCH     r0
        LDM     2
        CMA
        XCH     r1
        LDM     0
        JUN     TEST 
LOOP:   ADD     r0

TEST:   ISZ     r1, LOOP
DONE:   JUN     DONE