# Nom de l'exécutable
TARGET = main

# Compilateur à utiliser
CC = gcc

# Options de compilation
CFLAGS = -Wall -g -gdwarf-2

# Bibliothèques à lier
LIBS = -ldl

# Règle par défaut
all: librs $(TARGET)

# Règle pour l'exécutable
$(TARGET): src/*.c
	$(CC) $(CFLAGS) -o $(TARGET) src/*.c $(LIBS) -L ../rust/rust_ffi/target/debug/ -lrust_ffi

# Règle pour nettoyer les fichiers générés
clean:
	rm -f $(TARGET)


.phony: librs all run

librs:
	cd ../rust/rust_ffi && cargo build

run: all
	LD_LIBRARY_PATH="$(LD_LIBRARY_PATH):../rust/rust_ffi/target/debug/" ./$(TARGET)
