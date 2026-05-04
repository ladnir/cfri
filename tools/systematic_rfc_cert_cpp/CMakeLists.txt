cmake_minimum_required(VERSION 3.20)

project(systematic_rfc_cert LANGUAGES CXX)

add_executable(systematic_rfc_cert
    src/main.cpp
)

target_compile_features(systematic_rfc_cert PRIVATE cxx_std_20)

if(MSVC)
    target_compile_options(systematic_rfc_cert PRIVATE /O2 /W4 /permissive-)
else()
    target_compile_options(systematic_rfc_cert PRIVATE -O3 -Wall -Wextra -pedantic)
endif()
