# Define compiler and compiler flags
CC = gcc
CFLAGS = -Wall -Werror -g

# Define source files and object files
SRC = chunk.c debug.c main.c memory.c value.c vm.c
OBJ = $(patsubst %.c,bin/%.o,$(SRC))

# Define header files
HDR = chunk.h common.h debug.h memory.h value.h vm.h

# Define executable file
EXEC = bin/clox

#create bin directory
$(shell mkdir -p bin)

# Define make rules
$(EXEC): $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $(EXEC)

bin/%.o: %.c $(HDR)
	$(CC) $(CFLAGS) -c $< -o $@

-include $(OBJ:.o=.d)

clean:
	rm -f bin/*