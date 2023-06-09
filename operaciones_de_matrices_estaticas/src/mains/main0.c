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
		}
		if(!strcmp(argv[i],"-o")){
			i++;
			if(i==argc){
				fprintf(stderr, "-o miss used, should be \"-o <PATH-TO-OUTPUT-FILE>\"\n");
				exit(-1);
			}
			fo = fopen(argv[i], "w+");
		}
		if(!strcmp(argv[i],"-p")){
			matrix_save_setpretty();
		}
	}

	int mat1[ROWS][COLS];
	int mat2[ROWS][COLS];

	if (fi==stdin) printf("Ingrese valores de la matriz 1\n"); 
	matrix_get(fi, mat1);
	if (fi==stdin) printf("Ingrese valores de la matriz 2\n"); 
	matrix_get(fi, mat2);

	if (fi==stdin) printf("Mat1:\n");
	matrix_save(fo, mat1);
	if (fi==stdin) printf("Mat2:\n");
	matrix_save(fo, mat2);

	int ret[ROWS][COLS];

	if (fi==stdin) printf("Mat1 + Mat2:\n");
	add(mat1, mat2, ret);
	matrix_save(fo, ret);

	if (fi==stdin) printf("[Mat1]^T:\n");
	transpose(mat1, ret);
	matrix_save(fo, ret);

	if (fi==stdin) printf("Mat1 * 5:\n");
	scalar_mult(mat1, 5, ret);
	matrix_save(fo, ret);

	if (fi==stdin) printf("Mat1 * Mat2:\n");
	matrix_mult(mat1, mat2, ret);
	matrix_save(fo, ret);

	return 0;
}
