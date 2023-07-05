CC = g++ -std=c++2a
C_FLAGS = -c
OPT_FLAGS = -O3
DBG_FLAGS = -g -fsanitize=address -DDEBUG

APP = day5
BIN_DIR = ./bin
SOURCE_DIR = ./src

SOURCE = $(wildcard ./src/*.cpp)
OBJECTS = $(patsubst ./src/%.cpp, ./bin/%.o, $(SOURCE))
DEBUG_OBJECTS = $(patsubst ./src/%.cpp, ./bin/%_debug.o, $(SOURCE))

$(shell mkdir -p $(BIN_DIR))

all: $(BIN_DIR)/$(APP)

$(BIN_DIR)/$(APP): $(OBJECTS)
	$(CC) $(OPT_FLAGS) $^ -o $@

$(BIN_DIR)/%.o: $(SOURCE_DIR)/%.cpp
	$(CC) $(C_FLAGS) $(OPT_FLAGS) $< -o $@

debug: $(BIN_DIR)/$(APP)_debug

$(BIN_DIR)/$(APP)_debug: $(DEBUG_OBJECTS)
	$(CC) $(DBG_FLAGS) $^ -o $@

$(BIN_DIR)/%_debug.o: $(SOURCE_DIR)/%.cpp
	$(CC) $(C_FLAGS) $(DBG_FLAGS) $< -o $@

run: $(BIN_DIR)/$(APP)
	$(BIN_DIR)/$(APP)

run_debug: $(BIN_DIR)/$(APP)_debug
	$(BIN_DIR)/$(APP)_debug

watch: $(BIN_DIR)/$(APP)
	echo $(BIN_DIR)/$(APP) | entr $(MAKE) run

clean:
	rm -rf $(BIN_DIR)/*.o $(BIN_DIR)/$(APP)
