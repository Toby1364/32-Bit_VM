
.boot_from_cd
call .desktop::init

jmp .halt

.boot_from_drive
call .desktop::init

.halt
jmp .halt
