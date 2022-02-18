#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HAINUM 14
#define ALLCASES                                                               \
    case B1 ... B7:                                                            \
    case C1 ... C7:                                                            \
    case D1 ... D7

static unsigned long long combination_count = 0;
static unsigned long long agari_count = 0;
static unsigned long long pattern_count = 0;
static unsigned long long agari_pattern_count = 0;
FILE *output_file = NULL;

typedef enum Tile {
    Red,
    Green,
    White,
    East,
    South,
    West,
    North,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    _tile_end
} Tile;

void hailoop(Tile[], int, int);
int is_agari(Tile[]);
int check_hai(Tile[], int);

int main(int argc, char *argv[]) {
    Tile hai[HAINUM];
    char *fname = "patterns_general_four.dat";

    // initialization
    for (int i = 0; i < HAINUM; ++i)
        hai[i] = Red;
    if (argc > 1)
        fname = argv[1];
    output_file = fopen(fname, "wb");
    if (output_file == NULL)
        exit(1);

    // loop to end
    hailoop(hai, Red, 0);

    printf("total combination: %llu\n", combination_count);
    printf("total agari combination: %llu\n", agari_count);
    printf("total pattern: %llu\n", pattern_count);
    printf("total agari pattern: %llu\n", agari_pattern_count);

    fclose(output_file);

    return 0;
}

void hailoop(Tile hai[], int n, int layer) {
    if (layer == HAINUM) {
        /* execution */
        int tmp_count = 1;
        int tile_count[HAINUM] = {1};
        int tile_index = 0;
        for (int i = 1; i < HAINUM; ++i) {
            if (hai[i] == hai[i - 1]) {
                ++tile_count[tile_index];
            } else {
                ++tile_index;
                ++tile_count[tile_index];
            }
        }
        for (int i = 0; i < HAINUM; ++i) {
            switch (tile_count[i]) {
            case 0:
            case 4:
                tile_count[i] = 1;
                break;
            case 1:
            case 3:
                tile_count[i] = 4;
                break;
            case 2:
                tile_count[i] = 6;
                break;
            default:
                tile_count[i] = 0;
            }
            tmp_count *= tile_count[i];
        }
        combination_count += tmp_count;
        ++pattern_count;
        if (is_agari(hai)) {
            ++agari_pattern_count;
            agari_count += tmp_count;
        }
        return;
    } else {
        if (layer >= 4 && hai[layer - 4] == (Tile)n)
            ++n;
        for (int a = n; a < _tile_end; ++a) {
            hai[layer] = a;
            hailoop(hai, a, layer + 1);
        }
    }
}

int is_agari(Tile hai[]) {
    // copy for use
    int sum = 0;
    for (int i = 0; i < HAINUM; ++i) {
        sum += hai[i];
    }

    // initial step: get pair
    int flag[HAINUM - 1] = {0};
    int tot_flag = 0;
    for (int i = 1; i < HAINUM; ++i) {
        if (hai[i] == hai[i - 1])
            flag[i - 1] = 1;
        tot_flag += flag[i - 1];
    }
    if (tot_flag == 0)
        return 0;

    // second step: check sum despite pair, should mod 3 = 0
    tot_flag = 0;
    for (int i = 0; i < HAINUM - 1; ++i) {
        if (flag[i]) {
            int t = sum - 2 * hai[i];
            flag[i] = !(t % 3);
        }
        tot_flag += flag[i];
    }
    if (tot_flag == 0)
        return 0;

    // third step: check for triples
    tot_flag = 0;
    for (int i = 0; i < HAINUM - 1; ++i) {
        if (flag[i]) {
            Tile cphai[HAINUM - 2];
            for (int j = 0, k = 0; j < HAINUM; ++j, ++k) {
                if (j == i) {
                    ++j;
                    --k;
                    continue;
                }
                cphai[k] = hai[j];
            }
            flag[i] = check_hai(cphai, HAINUM - 2);
        }
        tot_flag += flag[i];
    }
    if (tot_flag != 0) {
        char agari_pattern[HAINUM] = {'\0'};
        for (int i = 0; i < HAINUM; ++i) {
            agari_pattern[i] = hai[i] + 'A';
        }
        fwrite(agari_pattern, 1, HAINUM, output_file);
    }

    return (tot_flag != 0);
}

int check_hai(Tile hai[], int n) {
    int _is_chow(const Tile *);
    int _is_pung(const Tile *);
    int _find_next_tile_index(const Tile *, int);
    if (!n)
        return 1;

    if ((n % 3)) {
        fprintf(stderr, "what the hell is %d?\n", n);
        return 0;
    }

    int used_index[HAINUM] = {0};
    for (int i = 0; i < HAINUM - 2;) {
        if (used_index[i]) {
            ++i;
            continue;
        }

        if ((_is_chow(hai + i) || _is_pung(hai + i)) && !(used_index[i + 1]) &&
            !(used_index[i + 2])) {
            if (_is_chow(hai + i)) {
                switch (hai[i]) {
                ALLCASES:
                    break;
                default:
                    return 0;
                }
            }
            used_index[i] = used_index[i + 1] = used_index[i + 2] = 1;
            i += 3;
            continue;
        }

        int next_index = -1, last_index = -1;
        next_index = i + _find_next_tile_index(hai + i, HAINUM - i - 2);
        if (next_index == -1)
            return 0;
        while (used_index[next_index])
            ++next_index;
        last_index =
            next_index +
            _find_next_tile_index(hai + next_index, HAINUM - next_index - 2);
        if (last_index == -1)
            return 0;
        while (used_index[last_index])
            ++last_index;
        if (hai[next_index] == hai[i] + 1 &&
            hai[last_index] == hai[next_index] + 1) {
            switch (hai[i]) {
            ALLCASES:
                break;
            default:
                return 0;
            }
            used_index[i] = used_index[next_index] = used_index[last_index] = 1;
        } else
            return 0;
    }

    return 1;
}

// const 3-tile set
int _is_chow(const Tile *hai) {
    if (hai[1] == hai[0] + 1 && hai[2] == hai[1] + 1) {
        switch (hai[0]) {
        ALLCASES:
            return 1;
        default:
            return 0;
        }
    }
    return 0;
}

// const 3-tile set
int _is_pung(const Tile *hai) { return (hai[1] == hai[0] && hai[2] == hai[1]); }

int _find_next_tile_index(const Tile *hai, int n) {
    Tile c = hai[0];
    for (int i = 1; i < n; ++i)
        if (hai[i] != c)
            return i;
    return -1;
}
