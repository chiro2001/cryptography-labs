CMAKE := cmake
BUILDIR := build
BUILDTYPE := Debug

all:
	$(CMAKE) -B $(BUILDIR) -S . -D CMAKE_BUILD_TYPE=$(BUILDTYPE)
	+$(CMAKE) --build $(BUILDIR)

run: all
	cd $(BUILDIR) && ctest

clean:
	-rm -rf $(BUILDIR)

.PHONY: all run