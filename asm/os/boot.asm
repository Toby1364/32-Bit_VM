
.start
ldi cr 0
jnci .start

call .terminal::clear

ldb cr 0x000_0000
ldi cx 137

sub cx cr

jne .install_prompt
jmp .loader::boot_from_drive


.install_prompt
ast ax 0
call .terminal::print

ldi cy 0x0C

ldi cx 0x3e
ldi dx 0x00
ldi by 0x80

.install_prompt_loop
call .terminal::input_char

cmp .install_prompt_up
dec cr
cmp .install_prompt_down

sub cy cr
je .install_prompt_proceed

jmp .install_prompt_loop

.install_prompt_up
ubs dx 0x7f_fad2
ubs cx 0x7f_fa8c

jmp .install_prompt_loop

.install_prompt_down
ubs dx 0x7f_fa8c
ubs cx 0x7f_fad2

jmp .install_prompt_loop

.install_prompt_proceed
call .terminal::clear
ubl cr 0x7f_fad2

je .loader::boot_from_cd
jmp .installer::install
