TESTS :=  ../test/print/*.json \
		../test/parse/*.bril

.PHONY: test
test:
	turnt --diff -c turnt_bril_rs.toml $(TESTS)

.PHONY: install
install:
	cargo install --path . --example bril2txt
	cargo install --path ./bril2json

# As more features are added it can be difficult to know if any of them conflict or haven't been appropriately guarded. This command runs cargo check with all possible combinations of feature flags to catch any breakages. Normally you would have to be careful of 2^N explosion but bril-rs builds so fast that this is currently not an issue.
# cargo install cargo-hack
.PHONY: features
features:
	cargo hack check --feature-powerset --no-dev-deps