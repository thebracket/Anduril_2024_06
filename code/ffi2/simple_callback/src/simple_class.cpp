#include "simple_class.h"
#include <iostream>
#include "simple_callback/src/main.rs.h"


SimpleClass::SimpleClass() {
    std::cout << "SimpleClass constructor\n";
    this->counter = 1;
    callback();
}

SimpleClass::~SimpleClass() {
    std::cout << "SimpleClass destructor\n";
}

void SimpleClass::set_counter(uint64_t value) {
    this->counter = value;
}

void SimpleClass::say_hello() const {
    for (int i = 0; i < this->counter; i++) {
        std::cout << "Hello from SimpleClass run (" << i << ")\n";
    }
}

std::unique_ptr<SimpleClass> create_simple_class() {
    return std::make_unique<SimpleClass>();
}