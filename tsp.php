<?php

ini_set('memory_limit', '6120M');
set_time_limit(300);

$time_start = microtime(true);

$universities = [
    'BTH' => ["lat" => 56.18, "long" => 15.59],
    'Uppsala' => ["lat" => 59.85, "long" => 17.63],
    'Lund' => ["lat" => 55.71, "long" => 13.20],
    'KTH' => ["lat" => 59.34, "long" => 18.07],
    'Chalmers' => ["lat" => 57.68, "long" => 11.97],
    'Luleå' => ["lat" => 65.61, "long" => 22.14],
    'Umeå' => ["lat" => 63.82, "long" => 20.30],
    'Linköping' => ["lat" => 58.39, "long" => 15.57],
    'Karlstad' => ["lat" => 59.40, "long" => 13.58],
    'Örebro' => ["lat" => 59.25, "long" => 15.24]
];

$university_names = array_keys($universities);

$permutations = [];

$N = count($university_names);
$p = array_fill(0, $N + 1, $N);
$i = 0;

while ($i < $N) {
    $p[$i]--;

    $j = $i%2 * $p[$i];

    $tmp = $university_names[$i];
    $university_names[$i] = $university_names[$j];
    $university_names[$j] = $tmp;

    $permutations[implode("", array_values($university_names))] = $university_names;

    $i = 1;
    while ($p[$i] == 0) {
        $p[$i] = $i;
        $i++;
    }
}

$shortest_trip = PHP_INT_MAX;
$shortest_trip_universities = [];

foreach ($permutations as $key => $trip) {
    $trip_length = 0;

    for ($i = 0; $i < count($trip) - 1; $i++) {
        $lat1 = $universities[$trip[$i + 1]]["lat"];
        $lat0 = $universities[$trip[$i]]["lat"];
        $long1 = $universities[$trip[$i + 1]]["long"];
        $long0 = $universities[$trip[$i]]["long"];
        $trip_length += sqrt(($lat1 - $lat0)*($lat1 - $lat0) + ($long1 - $long0)*($long1 - $long0));
    }

    if ($trip_length < $shortest_trip) {
        $shortest_trip = $trip_length;
        $shortest_trip_universities = $trip;
    }
}

$time_end = microtime(true);

$execution_time = ($time_end - $time_start);



echo "Shortest trip length: " . $shortest_trip . "<br />";
echo implode(" --> ", $shortest_trip_universities) . "<br />";
echo '<b>Total Execution Time:</b> '.$execution_time.' seconds';
