#!/usr/bin/env bash
cargo run localhost:58081 &
PID=$!
for i in {1..10}; do
	sleep 1
	echo 'Waiting for cargo'
done
curl http://localhost:58081/step_digit_cells --header 'Content-Type: application/json' -d "`cat cells`"
for i in {1..10}; do
	sleep 1
	clear
	curl http://localhost:58081/next
done
kill $PID
