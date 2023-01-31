#include <stdio.h>

int print_board(int board_rotation)
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
    printf("%s", edgeboard);
    }
if(board_rotation == 1)
    {
    printf("%s", pointboard);
    }

return 0;
}

int main()
{
print_board(0);
print_board(1);

return 0;
}
