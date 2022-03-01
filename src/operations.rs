pub mod addition;
pub mod subtraction;

pub mod operations {

    pub mod multiplication {
        pub fn multiply(number_one : f32, number_two : f32) -> f32{
            number_one * number_two
        }
    }

    pub mod division {
        pub fn divide(number_one : f32, number_two : f32) -> f32 {
            number_one / number_two
        }
    }
} 