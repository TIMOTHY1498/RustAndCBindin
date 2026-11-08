#include <stdio.h>

int printFromRust(char* passedValue) {
    printf("Hello from Rust!\n");
    printf("Value passed from Rust: %s\n", passedValue);
    return 0;
}
