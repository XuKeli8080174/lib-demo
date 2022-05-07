#ifdef _WIN32
    #include <stdlib.h>
    __declspec(dllexport)
#endif
int add(int a, int b) {
    return a + b;
}