# Install script for directory: C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk

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
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/Debug/cfltk.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/Release/cfltk.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/MinSizeRel/cfltk.lib")
  elseif("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib/cfltk.lib")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/lib" TYPE STATIC_LIBRARY FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/RelWithDebInfo/cfltk.lib")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_box.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_browser.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_button.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_dialog.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_draw.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_enums.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_group.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_image.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_input.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_menu.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_misc.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_output.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_printer.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_surface.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_table.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_text.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_tree.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_valuator.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_widget.h;F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/include/cfltk" TYPE FILE FILES
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_box.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_browser.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_button.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_dialog.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_draw.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_enums.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_group.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_image.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_input.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_menu.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_misc.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_output.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_printer.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_surface.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_table.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_text.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_tree.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_valuator.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_widget.h"
    "C:/Users/Arseny/.cargo/git/checkouts/fltk-rs-9372f34fe2791521/656a240/fltk-sys/cfltk/include/cfl_window.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig.cmake"
         "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/CMakeFiles/Export/F_/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk" TYPE FILE FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/CMakeFiles/Export/F_/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-debug.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk" TYPE FILE FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/CMakeFiles/Export/F_/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-debug.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-minsizerel.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk" TYPE FILE FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/CMakeFiles/Export/F_/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-minsizerel.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-relwithdebinfo.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk" TYPE FILE FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/CMakeFiles/Export/F_/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-relwithdebinfo.cmake")
  endif()
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
        message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk" TYPE FILE FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/CMakeFiles/Export/F_/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfig-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/share/cmake/cfltk" TYPE FILE FILES "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/cfltkConfigVersion.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "F:/PROGRAMMING/_rust/librs/librs/target/debug/build/fltk-sys-1fd734dd3e003794/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
