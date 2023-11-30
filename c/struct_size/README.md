
# 关于
C 语言里声明的空结构体, 其占用的内存大小为0字节.

而 C++ 里面的空结构体/空白类的实例, 是要占用一个字节的. 这个字节用于该对象的
unique address identifier.

如果一个类有多个实例, 那它们拥有不同的内存地址.

## Reference
[Geekds](https://www.geeksforgeeks.org/why-is-the-size-of-an-empty-class-not-zero-in-c/)
