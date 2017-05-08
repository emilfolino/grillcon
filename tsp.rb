start = Time.now

universities = {
    'BTH' => {"lat" => 56.18, "long" => 15.59},
    'Uppsala' => {"lat" => 59.85, "long" => 17.63},
    'Lund' => {"lat" => 55.71, "long" => 13.20},
    'KTH' => {"lat" => 59.34, "long" => 18.07},
    'Chalmers' => {"lat" => 57.68, "long" => 11.97},
    'Luleå' => {"lat" => 65.61, "long" => 22.14},
    'Umeå' => {"lat" => 63.82, "long" => 20.30},
    'Linköping' => {"lat" => 58.39, "long" => 15.57},
    'Karlstad' => {"lat" => 59.40, "long" => 13.58},
    'Örebro' => {"lat" => 59.25, "long" => 15.24},
    'Linné' => {"lat" => 56.85, "long" => 14.82}
}

university_names = universities.keys()
permutations = {}

N = university_names.length
p = [N]*(N + 1)

shortest_trip = 1000000
shortest_trip_universities = {}

i = 0
while i < N
    p[i] = p[i] - 1

    j = i%2 * p[i]

    tmp = university_names[i]
    university_names[i] = university_names[j]
    university_names[j] = tmp

    cities = university_names.clone
    trip_length = 0
    for i in 0..(cities.length - 2)
        lat1 = universities[cities[i + 1]]["lat"]
        lat0 = universities[cities[i]]["lat"]
        long1 = universities[cities[i + 1]]["long"]
        long0 = universities[cities[i]]["long"]
        trip_length += Math.sqrt((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0))
    end

    if trip_length < shortest_trip
        shortest_trip = trip_length
        shortest_trip_universities = cities
    end

    i = 1
    while p[i] == 0
        p[i] = i
        i += 1
    end
end



finish = Time.now

puts "Shortest trip length: " + shortest_trip.to_s
puts shortest_trip_universities.join(" --> ")

diff = finish - start
puts "Execution time: " + diff.to_s
