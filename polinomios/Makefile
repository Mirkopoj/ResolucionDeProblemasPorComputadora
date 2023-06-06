CC = g++
LDFLAGS = -lm
CFLAGS = -O -std=c++20 -I/usr/include/python3.11 -lcriterion -lpython3.11 -lginac
ifeq ($(DEBUG),1)
	CFLAGS += -g3
endif

MAINS = $(shell find src/mains/ -maxdepth 1 -type f -name "*.cpp")
SRCS = $(shell find src/ -maxdepth 1 -type f -name "*.cpp")
MAINOBJS = $(patsubst src/mains/%.cpp, obj/mains/%.o, $(MAINS))
MAINOUTS = $(patsubst src/mains/%.cpp, %, $(MAINS))
OBJS = $(patsubst src/%.cpp, obj/%.o, $(SRCS))

all: $(MAINOUTS)

$(MAINOUTS): $(OBJS) $(MAINOBJS)
	@mkdir -p bin
	$(CC) $(CFLAGS) -o bin/$@ obj/mains/$@.o $(OBJS) $(LDFLAGS)

obj/%.o: src/%.cpp 
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

obj/mains/%.o: src/mains/%.cpp 
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

.PHONY: clean

clean:
	rm -rf bin/ 
	rm -rf obj/
