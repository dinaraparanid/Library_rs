# Install script for directory: C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/png

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/Debug/fltk_pngd.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/Release/fltk_png.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/MinSizeRel/fltk_png.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/RelWithDebInfo/fltk_png.lib")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/FL/images" TYPE FILE FILES
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/png/png.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/png/pngconf.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/png/pnglibconf.h"
    )
endif()

