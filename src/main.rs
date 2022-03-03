mod operations;

mod input;
use input::get_input;

fn main() {

    let number_one : f32 = get_input();

    let number_two : f32 = get_input();

    println!("\nAddition -> {}", operations::addition::add(number_one, number_two));

    println!("\nSubtraction -> {}", operations::subtraction::subtract(number_one, number_two));

    println!("\nMultiplication -> {}", operations::multiplication::multiply(number_one, number_two));

    println!("\nDivision -> {}", operations::division::divide(number_one, number_two));
}