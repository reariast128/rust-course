use regex::Regex;

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}

fn divide(x: i32, y: i32) -> i32 {
    return x / y;
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn evaluate_expression<F>(expression: String, regex_expression: Regex, operation: F) -> String where F: Fn(i32, i32) -> i32 {

    let mut evaluated_expression: String = expression;

    loop {
        /* Capturo la expresión */


        let captures: Option<regex::Captures<'_>> = regex_expression.captures(evaluated_expression.as_str());

        /* A continuación, se va a validar si la captura contiene un None. 
            Si es así, es porque no hay más operaciones, y sale del bucle. 
            Si no, vuelve a evaluar la cadena, para buscar operadores y operandos y resolver la operación. */
        

        if captures.is_none() {
            return evaluated_expression;
        }

        let captures = captures.unwrap();

        let captured_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let operated_value: i32 = operation(left_value, right_value);

        evaluated_expression = evaluated_expression.replace(captured_expression, &operated_value.to_string());
    }
}



fn main() {
    println!("Bienvenidx a la calculadora científica.");

    /* Evaluar una expresión mediante regex.
    La expresión para la suma sería:
        (\d+) \s? \+ \s? (\d+)*/

    let regex_multiply = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let regex_divide = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
    let regex_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_subtract = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    /* Traer los datos del usuario. */

    println!("Por favor, introduce tu expresión:");

    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    /* Evaluar las expresiones */

    let mut result: String = evaluate_expression(expression, regex_multiply, multiply);
    println!("{}", result);
    result = evaluate_expression(result, regex_divide, divide);
    println!("{}", result);
    result = evaluate_expression(result, regex_add, add);
    println!("{}", result);
    result = evaluate_expression(result, regex_subtract, subtract);

    /* Mostrar el resultado */

    println!("El resultado es: {}", result)
}
