fn main() {
    let n1: i32 = 123;
    let n2: i32 = 123;

    println!("Hey! Cuánto es la suma de {} y {}?", n1, n2);
    let sum: i32 = n1 + n2;

    loop {
        let mut user_sum = String::new();
        std::io::stdin().read_line(&mut user_sum).unwrap();

        let parsed_user_sum: i32 = user_sum.trim().parse().unwrap();

        if parsed_user_sum == sum {
            println!("Es correcta tu respuesta!");
            break;
        } else {
            println!("Tu respuesta es incorrecta. Por favor, revísala.");
        }
    }
}