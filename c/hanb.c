#include <stdio.h>

int print_board(char *board, int board_rotation)
{
char *edgeboard = "\
        a   a   a   a   a         \n\
      a   a   a   a   a   a       \n\
    a   a   a   a   a   a   a     \n\
  a   a   a   a   a   a   a   a   \n\
a   a   a   a   a   a   a   a   a \n\
  a   a   a   a   a   a   a   a   \n\
    a   a   a   a   a   a   a     \n\
      a   a   a   a   a   a       \n\
        a   a   a   a   a         \n\
";

char *pointboard = "\
                    a                     \n\
               a         a                \n\
          a         a         a           \n\
     a         a         a         a      \n\
a         a         a         a         a \n\
     a         a         a         a      \n\
a         a         a         a         a \n\
     a         a         a         a      \n\
a         a         a         a         a \n\
     a         a         a         a      \n\
a         a         a         a         a \n\
     a         a         a         a      \n\
a         a         a         a         a \n\
     a         a         a         a      \n\
          a         a         a           \n\
               a         a                \n\
                    a                     \n\
";

if(board_rotation == 0)
    {
    printf("%s\n", board);
    printf("%s", edgeboard);
    }
if(board_rotation == 1)
    {
    printf("%s\n", board);
    printf("%s", pointboard);
    }

return 0;
}

int main()
{
char *example_board = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.";

print_board(example_board, 0);
print_board(example_board, 1);

return 0;
}
