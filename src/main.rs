use rand::Rng;

fn gen_pass(len: usize) -> String {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz\
         ABCDEFGHIJKLMNOPQRSTUVWXYZ\
         0123456789\
         !@#$%^&*()-_=+[]{};:,.?/";

    // символы ASCII, поэтому разбиваем на char

    let chars: Vec<char> = ALPHABET.chars().collect();
    let mut rng = rand::rng();
    (0..len)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

fn main() {
    let n: usize = 16; // длина пороля
    println!("Your random password: {}", gen_pass(n));
}
