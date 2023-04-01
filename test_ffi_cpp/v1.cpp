// v1.cpp
int foo(){
    return 1;
}

#include <exception>
#include <stdexcept>

extern "C" {
    void c_func() {
        throw std::runtime_error("AAAAAA");
    }
}