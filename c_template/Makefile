# make USE_SANITIZERS=1 to build with sanitizers
USE_SANITIZERS ?= 0

DEBUG ?= 0

# Name of the executable
TARGET = main

# Compiler to use
CC = gcc

# Path to the c common library project directory
C_COMMON_DIR = ../c_common

# Path to the compiled c common library
C_COMMON_INC_PATH = $(C_COMMON_DIR)/src/include

# Path to the compiled c common library
C_COMMON_LIB_PATH = $(C_COMMON_DIR)/

# Path to the Rust project directory
RUST_PROJECT_DIR = ../rust_ffi

# Path binding.h
INCLUDE_PATH = -I $(RUST_PROJECT_DIR) -I $(C_COMMON_INC_PATH)

# Path to the compiled Rust library
LIB_PATH = -L $(RUST_PROJECT_DIR)/ -L $(C_COMMON_LIB_PATH)

# Compilation flags
CFLAGS = $(INCLUDE_PATH) -Wall -Wextra -Wunused-result -fPIC

ifeq ($(USE_SANITIZERS), 1)
CFLAGS += -fsanitize=address,undefined,leak
endif

ifeq ($(DEBUG), 1)
CFLAGS += -O0 -g -gdwarf-2
else
CFLAGS += -O2
endif

# OS-specific flags
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
DYN_LIB_EXT = dylib

else ifeq ($(UNAME_S),Linux)
DYN_LIB_EXT = so

else ifeq ($(UNAME_S),Windows_NT)
DYN_LIB_EXT = dll
endif

C_COMMON_LIB = libtree.$(DYN_LIB_EXT)

# Libraries to link
LIBS = -lrust_ffi -ltree

# C source files
SRC_C = $(wildcard src/*.c)

# Object files
OBJ_C = $(SRC_C:.c=.o)

# Dependencies for C source files
DEP_C = $(SRC_C:.c=.d)

# Default rule
all: $(TARGET)

# Rule for building the c_common library
$(C_COMMON_DIR)/$(C_COMMON_LIB):
	$(MAKE) -C $(C_COMMON_DIR) all

# Rule for building the executable
$(TARGET): $(OBJ_C) $(C_COMMON_DIR)/$(C_COMMON_LIB)
	$(CC) $(CFLAGS) -Wl,-rpath=$(RUST_PROJECT_DIR)/ -Wl,-rpath=$(C_COMMON_LIB_PATH) -o $(TARGET) $(OBJ_C) $(LIB_PATH) $(LIBS)

# Rule for building object files from C source files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Rule for generating dependencies for C source files
%.d: %.c
	$(CC) $(CFLAGS) -MM -MT $@ $< > $@

# Rule for cleaning up generated files
clean:
	rm -f $(TARGET) $(OBJ_C) $(DEP_C)
	rm -f compile_commands.json
	$(MAKE) -C $(C_COMMON_DIR) clean

compile_commands.json: $(SRC_C)
	bear -- make

.phony: all clean valgrind help

# Rule for running the executable with valgrind
valgrind: $(TARGET)
	valgrind --leak-check=full ./$(TARGET)

# Rule for displaying help
help:
	@echo "Makefile for building the project"
	@echo "Available targets:"
	@echo "  all       - Build the project and generate the executable"
	@echo "  clean     - Delete the generated files"
	@echo "  valgrind  - Run the executable with valgrind"
	@echo "  help      - Display this help"

# Rule for displaying make variables
show-variables:
	@echo "C_COMMON_DIR=$(C_COMMON_DIR)"
	@echo "C_COMMON_INC_PATH=$(C_COMMON_INC_PATH)"
	@echo "C_COMMON_LIB_PATH=$(C_COMMON_LIB_PATH)"
	@echo "RUST_PROJECT_DIR=$(RUST_PROJECT_DIR)"
	@echo "INCLUDE_PATH=$(INCLUDE_PATH)"
	@echo "LIB_PATH=$(LIB_PATH)"
	@echo "CFLAGS=$(CFLAGS)"
	@echo "LIBS=$(LIBS)"
	@echo "SRC_C=$(SRC_C)"
	@echo "OBJ_C=$(OBJ_C)"
	@echo "DEP_C=$(DEP_C)"

