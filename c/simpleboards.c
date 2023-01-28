#include <stdio.h>

int main()
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

printf("%s", edgeboard);
printf("%s", pointboard);

return 0;
}
