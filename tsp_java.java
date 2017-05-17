import java.util.List;

public class tsp_java {
    public static void main(String[] args) {
        int n = 11;

        String[] universities = {"BTH", "Uppsala", "Lund", "KTH", "Chalmers", "Luleå", "Umeå", "Linköping", "Karlstad", "Örebro", "Linné"};

        List university_list = java.util.Arrays.asList(universities);

        String[] mut_universities = {"BTH", "Uppsala", "Lund", "KTH", "Chalmers", "Luleå", "Umeå", "Linköping", "Karlstad", "Örebro", "Linné"};

        double[][] coordinates = {{56.18, 15.59}, {59.85, 17.63}, {55.71, 13.20}, {59.34, 18.07}, {57.68, 11.97}, {65.61, 22.14}, {63.82, 20.30}, {58.39, 15.57}, {59.40, 13.58}, {59.25, 15.24}, {56.85, 14.82}};

        int p[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};

        double shortest_trip = Double.MAX_VALUE;
        String shortest_trip_universities[] = {"", "", "", "", "", "", "", "", "", "", ""};

        int i = 0;
        while (i < n) {
            p[i]--;

            int j = i%2 * p[i];
            String tmp = mut_universities[i];
            mut_universities[i] = mut_universities[j];
            mut_universities[j] = tmp;

            double trip_length = 0.0;
            for (int k = 0; k < n - 1; k++) {
                int index0 = university_list.indexOf(mut_universities[k]);
                int index1 = university_list.indexOf(mut_universities[k + 1]);

                double lat1 = coordinates[index1][0];
                double lat0 = coordinates[index0][0];
                double long1 = coordinates[index1][1];
                double long0 = coordinates[index0][1];

                trip_length += Math.sqrt((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0));
            }

            if (trip_length < shortest_trip) {
                shortest_trip = trip_length;
                for (int m = 0; m < n; m++) {
                    shortest_trip_universities[m] = mut_universities[m];
                }
            }

            i = 1;
            while (p[i] == 0) {
                p[i] = i;
                i++;
            }
        }

        System.out.format("Shortest trip length: %f\n", shortest_trip);
        System.out.print(String.join(" --> ", shortest_trip_universities));
        System.out.format("\nExecution time: \n");
    }
}
