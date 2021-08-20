// Structs
pub fn objects() {
    // custom data type
    let mut my_flag = Flag {
        country: String::from("Konoha"),
        pcolor: 128,
        scolor: 255
    };

    println!("Flag Specs: {} {} {}", my_flag.country, my_flag.pcolor, my_flag.scolor);
    my_flag.pcolor = 0;
    println!("Flag Specs: {:?}", (my_flag.country, my_flag.pcolor, my_flag.scolor));

    let mut your_flag = FlagTuple(String::from("Castle"), 165, 32);
    println!("Your Flag: {} {} {}", your_flag.0, your_flag.1, your_flag.2);
    your_flag.2 = 12;
    println!("Your Flag: {:?}", (your_flag.0, your_flag.1, your_flag.2));

    let mut sasuke = Ninja::new("Genin", "Fire", "Sharingan");
    println!("Sasuke: {} {} {}", sasuke.level, sasuke.style, sasuke.speciality);
    println!("Sasuke Speciality: {}", sasuke.get_speciality());
    sasuke.set_speciality("Rinnegan");
    println!("Sasuke: {:?}", sasuke.to_tuple());
}

#[derive(Debug)]
struct Ninja {
    level: String,
    style: String,
    speciality: String
}

impl Ninja {
    // Construct Ninja
    fn new(level: &str, style: &str, speciality: &str) -> Ninja {
        Ninja {
            level: level.to_string(),
            style: style.to_string(),
            speciality: speciality.to_string()
        }
    }

    // Get Speciality
    fn get_speciality(&self) -> String {
        format!("{}", self.speciality)
    }

    // Set Speciality
    fn set_speciality(&mut self, speciality: &str) {
        self.speciality = speciality.to_string();
    }

    // Convert To Tuple
    fn to_tuple(self) -> (String, String, String) {
        (self.level, self.style, self.speciality)
    }
}

// Tuple
struct FlagTuple(String, u8, u8);

// Struct
struct Flag {
    country: String,
    pcolor: u8,
    scolor: u8
}