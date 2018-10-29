pub mod nested {
    pub fn function() {
        println!("called `deeply::nested::function()`");
    }
}

pub fn my_first_function() {}

pub fn my_second_function() {}

pub struct AndATraitType();