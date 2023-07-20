
func hold() {
    asm {
        jmp .hold
    }
}

func print(x: array[u8? 70]) {
    reg ax = x[0];
}

func main() {
    let chars: array[u8? 70] = [3, 20, 100];

    print(x: chars);

    hold();
}
