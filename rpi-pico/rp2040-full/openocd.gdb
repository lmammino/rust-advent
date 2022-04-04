target extended-remote :3333

set print asm-demangle on

set backtrace limit 32

# break DefaultHandler
# break HardFault

# break main

# monitor arm semihosting enable

load 

next
next
next
step
# stepi
