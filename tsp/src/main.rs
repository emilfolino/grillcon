extern crate time;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    let original_university_names: Vec<String> = vec!["BTH".to_string(), "Uppsala".to_string(), "Lund".to_string(), "KTH".to_string(), "Chalmers".to_string(), "Luleå".to_string(), "Umeå".to_string(), "Linköping".to_string(), "Karlstad".to_string(), "Örebro".to_string(), "Linné".to_string()];
    let university_coordinates: Vec<(f64, f64)> = vec![(56.18, 15.59), (59.85, 17.63), (55.71, 13.20), (59.34, 18.07), (57.68, 11.97), (65.61, 22.14), (63.82, 20.30), (58.39, 15.57), (59.40, 13.58), (59.25, 15.24), (56.85, 14.82)];

    let mut university_names: Vec<String> = original_university_names.to_vec();
    let n: usize = university_names.len();

    let mut p = vec![n; n + 1];
    let mut i: usize = 0;

    let mut shortest_trip: f64 = 1000000.0;
    let mut shortest_trip_universities: Vec<String> = Vec::with_capacity(n as usize);

    while i < n {
        p[i] = p[i] - 1;

        let j: usize = i % 2 * p[i];

        university_names.swap(i, j);

        let cities = university_names.clone();
        let mut trip_length: f64 = 0.0;
        for k in 0..(cities.len() - 1) {
            let index1 = original_university_names.iter().position(|ref r| r == &cities[k + 1].as_str()).unwrap();
            let index0 = original_university_names.iter().position(|ref r| r == &cities[k].as_str()).unwrap();

            let lat1: f64 = university_coordinates[index1].0;
            let lat0: f64 = university_coordinates[index0].0;
            let long1: f64 = university_coordinates[index1].1;
            let long0: f64 = university_coordinates[index0].1;
            trip_length += ((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0)).sqrt();
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

    println!("Shortest trip length: {}", shortest_trip);
    println!("{}", shortest_trip_universities.join(" --> "));
    println!("Total Execution Time: {} seconds", start.to(end));
}
