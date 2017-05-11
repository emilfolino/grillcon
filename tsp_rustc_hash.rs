use std::collections::HashMap;

fn main() {
    let mut universities = HashMap::new();
    universities.insert("BTH", (56.18, 15.59));
    universities.insert("Uppsala", (59.85, 17.63));
    universities.insert("Lund", (55.71, 13.20));
    universities.insert("KTH", (59.34, 18.07));
    universities.insert("Chalmers", (57.68, 11.97));
    universities.insert("Luleå", (65.61, 22.14));
    universities.insert("Umeå", (63.82, 20.30));
    universities.insert("Linköping", (58.39, 15.57));
    universities.insert("Karlstad", (59.40, 13.58));
    universities.insert("Örebro", (59.25, 15.24));
    universities.insert("Linné", (56.85, 15.24));

    let mut university_names: Vec<&str> = universities.iter().map(|(name, _)| name.clone()).collect();

    let n: usize = university_names.len();

    let mut p = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut i: usize = 0;

    let mut shortest_trip: f64 = 1000000.0;
    let mut shortest_trip_universities: Vec<&str> = Vec::with_capacity(n as usize);
    while i < n {
        p[i] = p[i] - 1;

        let j: usize = i % 2 * p[i];

        university_names.swap(i, j);

        let cities = &university_names;

        let mut trip_length: f64 = 0.0;
        for k in 0..(cities.len() - 1) {
            let coordinates1 = universities.get(cities[k+1]).unwrap();
            let coordinates0 = universities.get(cities[k]).unwrap();

            let lat1: f64 = coordinates1.0;
            let lat0: f64 = coordinates0.0;
            let long1: f64 = coordinates1.1;
            let long0: f64 = coordinates0.1;
            trip_length += ((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0)).sqrt();
        }

        if trip_length < shortest_trip {
            shortest_trip = trip_length;
            shortest_trip_universities = cities.to_vec();
        }

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    println!("Shortest trip length: {}", shortest_trip);
    println!("{}", shortest_trip_universities.join(" --> "));
}
