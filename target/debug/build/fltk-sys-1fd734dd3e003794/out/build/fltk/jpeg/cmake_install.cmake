# Install script for directory: C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/jpeg

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
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/Debug/fltk_jpegd.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/Release/fltk_jpeg.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/MinSizeRel/fltk_jpeg.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/lib/RelWithDebInfo/fltk_jpeg.lib")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/FL/images" TYPE FILE FILES
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/jpeg/jconfig.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/jpeg/jerror.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/jpeg/jmorecfg.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/fltk/jpeg/jpeglib.h"
    )
endif()
