//------------------------------
//       Structs and its types
//------------------------------

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
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
}
