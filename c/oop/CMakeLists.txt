
cmake_minimum_required(VERSION 3.20)
project(object)

set(SOURCE_FILES
    button.c
    button.h
    object.c
    object.h
    widget.c
    widget.h
)

add_library(object-static STATIC
    ${SOURCE_FILES}
)

add_executable(test-button
    main.c
)
target_link_libraries(test-button
    object-static
)
