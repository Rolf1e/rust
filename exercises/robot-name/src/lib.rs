use rand::Rng;

pub struct Robot{
    name : String
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: String::from(generate_name())}
    }

    pub fn name(&self) -> &str {
        println!("{}", &self.name);
        &self.name
    }

    pub fn reset_name(&mut self) {
        println!("{}", &self.name);
        self.name = generate_name()
    }

}

fn generate_name() -> String { 
    let number = generate_random_number();
    match number {
        0 ..=9 => format!("{}{}00{}", generate_random_chars(), generate_random_chars(), number),
        10 ..=99 => format!("{}{}0{}", generate_random_chars(), generate_random_chars(), number),
        _ => format!("{}{}{}", generate_random_chars(), generate_random_chars(), number),
    }
}


fn generate_random_chars() -> char {
    rand::thread_rng()
        .gen_range(b'A', b'Z') as char
}

fn generate_random_number() -> i32 {
    rand::thread_rng()
        .gen_range(0, 999)
}
