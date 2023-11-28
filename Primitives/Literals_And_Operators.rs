//Literals and operators
/*
Annotation:-
hexadecimal----> 0x
octal-----> 0o
binary-----> 0b
*/

//Literals_And_Operators.rs
fn main() {
    // Integer addition
    println!("1 + 5 = {}", 1u32 + 5);

    // Integer subtraction
    println!("2 - 6 = {}", 2i32 - 6);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e5 is {}, -2.5e-3 is {}", 1e5, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("2 << 4 is {}", 2u32 << 4); // 2*pow(2,4);
    println!("80 >> 4 is {}", 80u32 >> 4); // 80/pow(2,4)

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

/*
Output:
1 + 5 = 6
2 - 6 = -4
1e5 is 100000, -2.5e-3 is -0.0025
true AND false is false
true OR false is true
NOT true is false
0011 AND 0101 is 0001
0011 OR 0101 is 0111
0011 XOR 0101 is 0110
2 << 4 is 32
80 >> 4 is 5
One million is written as 1000000
*/
