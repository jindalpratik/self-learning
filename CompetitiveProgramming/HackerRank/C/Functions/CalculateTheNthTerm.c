#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
//Complete the following function.

int find_nth_term(int n, int a, int b, int c) {
    int sum;
    if(n == 1){
        return a;
    } else if(n == 2){
        return b;
    } else if(n == 3){
        return c;
    } else{
        for(int i = 4; i <= n; i++){
            sum = a + b + c;
            a = b;
            b = c;
            c = sum;
        }
    }
    return sum;
}

int main() {
    int n, a, b, c;
    int one_digit = 4;
  
    scanf("%d %d %d %d", &n, &a, &b, &c);
    int ans = find_nth_term(n, a, b, c);
 
    printf("%d", ans); 
    return 0;
}