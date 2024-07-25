TARGET=target/debug
INC=/usr/local/include
LIB=/usr/local/lib

install:
	cp $(TARGET)/mtdynamic.h $(INC)
	cp $(TARGET)/libmtdynamic.a $(LIB)

uninstall:
	$(RM) $(INC)/mtdynamic.h
	$(RM) $(LIB)/libmtdynamic.a
