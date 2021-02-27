if(NOT EXISTS "F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/build/fltk/install_manifest.txt")
   message(FATAL_ERROR
      "Cannot find install manifest: \"F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/build/fltk/install_manifest.txt\"")
endif(NOT EXISTS "F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/build/fltk/install_manifest.txt")

file(READ "F:/PROGRAMMING/_rust/librs/librs/target/release/build/fltk-sys-8a9170e323a1ff89/out/build/fltk/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")

foreach(file ${files})
message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
   exec_program("C:/Program Files (x86)/Microsoft Visual Studio/2019/Community/Common7/IDE/CommonExtensions/CMAKE/cmake-3.18.2-win64-x64/bin/cmake.exe"
      ARGS "-E remove -f \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
   )
   if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
   endif(NOT "${rm_retval}" STREQUAL 0)
endforeach(file)
