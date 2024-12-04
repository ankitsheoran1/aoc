pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    Nice(u32),
    Naughty,
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
}

// Move yesterday's function to an associated function in the struct
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }

    let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
    let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

    let ratio = good_deeds / (good_deeds + bad_deeds);

    ratio >= 0.75
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let mut status: Niceness = Niceness::Naughty;
        if is_nice(good_deeds, bad_deeds) {
            status = Niceness::Nice(good_deeds);
        }
        Kid {
            name,
            niceness: status
        }
        // Return a Kid instance
    }
}
