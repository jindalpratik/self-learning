#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() {
	
    int n, sum = 0;
    scanf("%d", &n);
    for(int new_n = n; new_n != 0; new_n/=10)
    {
        sum += (new_n%10);
    }
    printf("%d",sum);
    return 0;
}