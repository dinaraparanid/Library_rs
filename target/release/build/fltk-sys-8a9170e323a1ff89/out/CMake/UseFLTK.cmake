#
# UseFLTK.CMake - FLTK CMake environment configuration file for external projects.
#
# This file is deprecated and will be removed in FLTK 1.4.0 or later
#
# automatically generated - do not edit
#

include_directories("F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/build/fltk;C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk")

message(AUTHOR_WARNING
" * Warning: UseFLTK.cmake is deprecated and will be removed in FLTK 1.4.0
 * or later. Please use 'include_directories(\${FLTK_INCLUDE_DIRS})' or
 * 'target_include_directories(<target> PUBLIC|PRIVATE \${FLTK_INCLUDE_DIRS})'
 * instead of 'include(\${FLTK_USE_FILE})'.")
