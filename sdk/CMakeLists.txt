cmake_minimum_required(VERSION 3.13)

include(pico_sdk_import.cmake)

project(pico_examples C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

if (PICO_SDK_VERSION_STRING VERSION_LESS "1.4.0")
    message(FATAL_ERROR "Raspberry Pi Pico SDK version 1.4.0 (or later) required. Your version is ${PICO_SDK_VERSION_STRING}")
endif()

pico_sdk_init()

add_library(drone_raspberrypi_pico_sdk wrapper.c)

target_link_libraries(drone_raspberrypi_pico_sdk
    pico_standard_link
    pico_mem_ops
    pico_bit_ops
    pico_int64_ops
    pico_divider
    pico_float
    pico_double
    pico_platform
    pico_multicore
    pico_bootrom
    hardware_regs
    hardware_clocks)
