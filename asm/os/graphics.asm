
.draw_mouse
lod cx 7
sto cx 0x7f_0000
sto cx 0x7f_000d

sto cx 0x7f_0011
sto cx 0x7f_001a

mov cx cr
ldi cx 15
add cx
mov cr cx

sto cx 0x7f_0004
sto cx 0x7f_001e

lod cx 9
sto cx 0x7f_0002
sto cx 0x7f_000f

mov cx cr
ldi cy 20
add cy
mov cr cx

sto cx 0x7f_0006
sto cx 0x7f_0020

ldi cy 7
add cy
mov cr cx

sto cx 0x7f_0013
sto cx 0x7f_001c

ldi cx 2
ubs cx 0x7f_0008
ubs cx 0x7f_0015
ubs cx 0x7f_0022

ldi cx 0
ubs cx 0x7f_0009
ubs cx 0x7f_000a
ubs cx 0x7f_000b

ubs cx 0x7f_0016
ubs cx 0x7f_0017
ubs cx 0x7f_0018

ubs cx 0x7f_0023
ubs cx 0x7f_0024
ubs cx 0x7f_0025

ldi cx 255
ubs cx 0x7f_000c
ubs cx 0x7f_0019
ubs cx 0x7f_0026

pop pc


.draw_background
ldi cx 200
ubs cx 0x7f_a009
ubs cx 0x7f_a00a

ldi cx 180
ubs cx 0x7f_a015
ubs cx 0x7f_a016

ldi cx 160
ubs cx 0x7f_a021
ubs cx 0x7f_a022

ldi cx 140
ubs cx 0x7f_a02d
ubs cx 0x7f_a02e

ldi cx 120
ubs cx 0x7f_a039
ubs cx 0x7f_a03a

ldi cx 100
ubs cx 0x7f_a045
ubs cx 0x7f_a046

ldi cx 80
ubs cx 0x7f_a051
ubs cx 0x7f_a052

ldi cx 60
ubs cx 0x7f_a05d
ubs cx 0x7f_a05e

ldi cx 40
ubs cx 0x7f_a069
ubs cx 0x7f_a06a

ldi cx 20
ubs cx 0x7f_a075
ubs cx 0x7f_a076

ldi cx 255
ubs cx 0x7f_a00b
ubs cx 0x7f_a017
ubs cx 0x7f_a023
ubs cx 0x7f_a02f
ubs cx 0x7f_a03b
ubs cx 0x7f_a047
ubs cx 0x7f_a053
ubs cx 0x7f_a05f
ubs cx 0x7f_a06b
ubs cx 0x7f_a077

ldi cx 0
ubs cx 0x7f_a008
ubs cx 0x7f_a014
ubs cx 0x7f_a020
ubs cx 0x7f_a02c
ubs cx 0x7f_a038
ubs cx 0x7f_a044
ubs cx 0x7f_a050
ubs cx 0x7f_a05c
ubs cx 0x7f_a068
ubs cx 0x7f_a074


ldi cx 0
sto cx 0x7f_a000
sto cx 0x7f_a002

sto cx 0x7f_a00c
sto cx 0x7f_a018
sto cx 0x7f_a024
sto cx 0x7f_a030
sto cx 0x7f_a03c
sto cx 0x7f_a048
sto cx 0x7f_a054
sto cx 0x7f_a060
sto cx 0x7f_a06c

lod cx 2
sto cx 0x7f_a004
sto cx 0x7f_a010
sto cx 0x7f_a01c
sto cx 0x7f_a028
sto cx 0x7f_a034
sto cx 0x7f_a040
sto cx 0x7f_a04c
sto cx 0x7f_a058
sto cx 0x7f_a064
sto cx 0x7f_a070

lod cx 4
sto cx 0x7f_a006
sto cx 0x7f_a012
sto cx 0x7f_a01e
sto cx 0x7f_a02a
sto cx 0x7f_a036
sto cx 0x7f_a042
sto cx 0x7f_a04e
sto cx 0x7f_a05a
sto cx 0x7f_a066
sto cx 0x7f_a072

ldi cy 10
div cr
mov cr cy

mov cr cx
sto cx 0x7f_a00e

add cy
mov cr cx
sto cx 0x7f_a01a

add cy
mov cr cx
sto cx 0x7f_a026

add cy
mov cr cx
sto cx 0x7f_a032

add cy
mov cr cx
sto cx 0x7f_a03e

add cy
mov cr cx
sto cx 0x7f_a04a

add cy
mov cr cx
sto cx 0x7f_a056

add cy
mov cr cx
sto cx 0x7f_a062

add cy
mov cr cx
sto cx 0x7f_a06e


pop pc
