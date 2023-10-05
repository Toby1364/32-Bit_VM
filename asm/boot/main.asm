.wait
jnci .wait

mov ptr cr
ldi by 0x03_00ff

cmp .boot_stage_2

ast ax 0
call .terminal::print

ldb cr 0x000_0000

je .boot_from_cd

jmp .boot_from_drive

.halt_loop
jmp .halt_loop


.unbootable_code_on_cd
ast ax 3
call .terminal::print

jmp .halt_loop


.boot_from_drive
ast ax 2
call .terminal::print

ldi ax 0xfffc

.moving_loop
lor ar 0 ax
str ar 3 ax

mov ax cr

dec ax
dec ax
dec ax
dec ax

jne .moving_loop

ptrm 0x0 0x03_00ff

.boot_stage_2
ldi ax 0xfffc

.os_moving_loop
ldi ay 0x10
add ar
ldr br 0 ar

ldi ay 0xff
add ar
str br 0 ar

mov ax cr

dec ax
dec ax
dec ax
dec ax

jne .os_moving_loop
ptrm 0x0 0xff


.boot_from_cd
ast ax 1
call .terminal::print
ast ax 2
call .terminal::print

lcdb cr 0x00_0000
je .unbootable_code_on_cd

ldi ax 0x00_0000
ldi bx 0x00_ffff
ldi by 0x01_ffff

.load_ram
lcdr ay 0x00 ax
str ay 0x00 bx

inc ax
inc ax
inc ax
inc ax

sub by bx
mov br cr

inc bx
inc bx
inc bx
inc bx

jne .load_ram
ptrm 0x00_0000 0x00_ffff
