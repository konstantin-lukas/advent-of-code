#include <stdio.h>
#include <assert.h>
#include <ctype.h>
#include <malloc.h>

int main() {
    FILE *fp = fopen("../input.txt", "r");
    assert(NULL != fp);

    char *array = NULL;
    unsigned long long int countArray[] = {0,0,0,0,0,0,0,0,0};
    while (!feof(fp)) {
        char ch = fgetc(fp);
        if (isdigit(ch)) {
            countArray[ch - 48]++;
        }
    }

    fclose(fp);

    for (int i = 0; i < 256; ++i) {
        unsigned long long int newSix = countArray[7] + countArray[0];
        unsigned long long int newEight = countArray[0];
        for (int j = 0; j < 9; ++j) {
            if (j != 6 && j != 8) countArray[j] = countArray[j + 1];
        }
        countArray[6] = newSix;
        countArray[8] = newEight;
    }

    unsigned long long int count = 0;
    for (int i = 0; i < 9; ++i) {
        count += countArray[i];
        printf("No. %d: %llu\n",i, countArray[i]);
    }
    printf("Count: %llu\n",count);
    return 0;
}
