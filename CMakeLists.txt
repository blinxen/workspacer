cmake_minimum_required(VERSION 3.27.7)
project(workspacer VERSION 0.1.0 LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 20)

# Create variales with sources for both the executable and the library
set(BINARY_SOURCES
    src/utils.cpp
    src/config.cpp
    src/workspaces.cpp
    src/main.cpp

)
set(TERMINAL_LIB_SOURCES
    src/terminal/terminal.cpp
)

add_compile_options(-Wzero-as-null-pointer-constant)

# Dependencies
find_package(ftxui 5 REQUIRED)

# Add terminal library
add_library(terminal STATIC ${TERMINAL_LIB_SOURCES})
# Link TUI library with terminal lib
target_link_libraries(terminal
    PRIVATE ftxui::screen
    PRIVATE ftxui::dom
    PRIVATE ftxui::component
)

add_executable(${PROJECT_NAME} ${BINARY_SOURCES})

# Link library with executable
target_link_libraries(${PROJECT_NAME} terminal)
