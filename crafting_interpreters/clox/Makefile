# Define compiler and compiler flags
CC = gcc
CFLAGS = -Wall -g #-Werror

# Define source files and object files
SRC = $(wildcard *.c)
OBJ = $(patsubst %.c,bin/%.o,$(SRC))

# Define header files
HDR = $(wildcard *.h)

# Define executable file
EXEC = bin/clox

#create bin directory
$(shell mkdir -p bin)

# Define make rules
$(EXEC): $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $(EXEC) && rm $(OBJ)

bin/%.o: %.c $(HDR)
	$(CC) $(CFLAGS) -c $< -o $@

-include $(OBJ:.o=.d)

clean:
	rm -f bin/*

expand:
	$(CC) $(CFLAGS) -E $(SRC) > $(EXEC).source.c

run:
	./$(EXEC)
