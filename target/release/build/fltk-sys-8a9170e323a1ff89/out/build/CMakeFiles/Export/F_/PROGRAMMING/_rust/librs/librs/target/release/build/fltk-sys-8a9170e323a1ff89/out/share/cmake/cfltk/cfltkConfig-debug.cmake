#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "cfltk::cfltk" for configuration "Debug"
set_property(TARGET cfltk::cfltk APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(cfltk::cfltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_DEBUG "CXX"
  IMPORTED_LOCATION_DEBUG "F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/lib/cfltk.lib"
  )

list(APPEND _IMPORT_CHECK_TARGETS cfltk::cfltk )
list(APPEND _IMPORT_CHECK_FILES_FOR_cfltk::cfltk "F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/lib/cfltk.lib" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
