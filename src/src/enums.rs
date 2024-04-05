//--------------------------------------------------------------------------------------------------
//         Enums
//--------------------------------------------------------------------------------------------------

// Define an enum for the days of the week
#[derive(Debug)]
enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

// Define an enum for different types of travel
enum TravelType {
    Car,
    Bus,
    Train,
    Plane,
}

// Implement methods for the TravelType enum
impl TravelType {
    // Define a method to calculate travel allowance based on miles
    fn travel_allowance(&self, miles: f32) -> f32 {
        // Corrected return type syntax
        // Calculate allowance based on the type of travel
        let allowance = match self {
            TravelType::Car => miles * 2.0,
            TravelType::Bus => miles * 3.0,
            TravelType::Train => miles * 4.0,
            TravelType::Plane => miles * 5.0,
        };
        allowance // Return the calculated allowance
    }
}

// Entry point of the program
fn main() {
    let day = WeekDay::Sunday; // Create a variable for the day, not used in this snippet

    let participant = TravelType::Car; // Define the travel type for a participant
                                       // Print the travel allowance for the participant
    println!(
        "Allowance of participant is: {}",
        participant.travel_allowance(100.0)
    );
}
