# make USE_SANITIZERS=1 to build with sanitizers
USE_SANITIZERS ?= 0

DEBUG ?= 0

# OS-specific flags
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
LDFLAGS = -undefined dynamic_lookup
DYN_LIB_EXT = dylib

else ifeq ($(UNAME_S),Linux)
DYN_LIB_EXT = so

else ifeq ($(UNAME_S),Windows_NT)
# LDFLAGS = -lws2_32
DYN_LIB_EXT = dll
endif

# Name of the library
TARGET = libtree.$(DYN_LIB_EXT)

# Compiler to use
CC = gcc

# Path to the compiled c common library
C_COMMON_INC_PATH = ./src/include

# Path to the Rust project directory for bindings.h
RUST_PROJECT_DIR = ../rust_ffi

# Compilation flags
CFLAGS = -I $(RUST_PROJECT_DIR)/ -I $(C_COMMON_INC_PATH) -Wall -Wextra -Wunused-result -fPIC

ifeq ($(USE_SANITIZERS), 1)
CFLAGS += -fsanitize=address,undefined,leak
endif

ifeq ($(DEBUG), 1)
CFLAGS += -O0 -g -gdwarf-2
else
CFLAGS += -O2
endif

# C source files
SRC_C = $(wildcard src/*.c)

# Object files
OBJ_C = $(SRC_C:.c=.o)

# Dependencies for C source files
DEP_C = $(SRC_C:.c=.d)

# Default rule
all: $(TARGET)

# Rule for building the shared library
$(TARGET): $(OBJ_C)
	$(CC) $(CFLAGS) $(LDFLAGS) -shared -o $(TARGET) $(OBJ_C)

# Rule for building object files from C source files
%.o: src/%.c | $(DEP_C)
	$(CC) $(CFLAGS) -c $< -o $@

# Rule for generating dependency files
%.d: src/%.c
	$(CC) $(CFLAGS) -MMD -MP -MF $@ -c $< -o $(patsubst %.d,%.o,$@)

# Rule for cleaning up generated files
clean:
	rm -f $(OBJ_C) $(DEP_C) $(TARGET)
	rm -f compile_commands.json

compile_commands.json: $(SRC_C)
	bear -- make

# Include the dependency on bindings.h for each .d file
$(DEP_C): $(RUST_PROJECT_DIR)/bindings.h

# Declare phony targets
.PHONY: all clean compile_commands.json
