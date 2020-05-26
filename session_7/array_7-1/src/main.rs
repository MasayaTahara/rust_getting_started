fn main() {
    // Array basic
    let ary = [1, 2, 3, 4, 5];

    //Specify the type of array and length
    let ary: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];

    // Initialize
    let mut ary: [u8; 16] = [0; 16];
    // You can assign because it is mut
    ary[0] = 10;
}
