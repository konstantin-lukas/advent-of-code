#include <stdio.h>
#include <assert.h>
#include <ctype.h>
#include <malloc.h>

int main() {
    FILE *fp = fopen("../input.txt", "r");
    assert(NULL != fp);

    int length = 0;
    int *array = NULL;
    int max = 0;
    while (!feof(fp)) {
        int number;
        length++;
        fscanf(fp, "%d,", &number);
        array = realloc(array, sizeof(int) * length);
        array[length - 1] = number;
        if (number > max) max = number;
    }

    fclose(fp);

    int minFuel = -1;
    int minPosition = 0;
    for (int i = 0; i <= max; ++i) {
        int fuel = 0;
        for (int j = 0; j < length; ++j) {
            if (fuel > minFuel && minFuel > -1) break;
            fuel += abs(array[j] - i);
        }
        if (minFuel < 0 || 0 <= minFuel && minFuel > fuel) {
            minFuel = fuel;
            minPosition = i;
        }
    }
    printf("Minimum fuel: %d at position %d",minFuel,minPosition);
    return 0;
}
