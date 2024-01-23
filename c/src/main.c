#include <stdio.h>
#include <dlfcn.h>
#include "../../rust_ffi/bindings.h"


int main() {
    // Call the function `add`
    int result = add(35, 34);
    printf("Result from external addition of 35 and 34: %d\n", result);

    // Call and concat
    char* s1 = "Hello, ";
    char* s2 = "world!";
    char* concatenated = concatenate_strings(s1, s2);
    printf("%s\n", concatenated);

    return 0;
}