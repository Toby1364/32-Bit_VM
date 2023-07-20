ldi cx 1
ubs cx 1
ldi cx 100
sto cx 0x7F_A004
sto cx 0x7F_A006
ldi cx 255
ubs cx 0x7F_A009
ubs cx 0x7F_A00B

ldi dr 1

.loop
lod dy 2
mov dy by

sto cx 0x7F_A000

ldi cy 2
div cr
mov cr dy

sto dy 0x7F_A002

mov dr cy
add cr
mov cr cx

ldi cr 25000

.timer
dec cr
jne .timer

mov cx cr
jle .loop

rnd dx
shr dx 13
inc dr

ldi cx 0

jmp .loop
