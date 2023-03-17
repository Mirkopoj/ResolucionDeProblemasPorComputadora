CC = gcc
CFLAGS = 
ifeq ($(DEBUG),1)
	CFLAGS := -g3 $(CFLAGS)
endif
LDFLAGS = -lm

SRCS = $(shell find -type f -name "*.c")
OBJS = $(patsubst ./%.c, obj/%.o, $(SRCS))

TARGET = Test
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS) $(LDFLAGS)
obj/%.o: %.c
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@ $(LDFLAGS)

.PHONY: test clean

test: Test
	./Test

clean:
	rm -f Test
	rm -rf obj/
