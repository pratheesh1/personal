# Define compiler and compiler flags
CC = gcc
CFLAGS = -Wall -g #-Werror

# Define source files and object files
SRC = chunk.c compiler.c debug.c main.c memory.c  scanner.c value.c vm.c
OBJ = $(patsubst %.c,bin/%.o,$(SRC))

# Define header files
HDR = chunk.h common.h compiler.h debug.h memory.h  scanner.h value.h vm.h

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