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
include CMakeFiles/cpptrader.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/cpptrader.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/cpptrader.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/cpptrader.dir/flags.make

CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o: CMakeFiles/cpptrader.dir/flags.make
CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/market_manager.cpp
CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o: CMakeFiles/cpptrader.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o -MF CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o.d -o CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/market_manager.cpp

CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/market_manager.cpp > CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.i

CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/market_manager.cpp -o CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.s

CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o: CMakeFiles/cpptrader.dir/flags.make
CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order.cpp
CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o: CMakeFiles/cpptrader.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o -MF CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o.d -o CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order.cpp

CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order.cpp > CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.i

CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order.cpp -o CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.s

CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o: CMakeFiles/cpptrader.dir/flags.make
CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order_book.cpp
CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o: CMakeFiles/cpptrader.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o -MF CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o.d -o CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order_book.cpp

CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order_book.cpp > CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.i

CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/matching/order_book.cpp -o CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.s

CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o: CMakeFiles/cpptrader.dir/flags.make
CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o: /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/providers/nasdaq/itch_handler.cpp
CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o: CMakeFiles/cpptrader.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o -MF CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o.d -o CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o -c /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/providers/nasdaq/itch_handler.cpp

CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/providers/nasdaq/itch_handler.cpp > CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.i

CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/source/trader/providers/nasdaq/itch_handler.cpp -o CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.s

# Object files for target cpptrader
cpptrader_OBJECTS = \
"CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o" \
"CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o" \
"CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o" \
"CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o"

# External object files for target cpptrader
cpptrader_EXTERNAL_OBJECTS =

libcpptrader.a: CMakeFiles/cpptrader.dir/source/trader/matching/market_manager.cpp.o
libcpptrader.a: CMakeFiles/cpptrader.dir/source/trader/matching/order.cpp.o
libcpptrader.a: CMakeFiles/cpptrader.dir/source/trader/matching/order_book.cpp.o
libcpptrader.a: CMakeFiles/cpptrader.dir/source/trader/providers/nasdaq/itch_handler.cpp.o
libcpptrader.a: CMakeFiles/cpptrader.dir/build.make
libcpptrader.a: CMakeFiles/cpptrader.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Linking CXX static library libcpptrader.a"
	$(CMAKE_COMMAND) -P CMakeFiles/cpptrader.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/cpptrader.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/cpptrader.dir/build: libcpptrader.a
.PHONY : CMakeFiles/cpptrader.dir/build

CMakeFiles/cpptrader.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/cpptrader.dir/cmake_clean.cmake
.PHONY : CMakeFiles/cpptrader.dir/clean

CMakeFiles/cpptrader.dir/depend:
	cd /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp /media/shihyu/ssd1/github/jason_note/src/c++/data/CppTrader/CppTrader_modify/temp/CMakeFiles/cpptrader.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/cpptrader.dir/depend

