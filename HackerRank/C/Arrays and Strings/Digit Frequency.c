#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() {

    int arr[10] = {0,0,0,0,0,0,0,0,0,0};
    char str[100];
    gets(str);
    int slen = strlen(str);
    int i = 0;
    while(i < slen) {
        if(str[i] == '0') {
            arr[0] += 1;
        }
        if(str[i] == '1') {
            arr[1] += 1;
        }
        if(str[i] == '2') {
            arr[2] += 1;
        }
        if(str[i] == '3') {
            arr[3] += 1;
        }
        if(str[i] == '4') {
            arr[4] += 1;
        }
        if(str[i] == '5') {
            arr[5] += 1;
        }
        if(str[i] == '6') {
            arr[6] += 1;
        }
        if(str[i] == '7') {
            arr[7] += 1;
        }
        if(str[i] == '8') {
            arr[8] += 1;
        }
        if(str[i] == '9') {
            arr[9] += 1;
        }
        i++;
    }
    for(i = 0; i < 10; i++)
        printf("%d ", *(arr + i));
    return 0;
}
