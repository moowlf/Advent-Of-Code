
# Compiler
CXX_COMPILER := g++

# Variables that hold information about building
OUTPUT_FOLDER 			:= ./output
OUTPUT_OBJECT_FOLDER 	:= ./output/objects
EXECUTABLE				:= problem
CURRENT_FOLDER			:= 2020

# Flags
CXX_FLAGS 			:= -std=c++20 -Wall -g -O0
LDFLAGS 			:= -L/usr/lib
INCLUDE_FOLDER 		:= -I./libs

# Object Generator
PROBLEM_SRC 	:= $(CURRENT_FOLDER)/main.cpp
OBJECT_FILES 	:= $(PROBLEM_SRC:$(CURRENT_FOLDER)/%.cpp=$(OUTPUT_OBJECT_FOLDER)/%.o)

# Rules
$(OUTPUT_OBJECT_FOLDER)/%.o: $(CURRENT_FOLDER)/%.cpp
	@mkdir -p $(@D)
	$(CXX_COMPILER) $(CXX_FLAGS) $(INCLUDE_FOLDER) -c $< -o $@ $(LDFLAGS)

$(EXECUTABLE): $(OBJECT_FILES)
	$(CXX_COMPILER) $(CXX_FLAGS) -o $(OUTPUT_FOLDER)/$(EXECUTABLE) $^ $(LDFLAGS)

all: build $(EXECUTABLE)

build: p
	@mkdir -p $(OUTPUT_FOLDER)
	@mkdir -p $(OUTPUT_OBJECT_FOLDER)
	@touch $(OUTPUT_FOLDER)/.gitkeep

clean:
	@rm -r $(OUTPUT_FOLDER)
