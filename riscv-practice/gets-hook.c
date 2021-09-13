#include <unistd.h>
#include <dlfcn.h>
#include <stdio.h>
#include <string.h>

void *g_lib = NULL;
char (*g_gets)(char*) = NULL;

// 最初に呼ばれる
__attribute__((constructor)) void check_hook_loaded(void) {
	puts("preload success");
}

// write関数が呼び出されると呼ばれる
char gets(char *s) {
	if(!g_lib || !g_gets){
		g_lib = dlopen("libc.so.6", RTLD_LAZY);
		g_gets = dlsym(g_lib, "gets");
	}
	char res = (*g_gets)(s);
	int i;
	for(i=0; i<8; i++) {
		if(s[i]=='\n')
			return res;
	}
	for(i=0; ; i++) {
		char buf[8];
		memcpy(buf, &s[i], 8);
		unsigned long buf2 = *(unsigned long*)buf;
		printf("%d : %lx\n", i, buf2);
		if(s[i+8]!='\n')
			break;
		//if(buf2>=0x000000000000 && buf2<=0x7fffffffffff) {
		//	printf("%lx\n", buf2);
		//}
	}
	return res;
}
