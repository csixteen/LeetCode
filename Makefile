.PHONY : rust

SRC := Algorithms/

define test_fun
	@cd $* &&     \
	cargo test && \
	cd ..;
endef

DIRS := $(shell find ./Algorithms/ -mindepth 1 -maxdepth 1 -type d)
JOBS := $(addprefix job,${DIRS})

rust: ${JOBS} ; @echo "[$@] finished!"

${JOBS}: job%:
	$(test_fun)
