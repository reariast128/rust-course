fn main() {
    let age: u8 = 18;
    let _unused: u8;
    let name: &str = "Creeping";
    let mut money: u8 = 50;
    println!("Hola soy {} y tengo {} años, y {} pesos", name, age, money);

    money += 30;
    println!("Ahora tengo {} pesos", money)
}
