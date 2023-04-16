#include "../../Include/operaciones.h"
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

extern int DEBUG;

int main(int argc, char *argv[], char *envp[]) {
	DEBUG = 1;
	FILE *fi = stdin;
	FILE *fo = stdout;
	for (int i=1; i<argc; i++) {
		if(!strcmp(argv[i],"-i")){
			i++;
			if(i==argc){
				fprintf(stderr, "-i miss used, should be \"-i <PATH-TO-INPUT-FILE>\"\n");
				exit(-1);
			}
			fi = fopen(argv[i], "r");
			if (!fi) {
				fprintf(stderr, "fi: file failed to open");
			}
		}
		if(!strcmp(argv[i],"-o")){
			i++;
			if(i==argc){
				fprintf(stderr, "-o miss used, should be \"-o <PATH-TO-OUTPUT-FILE>\"\n");
				exit(-1);
			}
			fo = fopen(argv[i], "w+");
			if (!fo) {
				fprintf(stderr, "fo: file failed to open");
			}
		}
		if(!strcmp(argv[i],"-p")){
			matrix_save_setpretty();
		}
	}

	matrix mat1;
	matrix mat2;

	if (fi==stdin) printf("Ingrese valores de la matriz 1\n"); 
	matrix_get(fi, &mat1);
	if (fi!=stdin) fclose(fi);

	//if (fo==stdout) printf("Mat1:\n");
	if (fo==stdout) printf("%dx%d: %d alocados, %d ocupados\n", rows(mat1), cols(mat1), element_capacity(mat1), element_count(mat1));
	//matrix_save(fo, mat1);

	matrix_free(&mat1);
	if (fo!=stdout) fclose(fo);
	return 0;
}
