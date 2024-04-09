#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

int main(int argc, char *argv[]) {

	printf("hello world!\n");
	// struct timeval begin, end;
	// gettimeofday(&begin, NULL);
	// int** x = malloc(1000 * sizeof(int*));
	// for (int i = 0; i < 1000; ++i) {
	// 	x[i] = malloc(sizeof(int) * 1000);
	// 	if (x[i] == NULL) {
	// 		printf("error allocating");
	// 		return 1;
	// 	}
	// }

	// // for (int i = 0; i < 100000; ++i) {
	// // 	free(x[i]);
	// // }
	// // free(x);
	// gettimeofday(&end, NULL);
	// double elapsed = (end.tv_sec - begin.tv_sec) * 1000.0 + (end.tv_usec - begin.tv_usec) / 1000.0;

	// printf("time taken: %lf \n", elapsed);
	return 0;
}
