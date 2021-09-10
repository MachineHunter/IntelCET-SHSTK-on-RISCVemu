#include <stdio.h>

int main(void) {
	char hoge[20];
	printf("sample: starting gets\n");
	gets(hoge);
	printf("sample: ending gets\n");
	return 0;
}
