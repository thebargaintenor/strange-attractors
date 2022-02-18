A = 2.0
B = 2.0
C = 1.0
D = '-1.0'

ATTRACTOR = clifford

MAX_STEPS = 10000

.PHONY: build
build:
	cargo build

.PHONY: attractor
attractor:
	@cargo run $(ATTRACTOR) -a=$(A) -b=$(B) -c=$(C) -d=$(D) --steps=$(MAX_STEPS)
