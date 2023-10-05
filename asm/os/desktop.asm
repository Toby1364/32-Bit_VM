
.init
ldi ax 1
ubs ax 1

.draw_loop

call .graphics::draw_background
call .graphics::draw_mouse

ldi ax 0x7fff
ldi ar 0x7fff

.app_search_loop
ldbr cr 0 ar

jne .draw_app

add ax

mov ar cr
ldi by 0x0FF_FE00

jl .app_search_loop 


jmp .draw_loop

pop pc


.draw_app
ldi cx 100
sto cx 0x7f_9000
sto cx 0x7f_9002

ldi cx 0
ubs cx 0x7f_9004

lod cx 4
ldi cy 40
div cr
mov cr cx
ubs cx 0x7f_9005

ldi bx 0
sto bx 0x7f_9006

ldi cx 0xff
ubs cx 0x7f_9009

ldi bx 0x02_0000
sto bx 0x7f_900a

mov ar by

ldr bx 0 by
sto bx 0x02_0000

inc by
inc by
inc by
inc by
ldr bx 0 by
sto bx 0x02_0004

inc by
inc by
inc by
inc by
ldr bx 0 by
sto bx 0x02_0008

inc by
inc by
inc by
inc by
ldr bx 0 by
sto bx 0x02_000c

add ax

jmp .app_search_loop
