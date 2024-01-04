use regex::Regex;

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

    /* Aplicar las operaciones */

    // Multiplicar
    loop {
        /* Capturo la expresión */

        let captures: Option<regex::Captures<'_>> = regex_multiply.captures(expression.as_str());

        /* A continuación, se va a validar si la captura contiene un None. 
            Si es así, es porque no hay más operaciones, y sale del bucle. 
            Si no, vuelve a evaluar la cadena, para buscar operadores y operandos y resolver la operación. */
        

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();

        let captured_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let sum: i32 = left_value * right_value;

        expression = expression.replace(captured_expression, &sum.to_string());
    }

    // Dividir
    loop {
        /* Capturo la expresión */

        let captures: Option<regex::Captures<'_>> = regex_divide.captures(expression.as_str());

        /* A continuación, se va a validar si la captura contiene un None. 
            Si es así, es porque no hay más operaciones, y sale del bucle. 
            Si no, vuelve a evaluar la cadena, para buscar operadores y operandos y resolver la operación. */
        

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();

        let captured_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let sum: i32 = left_value / right_value;

        expression = expression.replace(captured_expression, &sum.to_string());
    }

    // Suma
    loop {
        /* Capturo la expresión */

        let captures: Option<regex::Captures<'_>> = regex_add.captures(expression.as_str());

        /* A continuación, se va a validar si la captura contiene un None. 
            Si es así, es porque no hay más operaciones, y sale del bucle. 
            Si no, vuelve a evaluar la cadena, para buscar operadores y operandos y resolver la operación. */
        

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();

        let captured_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let sum: i32 = left_value + right_value;

        expression = expression.replace(captured_expression, &sum.to_string());
    }

    // Resta
    loop {
        /* Capturo la expresión */

        let captures: Option<regex::Captures<'_>> = regex_subtract.captures(expression.as_str());

        /* A continuación, se va a validar si la captura contiene un None. 
            Si es así, es porque no hay más operaciones, y sale del bucle. 
            Si no, vuelve a evaluar la cadena, para buscar operadores y operandos y resolver la operación. */
        

        if captures.is_none() {
            break;
        }

        let captures = captures.unwrap();

        let captured_expression = captures.get(0).unwrap().as_str();
        let left_value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let sum: i32 = left_value - right_value;

        expression = expression.replace(captured_expression, &sum.to_string());
    }


    /* Mostrar el resultado */

    println!("El resultado es: {}", expression)
}
