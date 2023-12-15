#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() {

    char *s;
    s = malloc(1024 * sizeof(char));
    scanf("%[^\n]", s);
    s = realloc(s, strlen(s) + 1);
    int slen = strlen(s);
    char cur;
    for(int i = 0; i < slen; i++) {
        if(s[i] == ' ') {
            printf("\n");
        } else {
            cur = s[i];
            printf("%c",cur );
        }
    }
    return 0;
}