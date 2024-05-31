#include "mylib.h"
#include <stdio.h>

// A really simple function that doubles a number
int double_it(int x) {
    return x * 2;
}

void print_message(const char *message) {
    printf("Printing a message from C: %s\n", message);
}

void print_struct(struct MyStruct s) {
    printf("Printing a struct from C: %d, %d\n", s.x, s.y);
}

void print_ptr_to_struct(struct MyStruct *s) {
    printf("Printing a pointer to a struct from C: %d, %d\n", s->x, s->y);
}

void callme(void (*callback)(int)) {
    printf("Calling a Rust function from C\n");
    callback(42);
}