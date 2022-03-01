mod operations;

fn main() {

    let add_result = operations::operations::addition::add(5f32, 5f32);

    println!("{}", add_result);

    let subtract_result = subtract(5f32, 4f32);

    println!("{}", subtract_result);

    let multiply_result = multiply(5f32, 5f32);

    println!("{}", multiply_result);

    let division_result = division(12f32, 4f32);

    println!("{}", division_result);
}

fn subtract(number_one : f32, number_two : f32) -> f32 {
    number_one - number_two
}

fn multiply(number_one : f32, number_two : f32) -> f32 {
    number_one * number_two
}

fn division(number_one : f32, number_two : f32) -> f32 {
    number_one / number_two
}