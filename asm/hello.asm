.ORIG x3000                        ; load program at this address
LEA R0, STR_HELLO                  ; load pointer to string
PUTS                               ; output string at address in R0
HALT                               ; halt execution
STR_HELLO .STRINGZ "Hello World!"  ; string
.END                               ; end of program