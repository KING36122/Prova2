#include <stdio.h>
#include <stdlib.h>

int main(){
	int A,B,X;
	
	//OPERAÇÕES NOT
	A = 1;
	X = !(!A);
	printf("NOT(NOT %d): %d\n",A,X);
	
	A = 0;
	X = !(!A);
	printf("NOT(NOT %d): %d\n",A,X);
	
	
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
	
	//OPERAÇÕES NOR
	A = 0;
	B = 0;
	X = !(!(A || B));
	printf("\nNOT(NOT(%d OR %d)): %d\n",A,B,X);
	
	A = 0;
	B = 1;
	X = !(!(A || B));
	printf("NOT(NOT(%d OR %d)): %d\n",A,B,X);
	
	A = 1;
	B = 0;
	X = !(!(A || B));
	printf("NOT(NOT(%d OR %d)): %d\n",A,B,X);
	
	A = 1;
	B = 1;
	X = !(!(A || B));
	printf("NOT(NOT(%d OR %d)): %d\n",A,B,X);
	
	//OPERAÇÕES AND
	A = 0;
	B = 0;
	X = !(A && B);
	printf("\nNOT(%d AND %d): %d\n",A,B,X);
	
	A = 0;
	B = 1;
	X = !(A && B);
	printf("NOT(%d AND %d): %d\n",A,B,X);
	
	A = 1;
	B = 0;
	X = !(A && B);
	printf("NOT(%d AND %d): %d\n",A,B,X);
	
	A = 1;
	B = 1;
	X = !(A && B);
	printf("NOT(%d AND %d): %d\n",A,B,X);
	
	//OPERAÇÕES OR
	A = 0;
	B = 0;
	X = !(A || B);
	printf("\nNOT(%d OR %d): %d\n",A,B,X);
	
	A = 0;
	B = 1;
	X = !(A || B);
	printf("NOT(%d OR %d): %d\n",A,B,X);
	
	A = 1;
	B = 0;
	X = !(A || B);
	printf("NOT(%d OR %d): %d\n",A,B,X);
	
	A = 1;
	B = 1;
	X = !(A || B);
	printf("NOT(%d OR %d): %d\n",A,B,X);
	return 0;
}
