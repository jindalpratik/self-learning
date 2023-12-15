#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() {
    int n;
    scanf("%d",&n);
    int arr[n];
    int i = 0, temp;
    while(i < n) {
        scanf("%d ", &temp);
        arr[i] = temp;
        i++;
    }
    
    i = 0;
    int sum = 0;
    while(i < n) {
        sum += arr[i];
        i++;
    }

    printf("%d", sum);
    
    return 0;
}