#include <string>
#include <iostream>
#include <memory>
#include <cassert>

int main() {
    auto str = std::make_unique<std::string>("Hello World");
    std::string & str_ref = *str;
    str.release();
    assert(str.get() == NULL);

    // Imagine a few thousands lines of code between here and there.

    std::cout << str_ref << std::endl;
    return 0;
}