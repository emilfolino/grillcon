echo "================================================="
echo "  RUNNING TSP ALGORITHMS"
echo "================================================="
echo "  PHP 5.6"
echo time /usr/bin/php5.6 tsp.php
echo "================================================="
echo "  PHP 7.0"
echo time php tsp.php
echo "================================================="
echo "  NODE"
echo time node tsp.js
echo "================================================="
echo "  PYTHON"
echo time python3 tsp.py
echo "================================================="
echo "  PYTHON ITERTOOLS"
echo time python3 tsp_itertools.py
echo "================================================="
echo "  RUBY"
echo time ruby tsp.rb
echo "================================================="
echo "  C"
echo time ./tsp
echo "================================================="
echo "  Java"
( time java tsp_java ) >> test_results.txt 2>&1
echo "================================================="
echo "  RUST HASH"
( time ./tsp_rustc_hash ) >> test_results.txt 2>&1
echo "================================================="
echo "  RUST"
( time ./tsp_rustc ) >> test_results.txt 2>&1
echo "================================================="
