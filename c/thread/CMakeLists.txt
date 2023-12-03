
cmake_minimum_required(VERSION 3.20)
project(thread)

add_executable(three-threads
    three_threads.cpp
)

add_executable(three-threads-atomic
    three_threads_atomic.cpp
)
