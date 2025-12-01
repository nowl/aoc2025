get-data DAY:
    wget --header "Cookie: $(cat cookies.txt)" https://adventofcode.com/2025/day/{{DAY}}/input -O data/day{{DAY}}

run BIN DATA:
    cargo run --bin {{BIN}} < data/{{DATA}}

run-release BIN DATA:
    cargo run --release --bin {{BIN}} < data/{{DATA}}
