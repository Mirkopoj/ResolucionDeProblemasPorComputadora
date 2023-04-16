CC = gcc
LDFLAGS = -lm
CFLAGS = -O
ifeq ($(DEBUG),1)
	CFLAGS += -g3
endif

MAINS = $(shell find src/mains/ -maxdepth 1 -type f -name "*.c")
SRCS = $(shell find src/ -maxdepth 1 -type f -name "*.c")
MAINOBJS = $(patsubst src/mains/%.c, obj/mains/%.o, $(MAINS))
MAINOUTS = $(patsubst src/mains/%.c, %, $(MAINS))
OBJS = $(patsubst src/%.c, obj/%.o, $(SRCS))

all: $(MAINOUTS)

$(MAINOUTS): $(OBJS) $(MAINOBJS)
	@mkdir -p bin
	$(CC) $(CFLAGS) -o bin/$@ obj/mains/$@.o $(OBJS) $(LDFLAGS)

obj/%.o: src/%.c 
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

obj/mains/%.o: src/mains/%.c 
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

.PHONY: clean

clean:
	rm -rf bin/ 
	rm -rf obj/
