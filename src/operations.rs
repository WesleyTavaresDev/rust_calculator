pub mod operations {
    pub mod addition {
       pub fn add(number_one : f32, number_two : f32) -> f32 {
           number_one + number_two
       } 
    }

    pub mod subtraction {
        pub fn subtract(number_one : f32, number_two : f32) -> f32 {
            number_one - number_two
        }
    }

    pub mod multiplication {
        pub fn multiply(number_one : f32, number_two : f32) -> f32{
            number_one * number_two
        }
    }
} 