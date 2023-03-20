CC = gcc
LDFLAGS = -lm
CFLAGS = -O
ifeq ($(DEBUG),1)
	CFLAGS := -g3 $(CFLAGS)
endif

MAINS = $(shell find -type f -name "main*.c")
SRCS = $(shell find -type f -name "[!main]*.c")
MAINOBJS = $(patsubst ./%.c, obj/%.o, $(MAINS))
OBJS = $(patsubst ./%.c, obj/%.o, $(SRCS))

all: $(MAINS:.c=)

$(MAINS:.c=): $(OBJS) $(MAINOBJS)
	$(CC) $(CFLAGS) -o $@ obj/$@.o $(OBJS) $(LDFLAGS)

obj/%.o: %.c
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

.PHONY: clean

clean:
	rm -f $(MAINS:.c=)
	rm -rf obj/
