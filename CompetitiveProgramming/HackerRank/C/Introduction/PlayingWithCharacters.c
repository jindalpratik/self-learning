#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() 
{
    char ch, s[100], scn[100];
    scanf("%c", &ch);
    printf("%c\n", ch);    
    scanf("%s", &s);
    printf("%s\n", s);   
    scanf(" %[^\n]%*c", scn);
    printf("%s", scn);
    return 0;
}
