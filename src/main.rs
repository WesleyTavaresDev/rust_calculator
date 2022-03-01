fn main() {

    let add_result = add(5f32, 5f32);

    println!("{}", add_result);
}

fn add(number_one : f32, number_two : f32) -> f32 {
    number_one + number_two
}