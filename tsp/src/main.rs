extern crate time;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    let original_university_names: Vec<String> = vec!["BTH".to_string(), "Uppsala".to_string(), "Lund".to_string()];
    let university_coordinates: Vec<(f64, f64)> = vec![(56.18, 15.59), (59.85, 17.63), (55.71, 13.20)];

    let mut university_names: Vec<String> = original_university_names.to_vec();
    let n: usize = university_names.len();

    let mut p = vec![n; n + 1];
    let mut i: usize = 0;

    let mut shortest_trip: i32 = 1000000;
    let mut shortest_trip_universities: Vec<String> = Vec::with_capacity(n as usize);

    while i < n {
        p[i] = p[i] - 1;

        let j: usize = i % 2 * p[i];

        university_names.swap(i, j);

        let cities = university_names.clone();
        let mut trip_length: i32 = 0;
        for k in 0..(cities.len() - 1) {
            let index = original_university_names.iter().position(|ref r| r == cities[k + 1]).unwrap();

            let lat1: f64 = university_coordinates[index].0;
            println!("{}", lat1);
            // let lat0: f64 = universities[cities[k].to_string()].0;
            // let long1: f64 = universities[cities[k + 1].to_string()].1;
            // let long0: f64 = universities[cities[k].to_string()].1;
            // trip_length += sqrt((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0));
        }

        if trip_length < shortest_trip {
            shortest_trip = trip_length;
            shortest_trip_universities = cities;
        }

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    let end = PreciseTime::now();

    // println!("Shortest trip length: " . $shortest_trip);
    // println!(implode(" --> ", $shortest_trip_universities));
    println!("Total Execution Time: {} seconds", start.to(end));
}
