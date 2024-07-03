#include <stdio.h>
#include <time.h>
#include <stdlib.h>

int random_number()
{
    int possibilities[] = {1, 2, 3, 4, 5, 6};

    srand(time(NULL));

    int random_index = rand() % 6;

    return possibilities[random_index];
}
