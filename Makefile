.PHONY : all

all: rust python

SRC := Algorithms/


#---------------------------------------------------------------------
# Testing Rust code


define test_rust
	@cd $* &&     \
	cargo test && \
	cd ..;
endef

DIRS := $(shell find ./Algorithms/ -mindepth 1 -maxdepth 1 -type d)
RUST_JOBS := $(addprefix rjob,${DIRS})

.PHONY : rust

rust: ${RUST_JOBS} ; @echo "[$@] finished!"

${RUST_JOBS}: rjob%:
	$(test_rust)


#----------------------------------------------------------------------
# Testing Python code


FILES := $(shell find ./Algorithms/ -name '*.py')
PYTHON_JOBS := $(addprefix pjob,${FILES})

.PHONY : python

python: ${PYTHON_JOBS} ; @echo "[$@] finished!"

${PYTHON_JOBS}: pjob%:
	@python3 $*
