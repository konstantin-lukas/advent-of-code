#include <stdio.h>
#include <assert.h>
#include <ctype.h>
#include <malloc.h>

int main() {
    FILE *fp = fopen("../input.txt", "r");
    assert(NULL != fp);

    int length = 0;
    char *array = NULL;
    while (!feof(fp)) {
        char ch = fgetc(fp);
        if (isdigit(ch)) {
            length++;
            array = realloc(array, length * sizeof(char));
            array[length - 1] = (char)(ch - 48);
        }
    }

    fclose(fp);

    for (int j = 0; j < 80; ++j) {
        int tmpLength = length;
        for (int i = 0; i < tmpLength; i++) {
            if (array[i] - 1 < 0) {
                array[i] = 6;
                length++;
                array = realloc(array, length * sizeof(char));
                array[length - 1] = 8;
            } else {
                array[i]--;
            }
        }
    }

    printf("%d",length);
    return 0;
}
