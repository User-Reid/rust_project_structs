#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) -> &mut Self {
        self.destination = new_destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price = self.price * 1.2;
        self
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut hawaii_trip: Flight = Flight::new(String::from("Houston"), String::from("Hawaii"), 1299.99, 60);

    let new_york_trip: Flight = Flight {
        origin: String::from("Austin, Texas"),
        destination: String::from("New York, New York"),
        ..hawaii_trip
    };

    println!("{:#?}", hawaii_trip);

    hawaii_trip.change_destination(String::from("Ireland")).increase_price().itinerary();
    println!("{}", hawaii_trip.price);

    println!("{:#?}", new_york_trip)
}