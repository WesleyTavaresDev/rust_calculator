use text_io::read;

pub fn get_input() -> f32 {

    println!("Please, enter a number: ");
    let number : f32 = read!();

    number
}