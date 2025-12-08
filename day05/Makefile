.PHONY: run
run:
	@GC_MAX_HEAP_SIZE=2g nix eval --impure --expr '(import ./day05.nix {}).main'

.PHONY: test
test:
	@nix eval --impure --expr '(import ./test.nix {})'

