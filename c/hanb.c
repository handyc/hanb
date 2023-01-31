#include <stdio.h>
#include <string.h>

int print_board(char* board, int board_rotation)
{
char edgeboard_0[] = "        a   a   a   a   a         \n";
char edgeboard_1[] = "      a   a   a   a   a   a       \n";
char edgeboard_2[] = "    a   a   a   a   a   a   a     \n";
char edgeboard_3[] = "  a   a   a   a   a   a   a   a   \n";
char edgeboard_4[] = "a   a   a   a   a   a   a   a   a \n";
char edgeboard_5[] = "  a   a   a   a   a   a   a   a   \n";
char edgeboard_6[] = "    a   a   a   a   a   a   a     \n";
char edgeboard_7[] = "      a   a   a   a   a   a       \n";
char edgeboard_8[] = "        a   a   a   a   a         \n";

char pointboard_0[]  = "                    a                     \n";
char pointboard_1[]  = "               a         a                \n";
char pointboard_2[]  = "          a         a         a           \n";
char pointboard_3[]  = "     a         a         a         a      \n";
char pointboard_4[]  = "a         a         a         a         a \n";
char pointboard_5[]  = "     a         a         a         a      \n";
char pointboard_6[]  = "a         a         a         a         a \n";
char pointboard_7[]  = "     a         a         a         a      \n";
char pointboard_8[] =  "a         a         a         a         a \n";
char pointboard_9[] =  "     a         a         a         a      \n";
char pointboard_10[] = "a         a         a         a         a \n";
char pointboard_11[] = "     a         a         a         a      \n";
char pointboard_12[] = "a         a         a         a         a \n";
char pointboard_13[] = "     a         a         a         a      \n";
char pointboard_14[] = "          a         a         a           \n";
char pointboard_15[] = "               a         a                \n";
char pointboard_16[] = "                    a                     \n";

size_t length = strlen(board);
if(length > 64)
{
board[64] = '\0';
}

if(board_rotation == 0)
    {
    edgeboard_0[8] = board[0];
    edgeboard_0[12] = board[1];
    edgeboard_0[16] = board[2];
    edgeboard_0[20] = board[3];
    edgeboard_0[24] = board[4];

    edgeboard_1[6] = board[5];
    edgeboard_1[10] = board[6];
    edgeboard_1[14] = board[7];
    edgeboard_1[18] = board[8];
    edgeboard_1[22] = board[9];
    edgeboard_1[26] = board[10];

    edgeboard_2[4] = board[11];
    edgeboard_2[8] = board[12];
    edgeboard_2[12] = board[13];
    edgeboard_2[16] = board[14];
    edgeboard_2[20] = board[15];
    edgeboard_2[24] = board[16];
    edgeboard_2[28] = board[17];

    edgeboard_3[2] = board[18];
    edgeboard_3[6] = board[19];
    edgeboard_3[10] = board[20];
    edgeboard_3[14] = board[21];
    edgeboard_3[18] = board[22];
    edgeboard_3[22] = board[23];
    edgeboard_3[26] = board[24];
    edgeboard_3[30] = board[25];

    edgeboard_4[0] = board[26];
    edgeboard_4[4] = board[27];
    edgeboard_4[8] = board[28];
    edgeboard_4[12] = board[29];
    edgeboard_4[16] = board[30];
    edgeboard_4[20] = board[31];
    edgeboard_4[24] = board[32];
    edgeboard_4[28] = board[33];
    edgeboard_4[32] = board[34];

    edgeboard_5[2] = board[35];
    edgeboard_5[6] = board[36];
    edgeboard_5[10] = board[37];
    edgeboard_5[14] = board[38];
    edgeboard_5[18] = board[39];
    edgeboard_5[22] = board[40];
    edgeboard_5[26] = board[41];
    edgeboard_5[30] = board[42];

    edgeboard_6[4] = board[43];
    edgeboard_6[8] = board[44];
    edgeboard_6[12] = board[45];
    edgeboard_6[16] = board[46];
    edgeboard_6[20] = board[47];
    edgeboard_6[24] = board[48];
    edgeboard_6[28] = board[49];

    edgeboard_7[6] = board[50];
    edgeboard_7[10] = board[51];
    edgeboard_7[14] = board[52];
    edgeboard_7[18] = board[53];
    edgeboard_7[22] = board[54];
    edgeboard_7[26] = board[55];

    edgeboard_8[8] = board[56];
    edgeboard_8[12] = board[57];
    edgeboard_8[16] = board[58];
    edgeboard_8[20] = board[59];
    edgeboard_8[24] = board[60];

    printf("%s\n", board);
    printf("%s", edgeboard_0);
    printf("%s", edgeboard_1);
    printf("%s", edgeboard_2);
    printf("%s", edgeboard_3);
    printf("%s", edgeboard_4);
    printf("%s", edgeboard_5);
    printf("%s", edgeboard_6);
    printf("%s", edgeboard_7);
    printf("%s", edgeboard_8);
    }
if(board_rotation == 1)
    {
    pointboard_0[20] = board[0];

    pointboard_1[15] = board[1];
    pointboard_1[25] = board[2];

    pointboard_2[10] = board[3];
    pointboard_2[20] = board[4];
    pointboard_2[30] = board[5];

    pointboard_3[5] = board[6];
    pointboard_3[15] = board[7];
    pointboard_3[25] = board[8];
    pointboard_3[35] = board[9];

    pointboard_4[0] = board[10];
    pointboard_4[10] = board[11];
    pointboard_4[20] = board[12];
    pointboard_4[30] = board[13];
    pointboard_4[40] = board[14];

    pointboard_5[5] = board[15];
    pointboard_5[15] = board[16];
    pointboard_5[25] = board[17];
    pointboard_5[35] = board[18];

    pointboard_6[0] = board[19];
    pointboard_6[10] = board[20];
    pointboard_6[20] = board[21];
    pointboard_6[30] = board[22];
    pointboard_6[40] = board[23];

    pointboard_7[5] = board[24];
    pointboard_7[15] = board[25];
    pointboard_7[25] = board[26];
    pointboard_7[35] = board[27];

    pointboard_8[0] = board[28];
    pointboard_8[10] = board[29];
    pointboard_8[20] = board[30];
    pointboard_8[30] = board[31];
    pointboard_8[40] = board[32];

    pointboard_9[5] = board[33];
    pointboard_9[15] = board[34];
    pointboard_9[25] = board[35];
    pointboard_9[35] = board[36];

    pointboard_10[0] = board[37];
    pointboard_10[10] = board[38];
    pointboard_10[20] = board[39];
    pointboard_10[30] = board[40];
    pointboard_10[40] = board[41];

    pointboard_11[5] = board[42];
    pointboard_11[15] = board[43];
    pointboard_11[25] = board[44];
    pointboard_11[35] = board[45];

    pointboard_12[0] = board[46];
    pointboard_12[10] = board[47];
    pointboard_12[20] = board[48];
    pointboard_12[30] = board[49];
    pointboard_12[40] = board[50];

    pointboard_13[5] = board[51];
    pointboard_13[15] = board[52];
    pointboard_13[25] = board[53];
    pointboard_13[35] = board[54];

    pointboard_14[10] = board[55];
    pointboard_14[20] = board[56];
    pointboard_14[30] = board[57];

    pointboard_15[15] = board[58];
    pointboard_15[25] = board[59];

    pointboard_16[20] = board[60];


    printf("%s\n", board);
    printf("%s", pointboard_0);
    printf("%s", pointboard_1);
    printf("%s", pointboard_2);
    printf("%s", pointboard_3);
    printf("%s", pointboard_4);
    printf("%s", pointboard_5);
    printf("%s", pointboard_6);
    printf("%s", pointboard_7);
    printf("%s", pointboard_8);
    printf("%s", pointboard_9);
    printf("%s", pointboard_10);
    printf("%s", pointboard_11);
    printf("%s", pointboard_12);
    printf("%s", pointboard_13);
    printf("%s", pointboard_14);
    printf("%s", pointboard_15);
    printf("%s", pointboard_16);
    }

return 0;
}

int main()
{
char example_board[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.";
//char example_board[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.abcdefghijklmnopqrstuvwxyz";

print_board(example_board, 0);
print_board(example_board, 1);

return 0;
}
