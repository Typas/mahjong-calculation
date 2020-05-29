#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HAINUM 11

#define CHOWHEAD case B1...B4: case C1...C4: case D1...D4

typedef enum Tile{
    Red,
    Green,
    White,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    _tile_end
} Tile;

typedef enum Hands {
    Not,   // 非此役
    PinHu, // 平和
    Dragon, // 番牌
    Straight, // 一氣通貫
    MixTerminal, // 混老頭
    SameChow, // 一般高
    MixTripleChow, // 三色同順
    AllPungs, // 對對和
    HalfFlush, // 混一色
    FullFlush, // 清一色
    SingleAnko, // 一暗刻
    DoubleAnko, // 二暗刻
    TripleAnko, // 三暗刻
    DoubleStepPung, // 二連刻
    TripleStepPung, // 三連刻
    LittleThreeDragons, // 小三元
    TriplePung, // 三色同刻
    PureTerminal, // 清老頭
    BigThreeDragons, // 大三元
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
Hands _dragon(Tile*);
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
/* 三元類 */
Hands _three_dragons(Tile*);
/* 暗刻類 */
Hands _anko(Tile*);
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

int main(int argc, char *argv[])
{
    Tile hai[HAINUM];
    char tmp[HAINUM];
    int counter = 0;
    char *fname = "patterns_general_three.dat";
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

void type_check(Tile *hai)
{
    // get pair, check validity
    // if valid, go fn score to calculate score
    // 高點法只計不同雀頭，順刻轉換必為三連刻
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
            Tile subhai[HAINUM/3][3] = {{Red}};
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
            for(int j = 0; j < HAINUM/3; ++j)
                _hai_copy(varhai+2+3*j, subhai[j], 3);

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

unsigned score(Tile *hai, int *old_check, unsigned current_score)
{
    int hand_check[_hands_end] = {0};
    Hands i = Not;
    i = _pin_hu(hai);
    hand_check[i] = 1;
    i = _dragon(hai);
    hand_check[i] = 1;
    i = _straight(hai);
    hand_check[i] = 1;
    i = _terminal(hai);
    hand_check[i] = 1;
    i = _same_chow(hai);
    hand_check[i] = 1;
    i = _mix_triple(hai);
    hand_check[i] = 1;
    i = _all_pungs(hai);
    hand_check[i] = 1;
    i = _flush(hai);
    hand_check[i] = 1;
    i = _three_dragons(hai);
    hand_check[i] = 1;
    i = _anko(hai);
    hand_check[i] = 1;
    i = _step_pungs(hai);
    hand_check[i] = 1;

    unsigned long long comb = _calc_combination(hai);
    // 役滿
    if(hand_check[BigThreeDragons] | hand_check[PureTerminal]) {
        // 只計役滿
        for(Hands i = PureTerminal; i < _hands_end; ++i) {
            patterns[i] += hand_check[i];
            combinations[i] += hand_check[i]*comb;
            scores[i] += ((i==PureTerminal)?200:200)*comb*hand_check[i] - current_score*comb*old_check[i];
        }
        for(Hands i = PinHu; i < PureTerminal; ++i) {
            patterns[i] -= old_check[i];
            combinations[i] -= old_check[i];
            scores[i] -= current_score*comb*old_check[i];
            old_check[i] = 0;
        }
        // 回傳最大值
        if(hand_check[BigThreeDragons] | hand_check[PureTerminal])
            return 200;
        else
            return 160;
    }

    // 非役滿
    unsigned result = 0;

    if(hand_check[PinHu])
        result += 5;
    
    if(hand_check[Dragon])
        result += 20;
    
    if(hand_check[MixTerminal])
        result += 80;

    if(hand_check[Straight])
        result += 10;

    if(hand_check[SameChow])
        result += 20;

    if(hand_check[MixTripleChow])
        result += 35;
    else if(hand_check[TriplePung])
        result += 120;

    if(hand_check[AllPungs])
        result += 40;

    if(hand_check[HalfFlush])
        result += 40;
    else if(hand_check[FullFlush])
        result += 80;

    if(hand_check[LittleThreeDragons])
        result += 80;

    if(hand_check[DoubleStepPung])
        result += 20;
    else if(hand_check[TripleStepPung])
        result += 80;

    if(result > 160) // hard cap
        result = 160;

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

const char* type_name(Hands h)
{
    switch(h) {
    case Not:
        return "無役";
    case PinHu:
        return "平和";
    case Dragon:
        return "番牌";
    case Straight:
        return "一氣通貫";
    case MixTerminal:
        return "混老頭";
    case SameChow:
        return "一般高";
    case MixTripleChow:
        return "三色同順";
    case AllPungs:
        return "對對和";
    case HalfFlush:
        return "混一色";
    case FullFlush:
        return "清一色";
    case SingleAnko:
        return "一暗刻";
    case DoubleAnko:
        return "二暗刻";
    case TripleAnko:
        return "三暗刻";
    case DoubleStepPung:
        return "二連刻";
    case TripleStepPung:
        return "三連刻";
    case LittleThreeDragons:
        return "小三元";
    case TriplePung:
        return "三色同刻";
    case PureTerminal:
        return "清老頭";
    case BigThreeDragons:
        return "大三元";
    default:
        return "";
    }
    return "";
}

// 確認牌的正確性
int _is_valid_hai(Tile* hai, int n)
{
    if (!n) return 1;
    if ((n % 3)) {
        // not possible
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
    case B2...B5: case C2...C5: case D2...D5:
        return 1;
    default:
        return 0;
    }
}

inline int _is_terminal(Tile hai)
{
    switch(hai) {
    case B1: case B6: case C1: case C6: case D1: case D6:
        return 1;
    default:
        return 0;
    }
}

inline int _is_honour(Tile hai)
{
    switch(hai) {
    case Red...White:
        return 1;
    default:
        return 0;
    }
}

// -------------- 役種回傳分數，同類應傳回最高分役 --------------------------
// 12: 眼
// 345, 678, 9AB: 面子

/* 平和 */
Hands _pin_hu(Tile *hai)
{
    if(_is_chow(hai+2) && _is_chow(hai+5) && _is_chow(hai+8))
        return PinHu;
    else
        return Not;
}

/* 番牌 */
Hands _dragon(Tile *hai)
{
    // 假設紅中為自風
    if(hai[2] == Red || hai[5] == Red || hai[8] == Red)
        return Dragon;
    else
        return Not;
}

/* 一氣通貫 */
Hands _straight(Tile *hai)
{
    // 12
    if(_is_chow(hai+2) && _is_chow(hai+5)) {
        if(hai[5] == hai[2] + 3) {
            switch(hai[2]) {
            case B1: case C1: case D1:
                return Straight;
            default:
                break;
            }
        }
    }
    
    // 13
    if(_is_chow(hai+2) && _is_chow(hai+8)) {
        if(hai[8] == hai[2] + 3) {
            switch(hai[2]) {
            case B1: case C1: case D1:
                return Straight;
            default:
                break;
            }
        }
    }

    // 23
    if(_is_chow(hai+5) && _is_chow(hai+8)) {
        if(hai[8] == hai[5] + 3) {
            switch(hai[5]) {
            case B1: case C1: case D1:
                return Straight;
            default:
                break;
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
        if(_is_terminal(hai[0]) && _is_terminal(hai[2]) && _is_terminal(hai[5]) && _is_terminal(hai[8]))
            return PureTerminal;
        
        // 混老頭
        if((_is_terminal(hai[0]) || _is_honour(hai[0]))
           && (_is_terminal(hai[2]) || _is_honour(hai[2]))
           && (_is_terminal(hai[5]) || _is_honour(hai[5]))
           && (_is_terminal(hai[8]) || _is_honour(hai[8])))
            return MixTerminal;
    }
    return Not;
}

/* 同順類 */
Hands _same_chow(Tile *hai)
{
    /* // 一色三同順 */
    /* if(_pin_hu(hai) == PinHu) */
    /*     if(hai[2] == hai[5] && hai[8] == hai[5]) */
    /*         return SameTripleChow; */
    
    // 一般高
    // 12
    if(_is_chow(hai+2) && _is_chow(hai+5))
        if(hai[5] == hai[2])
            return SameChow;

    // 13
    if(_is_chow(hai+2) && _is_chow(hai+8))
        if(hai[8] == hai[2])
            return SameChow;

    // 23
    if(_is_chow(hai+5) && _is_chow(hai+8))
        if(hai[8] == hai[5])
            return SameChow;
    
    return Not;
}

/* 三色類 */
Hands _mix_triple(Tile *hai)
{
    // 三色同刻
    if(_all_pungs(hai) == AllPungs) {
        if(hai[5] == hai[2] + 6 && hai[8] == hai[5] + 6) {
            switch(hai[2]) {
            case B1...B6:
                return TriplePung;
            default:
                return Not;
            }
        }
    }
    
    // 三色同順
    if(_pin_hu(hai) == PinHu) {
        if(hai[5] == hai[2] + 6 && hai[8] == hai[5] + 6) {
            switch(hai[2]) {
            case B1...B4:
                return MixTripleChow;
            default:
                return Not;
            }
        }
    }

    return Not; // flycheck is idiot
}

/* 對對和 */
Hands _all_pungs(Tile *hai)
{
    if(_is_pung(hai+2) && _is_pung(hai+5) && _is_pung(hai+8))
        return AllPungs;
    else
        return Not;
}

/* 一色類 */
Hands _flush(Tile *hai)
{
    // 0: 字, 1: 索, 2: 萬, 3: 筒
    int color = 0;
    for(int i=0; i<HAINUM; ++i)
        if((color = (hai[i]+3)/6) != 0)
            break;

    // 清一色
    if(color != 0){
        int i = 0;
        for(i=0; i<HAINUM; ++i) {
            if((hai[i]+3)/6 != color) {
                if((hai[i]+3)/6 != 0)
                    return Not;
                else
                    break;
            }
        }
        if(i == HAINUM)
            return FullFlush;
    }
    
    // 混一色
    for(int i=0; i<HAINUM; ++i)
        if((hai[i]+3)/6 != color && (hai[i]+3)/6 != 0)
            return Not;
    
    return HalfFlush;
}

/* 三元類 */
Hands _three_dragons(Tile *hai)
{
    // 大三元
    if(_is_honour(hai[2]) && _is_honour(hai[5]) && _is_honour(hai[8]))
        return BigThreeDragons;
    
    // 小三元
    if(_is_honour(hai[0])) {
        int count = 0;
        for(int i = 2; i < HAINUM ; i+=3)
            count += _is_honour(hai[i]);
        if(count == 2)
            return LittleThreeDragons;
        else
            return Not;
    }

    return Not; // flycheck is idiot
}

/* 暗刻類 */
Hands _anko(Tile *hai)
{
    int pungs = _is_pung(hai+2) + _is_pung(hai+5) + _is_pung(hai+8);

    switch(pungs) {
    case 1:
        return SingleAnko;
    case 2:
        return DoubleAnko;
    case 3:
        return TripleAnko;
    default:
        break;
    }
    
    return Not;
}

/* 連刻類 */
Hands _step_pungs(Tile *hai)
{
    // 三連刻
    if(_all_pungs(hai) == AllPungs) {
        if(hai[5] == hai[2] + 1 && hai[8] == hai[5] + 1) {
            switch(hai[2]) {
            case B1...B4: case C1...C4: case D1...D4:
                return TripleStepPung;
            default:
                break;
            }
        }
    }

    // 二連刻
    if(hai[5] == hai[2] + 1 && _is_pung(hai+5) && _is_pung(hai+2)) {
        switch(hai[2]) {
        case B1...B5: case C1...C5: case D1...D5:
            return DoubleStepPung;
        default:
            break;
        }
    }
    if(hai[8] == hai[5] + 1 && _is_pung(hai+8) && _is_pung(hai+5)) {
        switch(hai[5]) {
        case B1...B5: case C1...C5: case D1...D5:
            return DoubleStepPung;
        default:
            break;
        }
    }
    if(hai[8] == hai[2] + 1 && _is_pung(hai+8) && _is_pung(hai+2)) {
        switch(hai[2]) {
        case B1...B5: case C1...C5: case D1...D5:
            return DoubleStepPung;
        default:
            break;
        }
    }

    return Not;
}
