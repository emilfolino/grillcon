fn main() {
    // Points are universities in Sweden
    // Order is: BTH, Uppsala, Lund, KTH, Chalmers, Luleå, Umeå,
    // Linköping, Karlstad, Örebro
    let universities: [(f32, f32); 10] = [(56.18, 15.59), (59.85, 17.63), (55.71, 13.20), (59.34, 18.07), (57.68, 11.97), (65.61, 22.14), (63.82, 20.30), (58.39, 15.57), (59.40, 13.58), (59.25, 15.24)];

    println!("{}", universities[0].0);
}
