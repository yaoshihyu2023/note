# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.26

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /home/shihyu/.mybin/cmake/bin/cmake

# The command to remove a file.
RM = /home/shihyu/.mybin/cmake/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp

# Include any dependencies generated for this target.
include CMakeFiles/cpptrader-tests.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/cpptrader-tests.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/cpptrader-tests.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/cpptrader-tests.dir/flags.make

CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o: CMakeFiles/cpptrader-tests.dir/flags.make
CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test.cpp
CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o: CMakeFiles/cpptrader-tests.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o -MF CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o.d -o CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test.cpp

CMakeFiles/cpptrader-tests.dir/tests/test.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader-tests.dir/tests/test.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test.cpp > CMakeFiles/cpptrader-tests.dir/tests/test.cpp.i

CMakeFiles/cpptrader-tests.dir/tests/test.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader-tests.dir/tests/test.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test.cpp -o CMakeFiles/cpptrader-tests.dir/tests/test.cpp.s

CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o: CMakeFiles/cpptrader-tests.dir/flags.make
CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_itch_handler.cpp
CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o: CMakeFiles/cpptrader-tests.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o -MF CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o.d -o CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_itch_handler.cpp

CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_itch_handler.cpp > CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.i

CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_itch_handler.cpp -o CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.s

CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o: CMakeFiles/cpptrader-tests.dir/flags.make
CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_market_manager.cpp
CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o: CMakeFiles/cpptrader-tests.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o -MF CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o.d -o CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_market_manager.cpp

CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_market_manager.cpp > CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.i

CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_market_manager.cpp -o CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.s

CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o: CMakeFiles/cpptrader-tests.dir/flags.make
CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_matching_engine.cpp
CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o: CMakeFiles/cpptrader-tests.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o -MF CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o.d -o CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_matching_engine.cpp

CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_matching_engine.cpp > CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.i

CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/tests/test_matching_engine.cpp -o CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.s

# Object files for target cpptrader-tests
cpptrader__tests_OBJECTS = \
"CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o" \
"CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o" \
"CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o" \
"CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o"

# External object files for target cpptrader-tests
cpptrader__tests_EXTERNAL_OBJECTS =

cpptrader-tests: CMakeFiles/cpptrader-tests.dir/tests/test.cpp.o
cpptrader-tests: CMakeFiles/cpptrader-tests.dir/tests/test_itch_handler.cpp.o
cpptrader-tests: CMakeFiles/cpptrader-tests.dir/tests/test_market_manager.cpp.o
cpptrader-tests: CMakeFiles/cpptrader-tests.dir/tests/test_matching_engine.cpp.o
cpptrader-tests: CMakeFiles/cpptrader-tests.dir/build.make
cpptrader-tests: modules/CppCommon/libcppcommon.a
cpptrader-tests: libcpptrader.a
cpptrader-tests: modules/Catch2/src/libCatch2Main.a
cpptrader-tests: func_tracker/libfunc_tracker.a
cpptrader-tests: modules/CppCommon/libcppcommon.a
cpptrader-tests: /usr/lib/x86_64-linux-gnu/libbfd.so
cpptrader-tests: /usr/lib/x86_64-linux-gnu/libdl.so
cpptrader-tests: /usr/lib/x86_64-linux-gnu/librt.so
cpptrader-tests: /usr/lib/x86_64-linux-gnu/libuuid.so
cpptrader-tests: modules/CppCommon/modules/libfmt.a
cpptrader-tests: func_tracker/libfunc_tracker.a
cpptrader-tests: modules/Catch2/src/libCatch2.a
cpptrader-tests: CMakeFiles/cpptrader-tests.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Linking CXX executable cpptrader-tests"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/cpptrader-tests.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/cpptrader-tests.dir/build: cpptrader-tests
.PHONY : CMakeFiles/cpptrader-tests.dir/build

CMakeFiles/cpptrader-tests.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/cpptrader-tests.dir/cmake_clean.cmake
.PHONY : CMakeFiles/cpptrader-tests.dir/clean

CMakeFiles/cpptrader-tests.dir/depend:
	cd /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles/cpptrader-tests.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/cpptrader-tests.dir/depend

