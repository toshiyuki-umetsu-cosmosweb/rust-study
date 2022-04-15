SRCS = $(shell ls -1 *.rs)
#TARGET = $(notdir $(shell pwd))
TARGET = hello

.PHONY : all clean
all: 
	echo "rustcを使ってビルドする"
	rustc -o $(TARGET) $(SRCS)


clean:
	rm -f $(TARGET)

