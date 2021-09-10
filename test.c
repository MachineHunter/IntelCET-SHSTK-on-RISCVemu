#include <stdio.h>
#include <string.h>

int main() {
	char hoge[100];
	gets(hoge);

	// puts(hoge);
	// printf("1Byte: %c\n", ((int)*hoge>>0)&0xFF);
	// printf("length: %d\n", strlen(hoge));

	if(hoge[2]=='\n') puts("bingo");
	char buf[8];
	memcpy(buf, &hoge[0], 8);
	unsigned long fuga = *(unsigned long*)buf;
	printf("%lx\n", fuga);
	return 0;
}
