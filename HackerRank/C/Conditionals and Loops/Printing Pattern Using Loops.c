#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() 
{

    int n, k;
    scanf("%d", &n);
  	for(int i = 0;i <n; i++) { 
        k = n;
        for(int j = 0; j < n; j ++) { 
            if(k > n-i) { 
                printf("%d ",k);
                k--;
            }
            else { 
                printf("%d ",k);
            }
        }

        for(int j = 0; j < n - 1; j++) {
            if(k <= j+1) {
                k++;
                printf("%d ", k);
            }
            else {
                printf("%d ", k);
            }
        }
        printf("\n");
    }
  	for(int i = 0;i <n - 1; i++) { 
        k = n;
        for(int j = 0; j < n; j ++) { 
            if(k > i + 2) { 
                printf("%d ",k);
                k--;
            }
            else { 
                printf("%d ",k);
            }
        }

        for(int j = 0; j < n - 1; j++) {
            if(k <= j+1) {
                k++;
                printf("%d ", k);
            }
            else {
                printf("%d ", k);
            }
        }
        printf("\n");
    }
    return 0;
}