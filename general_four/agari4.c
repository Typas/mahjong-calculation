#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HAINUM 14
#define SETNUM (HAINUM/3)
#define TILE_PER_COLOR 9
#define BAMBOO_LAST B9
#define CHARACTER_LAST C9
#define DOTS_LAST D9
#define WORDNUM 7
#define FLUSHMAGIC (TILE_PER_COLOR-WORDNUM)

#define CHOWHEAD case B1...B7: case C1...C7: case D1...D7

#define TRIPLEFUNC(var, func, hai, boolarr)                     \
    var |= func(hai, boolarr, 2, 5, 8);                         \
    var |= func(hai, boolarr, 2, 5, 11);                        \
    var |= func(hai, boolarr, 2, 8, 11);                        \
    var |= func(hai, boolarr, 5, 8, 11)

#define DUOFUNC(var, func, hai, boolarr)                        \
    var |= func(hai, boolarr, 2, 5);                            \
    var |= func(hai, boolarr, 2, 8);                            \
    var |= func(hai, boolarr, 2, 11);                           \
    var |= func(hai, boolarr, 5, 8);                            \
    var |= func(hai, boolarr, 5, 11);                           \
    var |= func(hai, boolarr, 8, 11)

typedef enum Tile{
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

typedef enum Hands {
    Not,                // 非此役
    PinHu,              // 平和
    Dragon,             // 番牌
    Simple,             // 斷么
    Straight,           // 一氣通貫
    MixWithTerminal,    // 混全帶
    PureWithTerminal,   // 純全帶
    MixTerminal,        // 混老頭
    SameChow,           // 一般高
    DoubleSameChow,     // 二般高
    MixTripleChow,      // 三色同順
    TriplePung,         // 三色同刻
    AllPungs,           // 對對和
    HalfFlush,          // 混一色
    FullFlush,          // 清一色
    ThreeStepPung,      // 三連刻
    FourStepPung,       // 四連刻
    LittleThreeDragons, // 小三元
    BigThreeDragons,    // 大三元
    LittleFourWinds,    // 小四喜
    BigFourWinds,       // 大四喜
    AllHonours,         // 字一色
    PureTerminal,       // 清老頭
    SameQuadroChow,     // 一色四同順
    _hands_end
} Hands;

unsigned patterns[_hands_end] = {0};
unsigned long long combinations[_hands_end] = {0};
unsigned long long scores[_hands_end] = {0};
unsigned long long total_score = 0;

void type_check(Tile*);
unsigned score(Tile*, int*, unsigned);
const char* type_name(Hands);

/* 平和 */
Hands _pin_hu(Tile*);
/* 番牌 */
Hands _dragon(Tile*, int*);
/* 斷么 */
Hands _simple(Tile*);
/* 一氣通貫 */
Hands _straight(Tile*);
/* 么九類 */
Hands _terminal(Tile*);
/* 同順類 */
Hands _same_chow(Tile*);
/* 三色類 */
Hands _mix_triple(Tile*);
/* 對對和 */
Hands _all_pungs(Tile*);
/* 一色類 */
Hands _flush(Tile*);
/* 字牌類 */
Hands _honours(Tile*);
/* 連刻類 */
Hands _step_pungs(Tile*);

// 輔助函式
unsigned long long _calc_combination(const Tile*);
int _is_valid_hai(Tile*, int);
int _is_chow(Tile*);
int _is_pung(Tile*);
int _has_terminal(Tile*);
int _is_simple(Tile);
int _is_terminal(Tile);
int _is_honour(Tile);
int _all_same(int (*fn)(Tile*), Tile*);

int main(int argc, char *argv[])
{
    Tile hai[HAINUM];
    char tmp[HAINUM];
    int counter = 0;
    char *fname = "patterns_general_four.dat";
    FILE *fp = NULL;
    if(argc > 1)
        fname = argv[1];
    fp = fopen(fname, "rb");
    if(fp == NULL)
        exit(1);

    while(fread(tmp, 1, HAINUM, fp)) {
        for(int i=0; i<HAINUM; ++i)
            hai[i] = (Tile)(tmp[i] - 'A');
        type_check(hai);
        ++counter;
    }

    unsigned long long total_combination = 0;
    for(Hands i = Not; i < _hands_end; ++i) {
        total_combination += combinations[i];
    }
    printf("平均分值：%.3lf\n", (double)total_score/(double)total_combination);
    printf("役種\t和牌形\t組合數\t平均分數\n");
    for(Hands iter = Not; iter < _hands_end; ++iter)
        printf("%s\t%u\t%llu\t%.3lf\n", type_name(iter), patterns[iter], combinations[iter], (double)scores[iter]/combinations[iter]);
    printf("%d\n", counter);

    return 0;
}

// DONE: 拆牌確認
void type_check(Tile *hai)
{
    // get pair, check validity
    // if valid, go fn score to calculate score
    // 高點法只計不同雀頭
    int flag[HAINUM] = {0}; // turn on if same as before
    for(int i = 0; i < HAINUM-1; ++i)
        if(hai[i] == hai[i+1])
            flag[i] = 1;
    int hand_check[_hands_end] = {0};

    unsigned long long hai_score = 0;
    int _find_next_tile_index(const Tile*, int);
    void _tile_swap(Tile*, Tile*);
    void _hai_copy(Tile*, const Tile*, int);

    for(int i = 0; i < HAINUM-1; ++i) {
        if(!flag[i])
            continue;

        // hai variant
        Tile varhai[HAINUM];
        varhai[0] = varhai[1] = hai[i];
        for(int j = 0, k = 2; j < HAINUM; ++j, ++k) {
            if(j == i) {
                ++j;
                --k;
                continue;
            }
            varhai[k] = hai[j];
        }

        // check validity, then go score
        if(_is_valid_hai(varhai+2, HAINUM-2)) {
            // rearrange
            if(_all_pungs(varhai) == Not) {
                Tile subhai[SETNUM][3] = {{Red}};
                int used_index[HAINUM] = {0};
                used_index[0] = used_index[1] = 1;
                int k = 0;
                for(int j = 2; j < HAINUM; ) {
                    if(used_index[j]) {
                        ++j;
                        continue;
                    }

                    if((_is_chow(varhai+j) || _is_pung(varhai+j)) && !(used_index[j+1]) && !(used_index[j+2])){
                        _hai_copy(subhai[k], varhai+j, 3);
                        used_index[j] = used_index[j+1] = used_index[j+2] = 1;
                        j+=3;
                        ++k;
                        continue;
                    }

                    int next_index = -1, last_index = -1;
                    next_index = j+_find_next_tile_index(varhai+j, HAINUM-j);
                    while(used_index[next_index])
                        ++next_index;
                    last_index = next_index+_find_next_tile_index(varhai+next_index, HAINUM-next_index);
                    while(used_index[last_index])
                        ++last_index;
                    used_index[j] = used_index[next_index] = used_index[last_index] = 1;
                    subhai[k][0] = varhai[j];
                    subhai[k][1] = varhai[next_index];
                    subhai[k][2] = varhai[last_index];
                    ++k;
                }
                for(int j = 0; j < SETNUM; ++j)
                    _hai_copy(varhai+2+3*j, subhai[j], 3);
            }

            hai_score = score(varhai, hand_check, hai_score);
        }
    }

    if(hai_score > 0)
        total_score += hai_score * _calc_combination(hai);
    else {
        ++patterns[Not];
        combinations[Not] += _calc_combination(hai);
    }
}

int _find_next_tile_index(const Tile* hai, int a)
{
    Tile c = hai[0];
    for(int i = 1; i < a; ++i)
        if(hai[i] != c)
            return i;

    return -1;
}

void _tile_swap(Tile* a, Tile* b)
{
    Tile tmp = *a;
    *a = *b;
    *b = tmp;
}

void _hai_copy(Tile* t, const Tile* c, int n)
{
    for(int i=0; i<n; ++i)
        t[i] = c[i];
}

#define SCORE_CHECK(var, arr, func, arg) \
    var = func(arg);                     \
    arr[var] = 1

// DONE: 計算分數組合
unsigned score(Tile *hai, int *old_check, unsigned current_score)
{
    int hand_check[_hands_end] = {0};
    int n_dragon = 0;
    Hands i = Not;
    SCORE_CHECK(i, hand_check, _pin_hu, hai);
    SCORE_CHECK(i, hand_check, _simple, hai);
    SCORE_CHECK(i, hand_check, _straight, hai);
    SCORE_CHECK(i, hand_check, _terminal, hai);
    SCORE_CHECK(i, hand_check, _same_chow, hai);
    SCORE_CHECK(i, hand_check, _mix_triple, hai);
    SCORE_CHECK(i, hand_check, _all_pungs, hai);
    SCORE_CHECK(i, hand_check, _flush, hai);
    SCORE_CHECK(i, hand_check, _honours, hai);
    SCORE_CHECK(i, hand_check, _step_pungs, hai);
    i = _dragon(hai, &n_dragon);
    hand_check[i] = 1;

    unsigned long long comb = _calc_combination(hai);
    // 役滿
    if(hand_check[PureTerminal] | hand_check[BigFourWinds] || hand_check[SameQuadroChow] | hand_check[AllHonours]) {
        // 只計役滿
        for(Hands i = BigFourWinds; i < _hands_end; ++i) {
            patterns[i] += hand_check[i];
            combinations[i] += hand_check[i]*comb;
            scores[i] += ((i==AllHonours)?320:(i==BigFourWinds||i==PureTerminal)?400:480)*comb*hand_check[i] - current_score*comb*old_check[i];
        }
        for(Hands i = PinHu; i < BigFourWinds; ++i) {
            patterns[i] -= old_check[i];
            combinations[i] -= old_check[i];
            scores[i] -= current_score*comb*old_check[i];
            old_check[i] = 0;
        }
        // 回傳最大值
        if(hand_check[SameQuadroChow])
            return 480;
        if(hand_check[BigFourWinds] | hand_check[PureTerminal])
            return 400;
        else
            return 320;
    }

    // 非役滿
    unsigned result = 0;

    if(hand_check[PinHu])
        result += 5;

    if(hand_check[Dragon])
        result += 10 * n_dragon;

    if(hand_check[Simple])
        result += 5;
    else if(hand_check[MixWithTerminal])
        result += 30;
    else if(hand_check[PureWithTerminal])
        result += 40;
    else if(hand_check[MixTerminal])
        result += 120;

    if(hand_check[Straight])
        result += 30;

    if(hand_check[SameChow])
        result += 10;
    else if(hand_check[DoubleSameChow])
        result += 55;

    if(hand_check[MixTripleChow])
        result += 20;
    else if(hand_check[TriplePung])
        result += 120;

    if(hand_check[AllPungs])
        result += 40;

    if(hand_check[HalfFlush])
        result += 40;
    else if(hand_check[FullFlush])
        result += 80;

    if(hand_check[ThreeStepPung])
        result += 80;
    else if(hand_check[FourStepPung])
        result += 200;

    if(hand_check[LittleThreeDragons])
        result += 60;
    else if(hand_check[BigThreeDragons])
        result += 130;
    else if(hand_check[LittleFourWinds])
        result += 200;

    if(result > 320) // hard cap
        result = 320;

    if(result > current_score) {
        for(Hands iter = PinHu; iter < _hands_end; ++iter) {
            patterns[iter] += hand_check[iter] - old_check[iter];
            combinations[iter] += hand_check[iter]*comb - old_check[iter]*comb;
            scores[iter] += result*comb*hand_check[iter] - current_score*comb*old_check[iter];
            old_check[iter] = hand_check[iter];
        }
        return result;
    }
    else
        return current_score;

}

// 計算組合數
unsigned long long _calc_combination(const Tile* hai)
{
    unsigned long long tmp_count = 1;
    Tile tiles[HAINUM] = {Red};
    int tile_count[HAINUM] = {0};
    int tile_index = 0;
    for(int i = 0; i < HAINUM; ++i) {
        int j;
        for(j = 0; j < tile_index; ++j) {
            if(tiles[j] == hai[i]) {
                ++tile_count[j];
                break;
            }
        }
        if(j == tile_index) {
            tiles[tile_index] = hai[i];
            ++tile_count[tile_index];
            ++tile_index;
        }
    }
    for(int i=0 ; i<HAINUM; ++i) {
        switch(tile_count[i]){
        case 0: case 4:
            tile_count[i] = 1;
            break;
        case 1: case 3:
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
    return tmp_count;
}

// DONE: enum 對應的役種
const char* type_name(Hands h)
{
    switch(h) {
    case Not:
        return "無役";
    case PinHu:
        return "平和";
    case Dragon:
        return "番牌";
    case Simple:
        return "斷么";
    case Straight:
        return "一氣通貫";
    case MixWithTerminal:
        return "混全帶么";
    case PureWithTerminal:
        return "純全帶么";
    case MixTerminal:
        return "混老頭";
    case SameChow:
        return "一般高";
    case DoubleSameChow:
        return "二般高";
    case MixTripleChow:
        return "三色同順";
    case TriplePung:
        return "三色同刻";
    case AllPungs:
        return "對對和";
    case HalfFlush:
        return "混一色";
    case FullFlush:
        return "清一色";
    case ThreeStepPung:
        return "三連刻";
    case FourStepPung:
        return "四連刻";
    case LittleThreeDragons:
        return "小三元";
    case BigThreeDragons:
        return "大三元";
    case LittleFourWinds:
        return "小四喜";
    case BigFourWinds:
        return "大四喜";
    case AllHonours:
        return "字一色";
    case PureTerminal:
        return "清老頭";
    case SameQuadroChow:
        return "一色四同順";
    default:
        return "";
    }
    return "";
}

// DONE: 確認牌的正確性
int _is_valid_hai(Tile* hai, int n)
{
    if (!n) return 1;
    if ((n % 3)) {
        fprintf(stderr, "what the hell is %d?\n", n);
        return 0;
    }
     int used_index[HAINUM] = {0};
    for(int i = 0; i < HAINUM-2 ;) {
        if(used_index[i]) {
            ++i;
            continue;
        }

        if((_is_chow(hai+i) || _is_pung(hai+i)) && !(used_index[i+1]) && !(used_index[i+2])) {
            if(_is_chow(hai+i)) {
                switch(hai[i]) {
                CHOWHEAD:
                    break;
                default:
                    return 0;
                }
            }
            used_index[i] = used_index[i+1] = used_index[i+2] = 1;
            i += 3;
            continue;
        }

        int next_index = -1, last_index = -1;
        next_index = i+_find_next_tile_index(hai+i, HAINUM-i-2);
        if(next_index == -1)
            return 0;
        while(used_index[next_index])
            ++next_index;
        last_index = next_index+_find_next_tile_index(hai+next_index, HAINUM-next_index-2);
        if(last_index == -1)
            return 0;
        while(used_index[last_index])
            ++last_index;
        if(hai[next_index] == hai[i]+1 && hai[last_index] == hai[next_index]+1) {
            switch(hai[i]) {
            CHOWHEAD:
                break;
            default:
                return 0;
            }
            used_index[i] = used_index[next_index] = used_index[last_index] = 1;
        }
        else
            return 0;
    }

    return 1;
}

// 輔助函式，只輸入一組

int _is_chow(Tile* hai)
{
    if(hai[1] == hai[0]+1 && hai[2] == hai[1]+1) {
        switch(hai[0]) {
        CHOWHEAD:
            return 1;
        default:
            return 0;
        }
    }
    return Not;
}

int _is_pung(Tile* hai)
{
    return (hai[1] == hai[0] && hai[2] == hai[1]);
}

int _has_terminal(Tile* hai)
{
    return (_is_terminal(hai[0]) || _is_terminal(hai[2]));
}

// 輔助函式，只輸入單張

inline int _is_simple(Tile hai)
{
    switch(hai) {
    case B2...(BAMBOO_LAST-1): case C2...(CHARACTER_LAST-1): case D2...(DOTS_LAST-1):
        return 1;
    default:
        return 0;
    }
}

inline int _is_terminal(Tile hai)
{
    switch(hai) {
    case B1: case BAMBOO_LAST: case C1: case CHARACTER_LAST: case D1: case DOTS_LAST:
        return 1;
    default:
        return 0;
    }
}

inline int _is_honour(Tile hai)
{
    switch(hai) {
    case Red...North:
        return 1;
    default:
        return 0;
    }
}

inline int _all_same(int (*fn)(Tile*), Tile* hai)
{
    return (fn(hai+2) && fn(hai+5) && fn(hai+8) && fn(hai+11));
}

// -------------- 役種回傳分數，同類應傳回最高分役 --------------------------
// 12: 眼
// 345, 678, 9AB, CDE: 面子

/* 平和 */
Hands _pin_hu(Tile *hai)
{
    return (_all_same(_is_chow, hai) ? PinHu : Not);
}

/* DONE: 番牌 */
Hands _dragon(Tile *hai, int* n)
{
    int _is_dragon(Tile);
    for(int i=0; i<SETNUM; ++i)
        if(_is_dragon(hai[2+i*3]))
            ++(*n);
    return ((*n == 0) ? Not : Dragon);
}

int _is_dragon(Tile t)
{
    // 假設東為自風
    return (t == East || t == Red || t == Green || t == White);
}

/* DONE: 斷么 */
Hands _simple(Tile *hai)
{
    for(int i=0; i<HAINUM; ++i) {
        switch(hai[i]) {
        case Red...B1: case BAMBOO_LAST: case C1: case CHARACTER_LAST: case D1: case DOTS_LAST:
            return Not;
        case B2...(BAMBOO_LAST-1): case C2...(CHARACTER_LAST-1): case D2...(DOTS_LAST-1):
            break;
        default:
            return Not;
        }
    }
    return Simple;
}

/* 一氣通貫 */
Hands _straight(Tile *hai)
{
    Hands result = Not;
    int chows[SETNUM] = {0}; // boolean
    Hands _find_straight(Tile*, int*, int, int, int);

    for(int i=0; i<SETNUM; ++i)
        chows[i] = _is_chow(hai+3*i+2);
    TRIPLEFUNC(result, _find_straight, hai, chows);

    return result;
}

Hands _find_straight(Tile* hai, int* chows, int a, int b, int c)
{
    if(chows[a/3] & chows[b/3] & chows[c/3]) {
        if(hai[b] == hai[a]+3 && hai[c] == hai[b]+3) {
            switch(hai[a]) {
            case B1: case C1: case D1:
                return Straight;
            default:
                return Not;
            }
        }
    }
    return Not;
}

/* 么九類 */
Hands _terminal(Tile *hai)
{
    if(_all_pungs(hai) == AllPungs) {
        // 清老頭
        if(_is_terminal(hai[0]) && _is_terminal(hai[2]) && _is_terminal(hai[5]) && _is_terminal(hai[8]) && _is_terminal(hai[11]))
            return PureTerminal;

        // 混老頭
        if((_is_terminal(hai[0]) || _is_honour(hai[0]))
           && (_is_terminal(hai[2]) || _is_honour(hai[2]))
           && (_is_terminal(hai[5]) || _is_honour(hai[5]))
           && (_is_terminal(hai[8]) || _is_honour(hai[8]))
           && (_is_terminal(hai[11]) || _is_honour(hai[11])))
            return MixTerminal;
    }
    // 純全帶
    if( _is_terminal(hai[0]) && _all_same(_has_terminal, hai))
        return PureWithTerminal;

    // 混全帶
    if((_is_terminal(hai[0]) || _is_honour(hai[0]))
       && (_has_terminal(hai+2) || _is_honour(hai[2]))
       && (_has_terminal(hai+5) || _is_honour(hai[5]))
       && (_has_terminal(hai+8) || _is_honour(hai[8]))
       && (_has_terminal(hai+11) || _is_honour(hai[11])))
        return MixWithTerminal;

    return Not;
}


/* 同順類 */
Hands _same_chow(Tile *hai)
{
    Hands _find_same_chow(Tile*, int*, int, int);
    int chows[SETNUM] = {0};
    Hands result = Not;

    for(int i=0; i<SETNUM; ++i)
        chows[i] = _is_chow(hai+i*3+2);

    // 一色四同順, 拆解形為 111 123 222 333
    if((!chows[0]) && chows[1] && (!chows[2]) && (!chows[3]))
        if(hai[5] == hai[2] && hai[8] == hai[5]+1 && hai[11] == hai[8]+1) 
            return SameQuadroChow;
    
    // 二般高
    if(_pin_hu(hai) == PinHu)
        if(hai[2] == hai[5] && hai[11] == hai[8])
                return DoubleSameChow;

    // 一色三同順等同三連刻

    // 一般高
    DUOFUNC(result, _find_same_chow, hai, chows);

    return result;
}

Hands _find_same_chow(Tile* hai, int* arr, int a, int b)
{
    if(arr[a/3] & arr[b/3])
        if(hai[a] == hai[b])
            return SameChow;
    return Not;
}

/* DONE: 三色類 */
Hands _mix_triple(Tile *hai)
{
    int pungs[SETNUM] = {0}; // boolean
    int chows[SETNUM] = {0}; // boolean
    for(int i=0; i<SETNUM; ++i) {
        pungs[i] = _is_pung(hai+i*3+2);
        chows[i] = !pungs[i];
    }
    Hands result = Not;
    Hands _find_triple_pung(Tile*, int*, int, int, int);
    Hands _find_triple_chow(Tile*, int*, int, int, int);

    // 三色同刻
    TRIPLEFUNC(result, _find_triple_pung, hai, pungs);
    if(result != Not)
        return result;

    // 三色同順
    TRIPLEFUNC(result, _find_triple_chow, hai, chows);

    return result;
}
Hands _find_triple_pung(Tile* hai, int* arr, int a, int b, int c)
{
    if(arr[a/3] & arr[b/3] & arr[c/3]) {
        if(hai[b] == hai[a]+TILE_PER_COLOR && hai[c] == hai[b]+TILE_PER_COLOR) {
            switch(hai[a]) {
            case B1...BAMBOO_LAST:
                return TriplePung;
            default:
                return Not;
            }
        }
    }
    return Not;
}

Hands _find_triple_chow(Tile* hai, int* arr, int a, int b, int c)
{

    if(arr[a/3] & arr[b/3] & arr[c/3]) {
        if(hai[b] == hai[a]+TILE_PER_COLOR && hai[c] == hai[b]+TILE_PER_COLOR) {
            switch(hai[a]) {
            case B1...(BAMBOO_LAST-2):
                return MixTripleChow;
            default:
                return Not;
            }
        }
    }
    return Not;
}

/* 對對和 */
Hands _all_pungs(Tile *hai)
{
    return (_all_same(_is_pung, hai) ? AllPungs : Not);
}

/* DONE: 一色類 */
Hands _flush(Tile *hai)
{
    int _is_color(Tile);
    // 0: 字, 1: 索, 2: 萬, 3: 筒
    int color = _is_color(hai[0]);
    int i = 0;
    for(i=0; i<HAINUM; ++i)
        if((color=_is_color(hai[i])) != 0)
            break;
    // 字一色
    if(i == HAINUM)
        return AllHonours;

    // 清一色
    for(i=0; i<HAINUM; ++i) {
        if(_is_color(hai[i]) != color) {
            if(_is_color(hai[i]) != 0)
                return Not;
            else
                break;
        }
    }
    if(i == HAINUM)
        return FullFlush;

    // 混一色
    for(i=0; i<HAINUM; ++i)
        if(_is_color(hai[i]) != color && _is_color(hai[i]) != 0)
            return Not;

    return HalfFlush;
}

inline int _is_color(Tile t)
{
    return (t+FLUSHMAGIC)/TILE_PER_COLOR;
}

/* DONE: 字牌類 */
Hands _honours(Tile *hai)
{
    int _is_four_wind(Tile);
    int _is_three_dragon(Tile);
    int sum_wind = 0, sum_dragon = 0;

    for(int i=0; i<SETNUM; ++i) {
        sum_wind += _is_four_wind(hai[2+i*3]);
        sum_dragon += _is_three_dragon(hai[2+i*3]);
    }
    
    // 大四喜
    if(sum_wind == 4)
        return BigFourWinds;
    // 小四喜
    if(sum_wind == 3 && _is_four_wind(hai[0]))
        return LittleFourWinds;
    // 大三元
    if(sum_dragon == 3)
        return BigThreeDragons;
    // 小三元
    if(sum_dragon == 2 && _is_three_dragon(hai[0]))
        return LittleThreeDragons;

    return Not;
}

int _is_four_wind(Tile t)
{
    return (t == East || t == South || t == West || t == North);
}

int _is_three_dragon(Tile t)
{
    return (t == Red || t == Green || t == White);
}

/* DONE: 連刻類 */
Hands _step_pungs(Tile* hai)
{
    Hands result = Not;
    int pungs[SETNUM] = {0};
    Hands _three_step_pungs(Tile*, int*, int, int, int);

    /* 四連刻 */
    if(_all_pungs(hai) == AllPungs) {
        if(hai[5] == hai[2]+1 && hai[8] == hai[5]+1 && hai[11] == hai[8]+1) {
            switch(hai[2]) {
            case B1...(BAMBOO_LAST-3): case C1...(CHARACTER_LAST-3): case D1...(DOTS_LAST-3):
                return FourStepPung;
            default:
                break;
            }
        }
    }

    for(int i=0; i<SETNUM; ++i)
        pungs[i] = _is_pung(hai+i*3+2);
    /* 三連刻 */
    TRIPLEFUNC(result, _three_step_pungs, hai, pungs);

    return result;
}

Hands _three_step_pungs(Tile* hai, int* pungs, int a, int b, int c)
{
    if(pungs[a/3] & pungs[b/3] & pungs[c/3]) {
        if(hai[b] == hai[a]+1 && hai[c] == hai[b]+1) {
            switch(hai[a]) {
            case B1...(BAMBOO_LAST-2): case C1...(CHARACTER_LAST-2): case D1...(DOTS_LAST-2):
                return ThreeStepPung;
            default:
                return Not;
            }
        }
    }
    return Not;
}
