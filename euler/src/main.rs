// fn euler1() -> u32 {
//     let mut le_sum: u32 = 0;
//     for n in 1..333 {
//         le_sum += n;
//     }
//     le_sum
// }

fn euler2() -> u32 {
    fn a_plus_b(a: u32, b: &u32) -> u32 {
        a + *b
    }

    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;

    while b < 1000 {
        let c = a_plus_b(a, &b);
        a = b;
        b = c;
        if b % 2 == 0 {
            sum += b
        }
    }
    sum
}

fn main() {
    // println!("{}", euler1());
    println!("{}", euler2());
}
