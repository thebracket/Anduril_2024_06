#pragma once
#include <memory>

class SimpleClass {
    public:
    SimpleClass();
    void set_counter(uint64_t value);
    void say_hello() const;
    ~SimpleClass();

    private:
    uint64_t counter;
};

std::unique_ptr<SimpleClass> create_simple_class();