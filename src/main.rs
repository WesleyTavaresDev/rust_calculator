fn main() {

    let add_result = add(5f32, 5f32);

    println!("{}", add_result);

    let subtract_result = subtract(5f32, 4f32);

    println!("{}", subtract_result);

    let multiply_result = multiply(5f32, 5f32);

    println!("{}", multiply_result);
}

fn add(number_one : f32, number_two : f32) -> f32 {
    number_one + number_two
}

fn subtract(number_one : f32, number_two : f32) -> f32 {
    number_one - number_two
}

fn multiply(number_one : f32, number_two : f32) -> f32 {
    number_one * number_two
}