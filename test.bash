echo "================================================="
echo "  RUNNING TSP ALGORITHMS"
echo "================================================="
echo "  PHP 5.6"
( time /usr/bin/php5.6 tsp.php ) >> test_results.txt 2>&1
echo "================================================="
echo "  PHP 7.0"
( time php tsp.php ) >> test_results.txt 2>&1
echo "================================================="
echo "  NODE"
( time node tsp.js ) >> test_results.txt 2>&1
echo "================================================="
echo "  PYTHON"
( time python3 tsp.py ) >> test_results.txt 2>&1
echo "================================================="
echo "  PYTHON ITERTOOLS"
( time python3 tsp_itertools.py ) >> test_results.txt 2>&1
echo "================================================="
echo "  RUBY"
( time ruby tsp.rb ) >> test_results.txt 2>&1
echo "================================================="
echo "  C"
( time ./tsp ) >> test_results.txt 2>&1
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
