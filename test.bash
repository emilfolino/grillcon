echo "================================================="
echo "  RUNNING TSP ALGORITHMS"
echo "================================================="
# echo "  PHP 5.6"
# time /usr/bin/php5.6 tsp.php
# echo "================================================="
# echo "  PHP 7.0"
# time php tsp.php
# echo "================================================="
# echo "  NODE"
# time node tsp.js
# echo "================================================="
# echo "  PYTHON"
# time python3 tsp.py
# echo "================================================="
# echo "  PYTHON ITERTOOLS"
# time python3 tsp_itertools.py
# echo "================================================="
# echo "  RUBY"
# time ruby tsp.rb
# echo "================================================="
echo "  RUST HASH"
time ./tsp_rustc_hash
echo "================================================="
echo "  RUST"
time ./tsp_rustc
echo "================================================="
