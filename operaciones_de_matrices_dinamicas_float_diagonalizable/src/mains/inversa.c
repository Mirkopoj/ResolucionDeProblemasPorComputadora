#include "../../Include/operaciones.h"
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

int main(int argc, char *argv[], char *envp[]) {
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

	matrix mat;

	if (fi==stdin) printf("Ingrese valores de la matriz 1\n"); 
	matrix_get(fi, &mat);
	if (fi!=stdin) fclose(fi);

	if (fo==stdout) printf("Mat1:\n");
	matrix_save(fo, mat);

	matrix ret;
	ret.rows = mat.rows;
	ret.cols = mat.cols;
	matrix_alloc(&ret);

	matrix_inv(mat, &ret);
	matrix_save(stdout, ret);

	matrix_free(&mat);
	matrix_free(&ret);
	if (fo!=stdout) fclose(fo);
	return 0;
}
