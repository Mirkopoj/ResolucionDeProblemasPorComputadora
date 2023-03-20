CC = gcc
CFLAGS = 
ifeq ($(DEBUG),1)
	CFLAGS := -g3 $(CFLAGS)
endif
LDFLAGS = -lm

MAINS = $(shell find -type f -name "main*.c")
SRCS = $(shell find -type f -name "*.c")
objects = $(patsubst ./%.c, obj/%.o, $(SRCS))
OBJS = $(filter-out $(MAINS),$(objects))

TARGET = main

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS) $(LDFLAGS)
obj/%.o: %.c
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@ $(LDFLAGS)

.PHONY: test clean

test: $(TARGET)
	./Test

main1: TARGET = main1, $(TARGET)

clean:
	rm -f Test
	rm -rf obj/
