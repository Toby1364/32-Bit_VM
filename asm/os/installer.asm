
.install
ldi ax 137
sbd ax 0x000_0000

ldi ax 0x000_0010
ldi ay 0x00_ffff

ldi by 0x01_ffff

.loop
lor ar 0x0 ay
sdr ar 0x0 ax

mov ay bx
xor br
mov br cr

inc ax
inc ax
inc ax
inc ax

inc ay
inc ay
inc ay
inc ay

jne .loop

ptrm 0x0 0xff
