mod operations;

use operations::addition::add;
use operations::subtraction::subtract;
use operations::multiplication::multiply;
use operations::division::divide;

mod input;
use input::get_input;

fn main() {

    let number_one : f32 = get_input();

    let number_two : f32 = get_input();

    println!("\nAddition -> {}", add(number_one, number_two));

    println!("\nSubtraction -> {}", subtract(number_one, number_two));

    println!("\nMultiplication -> {}", multiply(number_one, number_two));

    println!("\nDivision -> {}", divide(number_one, number_two));
}