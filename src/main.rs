fn main() {
    println!("Por favor, introduce tu nombre: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Ahora, introduce tu edad");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let parsed_age: u8 = age.trim().parse().unwrap();

    println!("OK. Entonces, tu nombre es: {}, y tu edad es de {}.", name, parsed_age);
}
