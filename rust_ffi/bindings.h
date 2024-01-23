#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int32_t add(int32_t a, int32_t b);

char *concatenate_strings(const char *s1, const char *s2);

void free_concatenated_string(char *s);

void print_string(const char *s);

char *get_string(void);

const uint8_t *ffi_string(void);
