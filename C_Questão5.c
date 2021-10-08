#include <stdio.h>
#include <stdlib.h>

int main(){
	int A,B,X;
	
	//OPERAÕES NAND
	A = 0;
	B = 0;
	X = !(!(A && B));
	printf("\nNOT(NOT(%d AND %d)): %d\n",A,B,X);
	
	A = 0;
	B = 1;
	X = !(!(A && B));
	printf("NOT(NOT(%d AND %d)): %d\n",A,B,X);
	
	A = 1;
	B = 0;
	X = !(!(A && B));
	printf("NOT(NOT(%d AND %d)): %d\n",A,B,X);
	
	A = 1;
	B = 1;
	X = !(!(A && B));
	printf("NOT(NOT(%d AND %d)): %d\n",A,B,X);
	return 0;
}
