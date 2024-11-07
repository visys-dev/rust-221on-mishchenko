// Method
struct Rectangle_method_01 {
    width: u32,
    height: u32,
}

impl Rectangle_method_01 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[test]
fn test_method_01() {
    let rect1 = Rectangle_method_01 { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);
}



#[derive(Debug)]
struct TrafficLight_method_02 {
    color: String,
}

impl TrafficLight_method_02 {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}
#[test]
fn test_method_02() {
    let light = TrafficLight_method_02 {
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here
    light.show_state();
    // ..otherwise, there will be an error below
    println!("{:?}", light);
}



struct TrafficLight_method_03 {
    color: String,
}

impl TrafficLight_method_03 {
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
#[test]
fn test_method_03() {}




#[derive(Debug)]
struct TrafficLight_method_04 {
    color: String,
}

impl TrafficLight_method_04 {
    pub fn new() -> Self {
        Self {
            color: "red".to_string()
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn test_method_04() {
    let light = TrafficLight_method_04::new();
    assert_eq!(light.get_state(), "red");
}


//
struct Rectangle_method_05 {
    width: u32,
    height: u32,
}


impl Rectangle_method_05 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle_method_05 {
    fn can_hold(&self, other: &Rectangle_method_05) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[test]
fn test_method_05() {}


//
#[derive(Debug)]
enum TrafficLightColor_method_06 {
    Red,
    Yellow,
    Green,
}

// implement TrafficLightColor with a method
impl TrafficLightColor_method_06 {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor_method_06::Red => "red".to_string(),
            TrafficLightColor_method_06::Yellow => "yellow".to_string(),
            TrafficLightColor_method_06::Green => "green".to_string(),
        }
    }
}

fn test_method_06() {
    let c = TrafficLightColor_method_06::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
//


