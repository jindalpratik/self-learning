#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
// Complete the following function.

int calculate_the_maximum(int n, int k)
{
    int max_and = 0, max_or = 0, max_xor = 0, a = 1, b = 2, c = 2;
    while (a < n)
    {
        while (b <= n)
        {
            // printf("a = %d,b = %d\n",a,b);
            // printf("a&b = %d\n", a&b);
            if (((a & b) > max_and) && ((a & b) < k))
            {
                max_and = a & b;
            }
            // printf("a|b = %d\n", a|b);
            if (((a | b) > max_or) && ((a | b) < k))
            {
                max_or = a | b;
            }
            // printf("a^b = %d\n", a^b);
            if (((a ^ b) > max_xor) && ((a ^ b) < k))
            {
                
                max_xor = a ^ b;
            }
            // printf("%d\n%d\n%d\n",max_and,max_or,max_xor);
            b += 1;
        }
        a += 1;
        b = c + 1;
        c += 1;
    }
    printf("%d\n%d\n%d",max_and,max_or,max_xor);
    return 0;
}

int main()
{
    int n, k;
    scanf("%d %d", &n, &k);
    calculate_the_maximum(n, k);
    return 0;
}
