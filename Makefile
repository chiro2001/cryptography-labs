CMAKE := cmake
BUILD_DIR := build
BUILD_TYPE := Debug
LAB := 1

all:
	$(CMAKE) -B $(BUILD_DIR) -S . -D CMAKE_BUILD_TYPE=$(BUILD_TYPE)
	+$(CMAKE) --build $(BUILD_DIR)

submit:
	@rm -rf submit.zip
	@rm -rf ../.submit
	@mkdir -p ../.submit
	@cp -r * ../.submit
	@find . -wholename "*docs/lab*/*.*" -exec cp {} ../.submit \;
	@rm -rf submit.zip ../submit.zip
	@cd ../.submit && zip ../submit.zip -r .
	@rm -rf ../.submit
	@mv ../submit.zip .

run: all
	cd $(BUILD_DIR) && ctest

clean:
	-rm -rf $(BUILD_DIR)
	-rm -rf .xmake

.PHONY: all run