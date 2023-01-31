#!//usr/local/bin/python3

edgeboard = """
        a   a   a   a   a
      a   a   a   a   a   a
    a   a   a   a   a   a   a
  a   a   a   a   a   a   a   a
a   a   a   a   a   a   a   a   a
  a   a   a   a   a   a   a   a
    a   a   a   a   a   a   a
      a   a   a   a   a   a
        a   a   a   a   a
"""

pointboard = """
                    a
               a         a
          a         a         a
     a         a         a         a
a         a         a         a         a
     a         a         a         a
a         a         a         a         a
     a         a         a         a
a         a         a         a         a
     a         a         a         a
a         a         a         a         a
     a         a         a         a
a         a         a         a         a
     a         a         a         a
          a         a         a
               a         a
                    a
"""

def print_board(board):
    print(board)

def main():
    print_board(edgeboard)
    print_board(pointboard)

if __name__ == "__main__":
    main()

