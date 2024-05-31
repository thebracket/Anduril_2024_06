# Everybody's favourite C++ Brain Teaser - std::move

Does anyone want to tell me what this does?

```cpp
#include <iostream>
#include <string>
#include <vector>


int main() {
    std::string s("hello");
    std::vector<std::string> my_vec;
    my_vec.push_back(std::move(s));
    std::cout << s << "\n";
    return 0;
}
```

> On [cpp.sh](https://cpp.sh/) it prints nothing at all.

> **Show of hands** ---- Who knows what `std::move` actually does?

---

`std::move` transforms a variable into an `xvalue`. The `xvalue` is only guaranteed to be sufficiently "alive" to be destroyed. It might still contain your data, it might not!

*Use after move* is a common mistake in C++, and Rust decided that it needed to be a compile-time error.