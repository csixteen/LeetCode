.PHONY : all

all: rust python scala

SRC := src/


#------------------------------
#   Testing Rust code

define test_rust
	@cd $* &&     \
	cargo test --release && \
	cd ..;
endef

DIRS := $(shell find ./src/Rust/ -mindepth 1 -maxdepth 1 -type d)
RUST_JOBS := $(addprefix rjob,${DIRS})

.PHONY : rust

rust: ${RUST_JOBS} ; @echo "[$@] finished!"

${RUST_JOBS}: rjob%:
	$(test_rust)


#-----------------------------
#   Testing Python code

.PHONY : python

python:
	@cd src/Python && python3 -m unittest -vvv


#------------------------------
#   Testing Scala code

.PHONY : scala

scala:
	@cd src/Scala && sbt test
