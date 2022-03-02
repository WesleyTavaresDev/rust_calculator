mod operations;

fn main() {

    let add_result = operations::addition::add(5f32, 5f32);

    println!("{}", add_result);

    let subtract_result = operations::subtraction::subtract(5f32, 4f32);

    println!("{}", subtract_result);

    let multiply_result = operations::multiplication::multiply(5f32, 5f32);

    println!("{}", multiply_result);

    let division_result = operations::division::divide(12f32, 4f32);

    println!("{}", division_result);
}