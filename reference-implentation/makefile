TARGET_EXEC := fitrefgen
PROFILE_SRC_DIR := /home/mstadelman/Development/FitSDKRelease_21.89.00/cpp
SRC_FILE := ./src/fit-ref-gen.cpp
BUILD_DIR := ./build
LIBFIT := $(BUILD_DIR)/lib/libfit.a

VPATH := $(PROFILE_SRC_DIR)


# Find all the C and C++ files we want to compile
# Note the single quotes around the * expressions. Make will incorrectly expand these otherwise.
LIB_SRCS := $(shell find $(PROFILE_SRC_DIR) -maxdepth 1 -name '*.cpp')

# String substitution for every C/C++ file.
# As an example, ./other_dir/hello.cpp turns into ./build/hello.cpp.o
LIB_OBJS := $(addprefix $(BUILD_DIR)/, $(notdir $(LIB_SRCS:.cpp=.o)))
SRC_OBJ := $(addprefix $(BUILD_DIR)/, $(notdir $(SRC_FILE:.cpp=.o)))

# Every folder in ./src will need to be passed to GCC so that it can find header files
INC_DIRS := $(PROFILE_SRC_DIR)

# Add a prefix to INC_DIRS. So moduleA would become -ImoduleA. GCC understands this -I flag
INC_FLAGS := $(addprefix -I,$(INC_DIRS))

# The -MMD and -MP flags together generate Makefiles for us!
# These files will have .d instead of .o as the output.
CPPFLAGS := -fpic $(INC_FLAGS)
LDFLAGS := -L$(BUILD_DIR)/lib -lfit


all: $(BUILD_DIR)/bin/$(TARGET_EXEC)

# build our EXE
$(BUILD_DIR)/bin/$(TARGET_EXEC): $(SRC_OBJ)
	mkdir -p $(dir $@)
	$(CXX) $< $(CPPFLAGS) $(CXXFLAGS) $(LDFLAGS) -o $@

# build src file
$(SRC_OBJ): $(SRC_FILE) $(LIBFIT)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c $< -o $@

# build fit library from profile
$(LIBFIT): $(LIB_OBJS)
	mkdir -p $(dir $@)
	ar crf $@ $^

# compile all of the fit profile srcs
$(BUILD_DIR)/%.o: %.cpp $(BUILD_DIR)
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c $< -o $@

$(BUILD_DIR):
	mkdir -p $@

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
