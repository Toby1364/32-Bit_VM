
.print
ldi by 0x0a
lbr cr 0x00 ax

je .end_of_print
cmp .print_new_line

mov cr cx
call .print_char

inc ax
jmp .print

.print_new_line
call .new_line

inc ax
jmp .print

.end_of_print
pop pc


.new_line
lod dx 0x02_0000
ldi dy 70

div dr
inc dr

mov dr dx
mul dr

mov dr cr
ldi by 0x604
cmp .clear

mov dr dx
sto dx 0x02_0000

pop pc


.print_char
lod dx 0x02_0000
mov dx bx
ldi by 0x7f_fa00
add br

ldi cr 0x05ff
mov dx by

cmp .clear

sbr cx 0x00 br
inc dx
sto dx 0x02_0000

pop pc


.clear
ldi cr 0x7f_fa00
ldi by 0x7f_ffff

ldi bx 0

.clear_loop
str bx 0x00 cr

inc cr
inc cr
inc cr
inc cr

jl .clear_loop

ldi dx 0
sto dx 0x02_0000

pop pc
