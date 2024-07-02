run:
	cargo run
lib:
	cargo test --lib
test_stack:
	 cargo test --test stack_overflow
test_panic:
	 cargo test --test should_panic