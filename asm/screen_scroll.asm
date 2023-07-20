ldi ax 1
ldi by 128
ldi cy 128

.start
ldi ar 0

.loop
add ax
mov ar cr

cmp .start
mov cr dr

ldi bx 0xFA00
ldi by 0xFFFF

jmp .print
.arfix
ldi ar 0

.print
mov ar cr
sub cy cr
je .arfix

sbr ar 0x7F bx
sub by bx
mov br cr
inc bx
inc ar
jne .print

ldi by 128
mov dr cr

wit
jmp .loop
