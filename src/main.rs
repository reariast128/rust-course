fn main() {
    let mut names: Vec<String> = Vec::new();

    for _ in 1..4 {
        println!("Por favor, introduce un nombre:");
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();

        names.push(name.to_string())
    }

    for names_iter in names {
        println!("El nombre es {}", names_iter);
    }
}