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

	matrix mat1;
	matrix mat2;

	if (fi==stdin) printf("Ingrese valores de la matriz 1\n"); 
	matrix_get(fi, &mat1);
	if (fi==stdin) printf("Ingrese valores de la matriz 2\n"); 
	matrix_get(fi, &mat2);
	if (fi!=stdin) fclose(fi);

	if (fo==stdout) printf("Mat1:\n");
	if (fo==stdout) printf("%dx%d: %d alocados, %d ocupados\n", rows(mat1), cols(mat1), element_capacity(mat1), element_count(mat1));
	matrix_save(fo, mat1);
	if (fo==stdout) printf("Mat2:\n");
	if (fo==stdout) printf("%dx%d: %d alocados, %d ocupados\n", rows(mat2), cols(mat2), element_capacity(mat2), element_count(mat2));
	matrix_save(fo, mat2);

	matrix ret;
	ret.rows = mat1.rows;
	ret.cols = mat1.cols;
	matrix_alloc(&ret);

	if (fo==stdout) printf("Mat1 + Mat2:\n");
	add(mat1, mat2, &ret);
	matrix_save(fo, ret);

	if (fo==stdout) printf("[Mat1]^T:\n");
	transpose(mat1, &ret);
	matrix_save(fo, ret);

	if (fo==stdout) printf("Mat1 * 5:\n");
	scalar_mult(mat1, 5, &ret);
	matrix_save(fo, ret);

	matrix_free(&mat1);
	matrix_free(&mat2);
	matrix_free(&ret);
	if (fo!=stdout) fclose(fo);
	return 0;
}
