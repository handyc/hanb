#!/usr/local/bin/python3

def print_board(board, rotation):
    edgeboard = """        a   a   a   a   a
      a   a   a   a   a   a
    a   a   a   a   a   a   a
  a   a   a   a   a   a   a   a
a   a   a   a   a   a   a   a   a
  a   a   a   a   a   a   a   a
    a   a   a   a   a   a   a
      a   a   a   a   a   a
        a   a   a   a   a"""

    pointboard = """                    a
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
                    a"""

    if rotation == 0:
        print(edgeboard)
    elif rotation == 1:
        print(pointboard)

def main():
    example_board = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-."

    print_board(example_board, 0)
    print_board(example_board, 1)

if __name__ == "__main__":
    main()
