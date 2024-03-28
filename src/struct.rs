//------------------------------
//       Structs and its types
//------------------------------

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}
// Defines methods associated with the Car struct
impl Car {
    // methods for Car struct
    fn display_car_info(self) {
        // self is the instance of the struct
        println!("Owner: {}", self.owner);
        println!("Year: {}", self.year);
        println!("Fuel Level: {}", self.fuel_level);
        println!("Price: {}", self.price);
    }
}

fn display_car_info(car: &Car) {
    println!("Owner: {}", car.owner);
    println!("Year: {}", car.year);
    println!("Fuel Level: {}", car.fuel_level);
    println!("Price: {}", car.price);
}

fn main() {
    let my_car = Car {
        owner: String::from("John Doe"),
        year: 2010,
        fuel_level: 0.5,
        price: 10000,
    };
    let car_year = my_car.year;
    let another_car = Car {
        owner: String::from("Jane Doe"),
        ..my_car // copy the rest of the fields from my_car
    };
    println!("Car year: {}", car_year);

    // Tuple struct
    let point_2D = (1, 2);
    let point_3D = (1, 2, 3);
    let point_4D = (1, 2, 3, 4);

    struct Point2D(i32, i32);
    struct Point3D(i32, i32, i32);
    struct Point4D(i32, i32, i32, i32);

    let point_1 = Point2D(1, 2);
    let point_2 = Point3D(1, 2, 3);
    let point_3 = Point4D(1, 2, 3, 4);

    my_car.display_car_info();
}
