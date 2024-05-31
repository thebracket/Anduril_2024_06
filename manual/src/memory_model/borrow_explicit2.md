# Spot the Difference

> Let's make it a little easier

<table>
<tr>
</tr>
<tr>
<td>

```cpp
#include <iostream>

class MyClass {
    public:
    MyClass(std::string s) {
        this->message = s;
    }
    
    MyClass(MyClass &c) {
        std::cout << "A copy happened!\n";
    }
    
    void print() {
        std::cout << this->message << "\n";
    }
    
    private:
    std::string message;
};

void print_me(MyClass &c) {
    c.print();
}

int main() {
    MyClass hello("hello");
    print_me(hello);
    
    return 0;
}
```

</td>
<td>

```cpp
#include <iostream>

class MyClass {
    public:
    MyClass(std::string s) {
        this->message = s;
    }
    
    MyClass(MyClass &c) {
        std::cout << "A copy happened!\n";
    }
    
    void print() {
        std::cout << this->message << "\n";
    }
    
    private:
    std::string message;
};

void print_me(MyClass c) {
    c.print();
}

int main() {
    MyClass hello("hello");
    print_me(hello);
    
    return 0;
}
```

</td>
</table>

