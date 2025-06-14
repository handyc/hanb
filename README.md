# hanb

* hanb is a simple language for creating model universes at any scale  
* hanb can be used with or without a computer  
* if you know the alphabet then you can use hanb  

# board

the basis of the hanb system is a 61-cell hexagonal board  
(displayed in text mode in multiple sizes with 2 different rotations):

``         a   a   a   a   a         ``  
``       a   a   a   a   a   a       ``  
``     a   a   a   a   a   a   a     ``  
``   a   a   a   a   a   a   a   a   ``  
`` a   a   a   a   a   a   a   a   a ``  
``   a   a   a   a   a   a   a   a   ``  
``     a   a   a   a   a   a   a     ``  
``       a   a   a   a   a   a       ``  
``         a   a   a   a   a         `` 

``                 a                   ``  
``             a       a               ``   
``         a       a       a           ``  
``     a       a       a       a       ``    
`` a       a       a       a       a   ``    
``     a       a       a       a       ``   
`` a       a       a       a       a   ``    
``     a       a       a       a       ``    
`` a       a       a       a       a   ``    
``     a       a       a       a       ``    
`` a       a       a       a       a   ``    
``     a       a       a       a       ``    
``  a      a       a       a       a   ``    
``     a       a       a       a       ``   
``         a       a       a           ``   
``             a       a               ``    
``                 a                   ``    

``           a    a    a    a    a           ``  
``                                         ``  
``         a    a    a    a    a    a        ``   
``                                         ``  
``      a    a    a    a    a    a    a      ``   
``                                         ``  
``   a    a    a    a    a    a    a    a    ``   
``                                         ``  
`` a    a    a    a    a    a    a    a    a ``    
``                                         ``  
``   a    a    a    a    a    a    a    a    ``  
``                                         ``  
``      a    a    a    a    a    a    a      ``  
``                                         ``  
``         a    a    a    a    a    a        ``  
``                                         ``  
``           a    a    a    a    a           `` 

``                     a                     ``  
``                a         a                ``  
``           a         a         a           ``  
``      a         a         a         a      ``  
`` a         a         a         a         a ``  
``      a         a         a         a      ``  
`` a         a         a         a         a ``  
``      a         a         a         a      ``  
`` a         a         a         a         a ``  
``      a         a         a         a      ``  
`` a         a         a         a         a ``  
``      a         a         a         a      ``  
`` a         a         a         a         a ``  
``      a         a         a         a      ``  
``           a         a         a           ``  
``                a         a                ``  
``                     a                     ``  



each cell contains a single character from a set of 64 total characters:

a-z, A-Z, 0-9, - and .

boards can be written in a compressed form as 61-character strings, e.g.:

``tFZ2fD1LbWJYCkaB-feYB7wkEVjINP.-taGiY8iAef7R1noBhM-uz.Bdwgmld``  

``lmXDMBOHISwvumSev7rZtf449JmaUfPIN0EGWHK-yu782UobkdRNNhrkkDVTS``  

``acuXsBJ18dV-dXTD3pi8hDD6_bDFIQ94wi9vFNbZyCa3ZL8NNN6Bb2zVzoqRE``  

``fMTNqDeeIKrPQAqveHdY5.BZRDtHxMDlotxyR3y2Cy-5VnrnDpxX6Ssk1LRps``  

``19i7oMvDURxQzfHiVVCbA4dL278J-XvPfgL.rTOgcQrrK03nlCBJ3RMXHdOyP``  

``aYfqpEOK1s5gtDE-ocemVoUH45Ef88Pp9POLKF7cH97WADGBa2dBC6kL6wljU``  

``t7t82Ry18okmA1HCr0R8zi0HncVQyGtTo5xGo-44PXA4NRK0ND-sbVrT8fJMt``  

Each character in the string corresponds to the value of a single cell on the board.
e.g. in the string ``t7t82Ry18okmA1HCr0R8zi0HncVQyGtTo5xGo-44PXA4NRK0ND-sbVrT8fJMt``  
the value of cell 0 is 't', and the value of cell 1 is '7'.

``                     t                     ``  
``                7         t                ``  
``           8         2         R           ``  
``      y         1         8         o      ``  
`` k         m         A         1         H ``  
``      C         r         0         R      ``  
`` 8         z         i         0         H ``  
``      n         c         V         Q      ``  
`` y         G         t         T         o ``  
``      5         x         G         o      ``  
`` -         4         4         P         X ``  
``      A         4         N         R      ``  
`` K         0         N         D         - ``  
``      s         b         V         r      ``  
``           T         8         f           ``  
``                J         M                ``  
``                     t                     ``  


The values of cells can be interpreted in a variety of ways.

``t7t82Ry18okmA1HCr0R8zi0HncVQyGtTo5xGo-44PXA4NRK0ND-sbVrT8fJMt``  

Another way to read the same string results in the following board:  


``         t   7   t   8   2         ``  
``       R   y   1   8   o   k       ``  
``     m   A   1   H   C   r   0     ``  
``   R   8   z   i   0   H   n   c   ``  
`` V   Q   y   G   t   T   o   5   x ``  
``   G   o   -   4   4   P   X   A   ``  
``     4   N   R   K   0   N   D     ``  
``       -   s   b   V   r   T       ``  
``         8   f   J   M   t         `` 

The same characters are used as in the previous board, but the difference
in rotation means that the relationships between cell values are now different.

The main purpose of cell values is to indicate the presence
and type of matter contained in a cell.

A hanb board is a map of the relationships between objects.

Each board represents a finite space at a particular scale between 0 and 63,
also represented by the above character set, such that 'a' is a board filled
with quantum foam and '.' is a board containing a universe.

We can use the values of cells on a board to represent objects of different
sizes in this way, and interpret those cells as similar boards at smaller scales.
The rotations of the hexagon of any hanb board are 90 degrees different from
the cells of that board.


``         t   7   t   8   2         ``  
``       R   y   1   8   o   k       ``  
``     m   A   1   H   C   r   0     ``  
``   R   8   z   i   0   H   n   c   ``  
`` V   Q   y   G   t   T   o   5   x ``  
``   G   o   -   4   4   P   X   A   ``  
``     4   N   R   K   0   N   D     ``  
``       -   s   b   V   r   T       ``  
``         8   f   J   M   t         ``   
``                     |             ``  
``                     |             ``  
``                     t                     ``  
``                v         v                ``  
``           d         v         s           ``  
``      j         v         v         b      ``  
`` j         j         L         d         b ``  
``      j         j         b         b      ``  
`` j         L         L         L         b ``  
``      j         L         L         h      ``  
`` j         L         L         L         o ``  
``      a         L         L         h      ``  
`` j         L         L         e         h ``  
``      L         L         L         b      ``  
`` j         L         L         b         h ``  
``      L         b         b         r      ``  
``           b         b         f    |      ``  
``                J         b         |      ``  
``                     t              |      ``  
``                                    /      ``  
``                                   /       ``  
``                                  /        ``  
``         a   b   c   q   e       /         ``  
``       q   a   q   a   a   f    /          ``  
``     q   q   a   a   q   d   g             ``  
``   q   q   q   i   j   e   q   h           ``  
`` q   q   q   h   f   k   d   q   i         ``  
``   q   p   q   g   l   q   q   j           ``  
``     c   o   n   m   q   q   k             ``  
``       q   q   b   q   a   l               ``  
``         p   p   o   n   m                 `` 



We call 'a' a quantum foam particle and '.' a universe.
Values of 'a' particles are represented on 'b' size boards, and can group together
to create 'b' particles. Each level of particle can combine with others of its level
to form the next higher particle, and each level down to 'a' can be reduced
to a board of values one level below its own. Queries on 'a' particles return a
random value.

Multiple universes can be transmitted as a universe matrix #:

example universe matrix containing two universes:

``                     a                     ``  
``                a         a                ``  
``           a         a         9           ``  
``      a         a         9         8      ``  
`` a         a         a         .         a ``  
``      a         9         3         1      ``  
`` a         a         9         0         a ``  
``      a         9         a         a      ``  
`` a         a         .         a         a ``  
``      a         a         8         a      ``  
`` a         a         3         a         a ``  
``      a         a         a         a      ``  
`` a         a         a         a         a ``  
``      a         a         a         a      ``  
``           a         a         a           ``  
``                a         a                ``  
``                     a                     ``  

``aaaaa9aa98aaa.aa931aa90aa9aaaa.aaaa8aaa3aaaaaaaaaaaaaaaaaaaaa``  

example universe:

``                     a                     ``  
``                a         -                ``  
``           a         a         a           ``  
``      a         a         -         -      ``  
`` a         -         -         -         a ``  
``      a         -         -         -      ``  
`` -         -         -         -         - ``  
``      -         9         -         -      ``  
`` -         4         -         -         a ``  
``      -         -         9         -      ``  
`` a         3         9         -         a ``  
``      -         -         -         -      ``  
`` a         -         8         2         a ``  
``      a         -         2         2      ``  
``           -         -         -           ``  
``                a         -                ``  
``                     a                     ``  


size P board (province):

``                     O                     ``  
``                a         a                ``  
``           a         a         a           ``  
``      a         a         a         a      ``  
`` a         O         a         N         a ``  
``      a         N         a         N      ``  
`` N         a         a         L         K ``  
``      a         O         L         K      ``  
`` O         a         N         N         K ``  
``      a         L         a         a      ``  
`` a         L         O         a         O ``  
``      L         K         K         M      ``  
`` a         O         a         O         a ``  
``      a         a         L         a      ``  
``           K         a         a           ``  
``                K         L                ``  
``                     N                     ``  


size I board (insect):


``                     a                     ``  
``                a         a                ``  
``           a         a         a           ``  
``      a         a         a         a      ``  
`` a         G         G         G         a ``  
``      a         G         G         a      ``  
`` a         a         H         a         a ``  
``      a         H         H         a      ``  
`` a         H         H         H         a ``  
``      a         a         a         a      ``  
`` a         a         H         a         a ``  
``      a         H         H         a      ``  
`` a         H         H         H         a ``  
``      a         a         a         a      ``  
``           H         H         H           ``  
``                a         a                ``  
``                     a                     ``  


hanb board sizes increase by a factor of 10 with each character, e.g.

1x10^-10 meters or 1 Angstrom is represented by A  

1x10^-9 meters or 1 nanometer is represented by B  

...  

1x10^-6 meters or 1 micrometer is represented by E

...

1x10^-3 meters or 1 millimeter is represented by H  

1x10^-2 meters or 1 centimeter is represented by I  

...  

1x10^0 meters or 1 meter is represented by K  

and so on up to the maximum size of '.' (1 Rm = 1 x 10^27 m, size of universe) 

hanb is based on our actual universe but is highly simplified to allow for different kinds of
relationships to be tested at different scales without requiring a large amount of storage space

hanb can be used for a variety of modeling applications, including computer games,
social relationship simulations, terrain visualizations and other types of projects requiring
large dynamic spaces in limited memory

`a` -- quantum foam particle  

`b` -- Planck particle (real world Planck length = 1.616255(18) × 10^-35 m) 

`c` -- combined particle 

`d`--   

`e`--   

`f`--  

`g` (1 qm) 

`h` (10 qm) 

`i`  

`j` (1 rm) 

`k` (10 rm) 

`l` (100 rm) 

`m` (1 ym) 

`n` (10 ym) 

`o` (100 ym)  

`p` (1 zm) 

`q` (10 zm) -- smallest observed matter

`r` (100 zm) 

`s` (1 am)  

`t` (10 am) 

`u` (100 am) -- quark (1 x 10^-16 m) 

`v` (1 fm)(classical proton radius= 0.84 femtometers)

`w` (10 fm)(classical electron radius 2.82x10-15 m -> ~6x10^-15 m )

`x` (100 fm) 

`y` (1 pm)

`z`  

`A` -- Angstrom ("atom") (10^−10 meters)  

`B` -- small molecule (1 nm)  

`C` --  

`D` --  

`E` -- 1 µm  

`F` -- 10µm Finehair (human hair bet. 17 and 180 µm diameter)    

`G` -- Graphics pixel (.1 mm = 100 µm)  (.26mm used in CSS so hanb pixel is slightly smaller)

`H` -- "hair" (1 mm)

`I` -- "insect" / coin size (1 cm)  

`J` -- "jar" (10 cm) 

`K` -- person (1 m) 

`L` -- lot (10 m) 

`M` -- hamlet (100 m) 

`N` -- neighborhood (1 km)  

`O` -- borough (10 km) 

`P` -- province (100 km) 

`Q` -- kingdom / continent (small) (1 Mm) 

`R` -- region (large continent) / planet (folded) (10 Mm)  

`S` -- planet-moon system (100 Mm) (light second = 2.998e+8 m)

`T` -- 1 Gm (~ diameter of sun) (3 light seconds)

`U` -- 10 Gm (1 lightminute = 1.799e+10 m)

`V` -- 100 Gm (distance to Venus) 

`W` -- 1 Tm (1 lighthour = 1.079e+12m) 

`X` -- 10 Tm (complete star system) (light day = 2.59e+13)  

`Y` -- 100 Tm (1 light week = 1.813e+14)

`Z` -- 1 Pm (about 1 light month, 7.771e+14)

`0` -- 10^16 m (about 1 lightyear, 9.461e+15 m)  

`1` -- 10^17 (10 lightyears = 9.461e+16 m)

`2` -- 1 Em (100 lightyears = 9.461e+17)

`3` -- 10 Em (1000 lightyears = 9.461e+18) 

`4` -- 100 Em small galaxy (10,000 lightyears = 9.4607e+19)

`5` -- 1 Zm small galaxy (100,000 lightyears = 9.46073e+20m)

`6` -- galaxy (1 million lightyears = 9.46073e+21)  

`7` -- galaxy cluster (~10 million light years)

`8` -- small supercluster 1 Ym (~100 million light years)

`9` -- large supercluster (1 billion light years)

`-` -- filament (10 billion light years)
 
`.` -- universe (1 Rm = 1 x 10^27 m) (100 billion light years)
contained in `#` (matrix) board -- 10 Rm -- (1 trillion light years)

--

@ -- 100 Rm -- (10 trillion light years)
& -- (10^30 m = 1 Qm) (100 trillion light years)

--
# coordinates in hanb  

each hanb board has 61 physical cells and a return value of three characters:
we can reference any of these 64 cells using the hanb character alphabet:

``abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  

therefore we call cell 0 'a', cell 1 'b', etc.
with the return value coded as '9-.'


``                     a                     ``  
``                b         c                ``  
``           d         e         f           ``  
``      g         h         i         j      ``  
`` k         l         m         n         o ``  
``      p         q         r         s      ``  
`` t         u         v         w         x ``  
``      y         z         A         B      ``  
`` C         D         E         F         G ``  
``      H         I         J         K      ``  
`` L         M         N         O         P ``  
``      Q         R         S         T      ``  
`` U         V         W         X         Y ``  
``      Z         0         1         2      ``  
``           3         4         5           ``  
``                6         7                ``  
``                     8                     ``  
``                                         ``  
``                     9-.                   ``  

``         a   b   c   d   e         ``  
``       f   g   h   i   j   k       ``  
``     l   m   n   o   p   q   r     ``  
``   s   t   u   v   w   x   y   z   ``  
`` A   B   C   D   E   F   G   H   I ``  
``   J   K   L   M   N   O   P   Q   ``  
``     R   S   T   U   V   W   X     ``  
``       Y   Z   0   1   2   3       ``  
``         4   5   6   7   8         ``  
``                                 ``  
``                  9-.              ``  
 
therefore in the above board we could reference a cell
through its related character
 
there are 64 levels in hanb, which means a total of
61^64 cells or with 3-character returns included, 64^64 combinations.

``[ total 'a' cells on board ] x [ total b boards on c] x [ total c boards on d ] x ... x [ . boards on # ]``  



Every object can be referenced easily through the unique route to its
specific board, with the string length increasing at each level
of resolution. So the position of a universe within its
corresponding matrix # is a single character representing its position
on the matrix board.

e.g. a universe in position N of a universe matrix is addressed
simply as the string:
``N``  

consider a possible universe in which there is a planet 'R' located
somewhere in a galaxy of size 6:

6 -> 5 -> 4 -> 3 -> 2 -> 1 -> 0 ->Z -> Y -> X -> W -> V -> U -> T -> S -> R

in order to reference its location in the galaxy of size 6 we provide
the cell for its location on that size 6 board:
``K``

``6543210ZYXWVUTS``  
``|||||||||||||||``  
``K``  

then we also provide its position within the size 5 cell returned by the ``K``

``6543210ZYXWVUTS``  
``|||||||||||||||``  
``Ku``  

and on until we get to its position in S

``6543210ZYXWVUTS``  
``|||||||||||||||``  
``Ku3923sderfdewW``  

a complete location of any object within a matrix will be a length inversely
proportional to its size:

. -> - -> 9 -> 8 -> 7 -> 6 -> 5 -> 4 -> 3 -> 2 -> 1 -> 0 ->Z -> Y -> X -> W -> V -> U -> T -> S -> R

``.-9876543210ZYXWVUTS``  
``||||||||||||||||||||``  
``aaaaaKu3923sderfdewW``  

Notice that the location string goes only to the length of the board size and not object size,
as we are referencing the object's position within its board.
Above, we reference the location of an object of size R (planet size) within the hierarchy of
boards above it, all the way to the scale of the universe's position within its universe matrix.

The smallest possible 'a' objects (quantum foam particles) are located
in strings 64 characters long, listing universe cell, supercluster ...
all the way to the position within 'b':

``object container: #.-9876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcb``  
``                   |||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||| ``  
``     object size:  .-9876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba ``  
``                   |||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||| ``  
`` object location:  aaaaaKu3923sderfdewWddfglkhgre0lkheogr8ef978df7vpslfkvjshvkjhffd ``  

Size 'a' quantum particles can not be used as containers, and return a random 3-character string when queried.
and conversely,
Size '#' matrix does not exist in space and therefore does not have a position on a larger board.
The matrix board is the basis for creating space, and contains one or more universes and any other smaller particles.

Consider the following objects with their locations:

Universe:

``object container:  #``  
``                   |``  
``     object size:  .``  
``                   |``  
`` object location:  a``  


City:

``object container:  #.-9876543210ZYXWVUTSRQP``  
``                   ||||||||||||||||||||||||``  
``     object size:  .-9876543210ZYXWVUTSRQPO``  
``                   ||||||||||||||||||||||||``  
`` object location:  aaaaaKu3923sderfdewWddfg``  

Person:

``object container:  #.-9876543210ZYXWVUTSRQPONML``  
``                   ||||||||||||||||||||||||||||``  
``     object size:  .-9876543210ZYXWVUTSRQPONMLK``  
``                   ||||||||||||||||||||||||||||``  
`` object location:  aaaaaKu3923sderfdewWddfglkhg``  

Atom:

``object container:  #.-9876543210ZYXWVUTSRQPONMLKJIHGFEDCB``  
``                   ||||||||||||||||||||||||||||||||||||||``  
``     object size:  .-9876543210ZYXWVUTSRQPONMLKJIHGFEDCBA``  
``                   ||||||||||||||||||||||||||||||||||||||``  
`` object location:  aaaaaKu3923sderfdewWddfglkhgre0lkheogr``  




# situations and events

The basis for movement in hanb is the relationship between situations and events.
A situation is a hanb board that is described in terms of object positions and object types (use one 64-character string for position as above
and a second 64-character string to define object types):

`bdhdjsskjhbsdgyuulskjhddfsghdjjjhgfdhjgkjskjhgdgdhjdkjhgshgdhPPP`
`ddfgljhgelrufhfhfhuurlkjhffhdjklkjheghjdkjhgdf7876dljhfffjjdg111`

An event is the transition from one situation to a different situation in terms of position and/or type, e.g.

`bdhdjsskjhbsdgyuulskjhddfsghdjjjhgfdhjgkjskjhgdgdhjdkjhgshgdhPPP`
`ddfgljhgelrufhfhfhuurlkjhffhdjklkjheghjdkjhgdf7876dljhfffjjdg111`

--->

`HdhdjsskjhbsdgyuulskjhddfsghdjjjhgfdhjgkjskjhgdgdhjdkjhgshgdhPPP`
`ddfgljhgelrufhfhfhuurlkjhffhdjklkjheghjdkjhgdf7876dljhfffjjdg111`

In the above example, the position string has been altered by one character, such that cell 0 has changed from
object size 'b' to object size 'H'



# time in hanb

The transition from one situation to another, called an event, occurs within a duration of time.
As with space, time is represented as a series of scales using the hanb 64-character set:

``abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.`` 

Here, a time duration of 'a' is considered as "ambiguous time" and the unit 'b' is called
a Planck time particle equivalent to the amount of time taken for light to traverse one Planck length (spatial unit 'b').
( 1 x 10^-43 s, real world Planck time = 5.391247×10−44 s)

`a` -- ambiguous time particle  -- (1 x 10^-44 s is below real world Planck time of 5.391247×10−44 s) 

`b` -- Planck time particle -- 1 x 10^-43 s is above real world Planck time of 5.391247×10−44 s

`c` -- 1 x 10^-42 s 

`d` -- 1 x 10^-41 s 

`e` -- 1 x 10^-40 s 

`f` -- 1 x 10^-39 s 

`g` -- 1 x 10^-38 s 

`h` -- 1 x 10^-37 s

`i` -- 1 x 10^-36 s

`j` -- 1 x 10^-35 s

`k` -- 1 x 10^-34 s 

`l` -- 1 x 10^-33 s  

`m` -- 1 x 10^-32 s 

`n` -- 1 x 10^-31 s 

`o`   -- 1 x 10^-30 s

`p`   -- 1 x 10^-29 s 

`q`   -- 1 x 10^-28 s 

`r`   -- 1 x 10^-27 s 

`s`   -- 1 x 10^-26 s 

`t`   -- 1 x 10^-25 s 

`u`   -- 1 x 10^-24 s  

`v`   -- 1 x 10^-23 s 

`w`   -- 1 x 10^-22 s 

`x`   -- 1 x 10^-21 s 

`y`   -- 1 x 10^-20 s 

`z` -- 1 x 10^-19 s --  

`A` -- 1 x 10^-18 s --    

`B` -- 1 x 10^-17 s --  

`C` -- 1 x 10^-16 s --  

`D` -- 1 x 10^-15 s -- 

`E` -- 1 x 10^-14 s -- 

`F` -- 1 x 10^-13 s --   

`G` -- 1 x 10^-12 s --   

`H` -- 1 x 10^-11 s --  

`I` -- 1 x 10^-10 s --  

`J` -- 1 x 10^-9 s --   ( 1 x10^-8 sec = 1 nanosecond )  -- (2nd alternate: 1 x 10^24 sec) 

`K` -- 1 x 10^-8 s -- ( 1 x10^-8 sec ) 

`L` -- 1 x 10^-7 s -- ( 1 x10^-7 sec ) 

`M` -- 1 x 10^-6 s -- ( 1 x10^-6 sec = 1 microsecond )

`N` -- 1 x 10^-5 s -- ( 1 x10^-5 sec )

`O` -- 1 x 10^-4 s -- ( 1 x10^-4 sec )

`P` -- 1 x 10^-3 s -- ( 1 x10^-3 sec = 1 millisecond )

`Q` -- 1 x 10^-2 s -- ( 1 x10^-2 sec ) 

`R` -- 1 x 10^-1 s -- ( 1 x10^-1 sec )  

`S` -- 1 second ( 1 x10^0 sec )  

`T` -- 10 seconds ( 1 x10^1 sec) 

`U` -- 100 seconds ( 1 x10^2 sec = 1.67 minutes) 

`V` -- 1000 seconds ( 1 x10^3 sec = 16 minutes)  

`W` -- 10,000 seconds ( 1 x10^4 sec = 2.7 hrs)  

`X` -- 100,000 seconds ( 1 x10^5 sec = 1 day)  

`Y` -- 1,000,000 seconds ( 1 x10^6 sec = 11 days) 

`Z` -- 10 million seconds ( 1 x10^7 sec = 115 days) 

`0` -- 100 million ( 1 x10^8 sec = 1157 days = 3 yrs)  

`1` -- 1 billion seconds ( 1 x10^9 sec = 31 yrs )   

`2` -- 10 billion seconds ( 1 x10^10 sec = 317 yrs)  

`3` -- 100 billion seconds ( 1 x10^11 sec = 3170 yrs)  

`4` -- 1 trillion seconds ( 1 x10^12 sec = 31700 yrs)   

`5` -- 10 trillion seconds ( 1 x10^13 sec = 317097 yrs)    

`6` -- 100 trillion seconds ( 1 x10^14 sec = 3.17 million yrs)  

`7` -- 1 quadrillion seconds ( 1 x10^15 sec = 31.7 million years)  

`8` -- 10 quadrillion seconds ( 1 x10^16 sec = 317 million years) 

`9` -- 100 quadrillion seconds ( 1 x10^17 sec = 3.17 billion yrs) 

`-` -- 1 quintillion seconds ( 1 x10^18 sec = 31.7 billion years)   

`.` -- 10 quintillion seconds ( 1 x10^19 sec = 317 billion years)   


example times (additive):  

``S``  one second

``UT`` 110 seconds 

``VU`` 1100 seconds 

``.S`` 10 quintillion and one seconds  

``.SS`` 10 quintillion and two seconds  

``.SSSSSSSSS`` 10 quintillion and nine seconds  

``.T`` 10 quintillion and 10 seconds  

``..`` 10 quintillion + 10 quintillion seconds = 600 billion yrs  

``...`` 30 quintillion seconds = 1 trillion years
`` = 3 seconds x 10^18 ``

example times (multiplicative):  

``:S``  one second

``:T``  10 seconds

``:T:U``  one second plus 1 second

``:UT`` 100 seconds x 10 seconds 

``:VU`` 1000 x 100 seconds 

``:.S`` 10 quintillion x one seconds  

``:.T`` 10 quintillion x 110 seconds 

``:..`` 10 quintillion x 10 quintillion seconds  

full clock:
``a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a``  
ambiguous time (no event has occurred previously)

``a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:b``  
one Planck time unit (first possible event)

``a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:S``  
one second

``a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:.``  
317 billion years

``----:99999:888888:777777:666666:55555:W:V:U:T:S``  
4,320432x10^17 seconds = 13.7 billion years

``.:.:.``  
317 billion yrs ^3 + 317 billion yrs ^2 + 317 billion yrs ^1 


``.:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:S``  
317 billion yrs ^64 + 1 second 

``.:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:SS``  
317 billion yrs ^64 + 2 seconds 

``.:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:SSb``  
317 billion yrs ^64 + 2 seconds + 1 Planck time unit

``.:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:SSSSSSSSSS``  
317 billion yrs ^64 + 10 seconds 

``.:a:a:a:.:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:.:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:S``  
317 billion yrs ^64 + 317 billion yrs ^60 + 317 billion yrs ^41 + 1 second 

``.........``  
2.853 trillion years


``.........TS``
2.853 trillion years + 10 seconds + 1 second


floating clock (64 chars):
``.:-:999:8:7:6:5:444444:3:2:111:0:Z:YYY:X:WWW:VVV:UUU:TTT:SSSSSSS``
``.:-:999:8:7:6:5:UUU:TTT:SSSSSSS:RRR:QQ:PPPPP:OOOOO:NNNNNN:MMMMMM``

``.:a:a:a:a:a:a:a:TTTTTTT:S:a:a:PPPPPPPPPP:OOOOO:NNNNNN:MMMMMMMMMM``

``a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a:a``  



``-10ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba``  

``-10ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba``  

``-10ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba``  

hanb condensed 64 character time string:
``aaabbb....BBBBfgggghhhhhHHHHH45yy5555555554444455555555``  


# examples of location descriptions with situations, events and time durations:

Atom:

`` object container:  #.-9876443210ZYXWVUTSRQPONMLKJIHGFEDCB``  
``                    ||||||||||||||||||||||||||||||||||||||``  
``      object size:  .-9876443210ZYXWVUTSRQPONMLKJIHGFEDCBA``  
``                    ||||||||||||||||||||||||||||||||||||||``  
``  object location:  aaaaaKu3923sderfdewWddfglkhgre0lkheogr``  
``    time duration:  c``  
`` object situation:  ahbkjhffddlkhueuusgdghejkdjkgsdhsjhgsd``   
``    event outcome:  ahbkjhffddekhueuusedghejkdjkgsdhejhgsd`` 




# hanb replacement language (in progress)
describe objects at any resolution, combine various resolutions in any way

examples:  

apple  -- size J  
-- contains high detail sections of size I  
-- some I sections have very high detail sections size H  
-- some of those sections have pixel detail sections size G  

generating apple:  
-- generate J (apple)  

board='J' 

-- replace J with 61 I cells  

J -> dUp5R4nvFuBGHUDlMg1JyX8ID8H5Znwo82RoF0myOcUhjRjwORaoIJCcZnEsG

-- replace I cells with H cells

board='dUp5R4nvFuBGHUDlMg1JyX8ID8H5Znwo82RoF0myOcUhjRjwORaoIJCcZnEsG'  
for char in board  
  char -> lookup(char)

-- replace H cells with G cells (pixels)  
for char in board  
  char -> lookup(char)

-G returns RGB

automobile  -- size K  
-- steering wheel size J  
-- tire size J  
-- hood ornament size I  

city  -- size O  
continent  -- size Q  
planet  -- size R  

# 16 truth tables

`` all possible logical relationships between boolean values A and B ``  

``A B | R     A B | R     A B | R     A B | R ``  
``----|---    ----|---    ----|---    ----|---``  
``0 0 | 0     0 0 | 0     0 0 | 0     0 0 | 0 ``  
``0 1 | 0     0 1 | 0     0 1 | 0     0 1 | 0 ``  
``1 0 | 0     1 0 | 0     1 0 | 1     1 0 | 1 ``  
``1 1 | 0     1 1 | 1     1 1 | 0     1 1 | 1 ``  
`` ``  
``A B | R     A B | R     A B | R     A B | R ``  
``----|---    ----|---    ----|---    ----|---``  
``0 0 | 0     0 0 | 0     0 0 | 0     0 0 | 0 ``  
``0 1 | 1     0 1 | 1     0 1 | 1     0 1 | 1 ``  
``1 0 | 0     1 0 | 0     1 0 | 1     1 0 | 1 ``  
``1 1 | 0     1 1 | 1     1 1 | 0     1 1 | 1 ``  
`` ``  
``A B | R     A B | R     A B | R     A B | R ``  
``----|---    ----|---    ----|---    ----|---``  
``0 0 | 1     0 0 | 1     0 0 | 1     0 0 | 1 ``  
``0 1 | 0     0 1 | 0     0 1 | 0     0 1 | 0 ``  
``1 0 | 0     1 0 | 0     1 0 | 1     1 0 | 1 ``  
``1 1 | 0     1 1 | 1     1 1 | 0     1 1 | 1 ``  
`` ``  
``A B | R     A B | R     A B | R     A B | R ``  
``----|---    ----|---    ----|---    ----|---``  
``0 0 | 1     0 0 | 1     0 0 | 1     0 0 | 1 ``  
``0 1 | 1     0 1 | 1     0 1 | 1     0 1 | 1 ``  
``1 0 | 0     1 0 | 0     1 0 | 1     1 0 | 1 ``  
``1 1 | 0     1 1 | 1     1 1 | 0     1 1 | 1 ``  

Since the inputs are always the same, the tables differ only in the single bit
output they have for each input pair:


``| R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R ``  
``|--- |--- |--- |--- |--  |--- |--- |--- |--- |--- |--- |--- |--  |--- |--- |---``  
``| 0  | 0  | 0  | 0  | 0  | 0  | 0  | 0  | 1  | 1  | 1  | 1  | 1  | 1  | 1  | 1 ``  
``| 0  | 0  | 0  | 0  | 1  | 1  | 1  | 1  | 0  | 0  | 0  | 0  | 1  | 1  | 1  | 1 ``  
``| 0  | 0  | 1  | 1  | 0  | 0  | 1  | 1  | 0  | 0  | 1  | 1  | 0  | 0  | 1  | 1 ``  
``| 0  | 1  | 0  | 1  | 0  | 1  | 0  | 1  | 0  | 1  | 0  | 1  | 0  | 1  | 0  | 1 `` 

Output tables can be combined to form logic filters:

``| R  | R  | R  | R  | R  | R  ``  
``|--- |--- |--- |--- |--  |--- ``  
``| 0  | 0  | 0  | 0  | 1  | 0  ``  
``| 0  | 0  | 1  | 1  | 1  | 0  ``  
``| 0  | 0  | 1  | 1  | 1  | 0  ``  
``| 1  | 1  | 1  | 0  | 0  | 1  `` 

AND, AND, OR, XOR, NAND, AND


# 64 character truth tables

each hanb character string is 64 characters long


a truth table operating on 2 boolean values contains 4 output values (2^2)

a truth table operating on 2 boolean values contains 16 possible output values (2^(2^2))

a truth table operating on 6 boolean values contains 64 output values (2^6)

a truth table operating on 6 boolean values contains 1.845 x 10^19 possible output values 2^(2^6)


``A B C D E F | R ``  
``------------|---``  
``0 0 0 0 0 0 | 0 ``  
``0 0 0 0 0 1 | 0 ``  
``0 0 0 0 1 0 | 0 ``  
``0 0 0 0 1 1 | 0 ``  
``...             ``  
``1 1 1 1 0 1 | 0 ``  
``1 1 1 1 1 0 | 0 ``  
``1 1 1 1 1 1 | 0 ``  

these values can be rewritten as a 64-digit string: 0000000000000000000000000000000000000000000000000000000000000000 

6 strings combined can be written as sequences of 6 bits: {0,0,0,0,0,0}, {0,0,0,0,0,0}, {0,0,0,0,0,0}, ... {0,0,0,0,0,0} 

a sequence of 6 bits can be represented by a single hanb character:

``0   000000  a    1   000001  b    2   000010  c    3   000011  d ``  
``4   000100  e    5   000101  f    6   000110  g    7   000111  h ``  
``8   001000  i    9   001001  j   10   001010  k   11   001011  l ``  
``12  001100  m   13   001101  n   14   001110  o   15   001111  p ``  
``16  010000  q   17   010001  r   18   010010  s   19   010011  t ``  
``20  010100  u   21   010101  v   22   010110  w   23   010111  x ``  
``24  011000  y   25   011001  z   26   011010  A   27   011011  B ``  
``28  011100  C   29   011101  D   30   011110  E   31   011111  F ``  
``32  100000  G   33   100001  H   34   100010  I   35   100011  J ``  
``36  100100  K   37   100101  L   38   100110  M   39   100111  N ``  
``40  101000  O   41   101001  P   42   101010  Q   43   101011  R ``  
``44  101100  S   45   101101  T   46   101110  U   47   101111  V ``  
``48  110000  W   49   110001  X   50   110010  Y   51   110011  Z ``  
``52  110100  0   53   110101  1   54   110110  2   55   110111 56 ``  
``56  111000 57   57   111001  5   58   111010  6   59   111011  7 ``  
``60  111100  8   61   111101  9   62   111110  -   63   111111  . ``  

therefore each hanb character can represent 6 separate function outputs in a string of 64 characters:

``Asd45dldkjhgkdljhw44hsh444dsSSWGGTTEEE55598989jlcFFFFFsflksjflkj``  

each of these 6 separate function outputs can be applied to a specific bit of an input character,
allowing for 64^64 different character filters

these filters take in one hanb character and output one hanb character

the form of the truth table can be reduced to 64-character form as:


``I | R ``  
``--|---``  
``a | a ``  
``b | a ``  
``c | a ``  
``d | a ``  
``...   ``  
``9 | a ``  
``- | a ``  
``. | a ``  

the truth tables and character mappings are therefore interchangeable

a table can be created of size 64 x 64 representing the output character for any two hanb input characters

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  


Within each 64x64 = 4096 table there are 64 possibilities for each character,

4096x64=262,144 which is the total number of possible unique two character functions

(compare with boolean 16 total functions)

For each of the 16 boolean functions there should be one or more analogous hanb functions:

"identity function"?

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b abaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aacaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaadaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaeaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaafaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaagaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaahaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaajaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaakaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaalaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaamaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaanaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaoaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaapaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaqaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaraaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaasaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaataaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaauaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaavaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaawaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaxaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaayaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaazaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaAaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaBaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaCaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaDaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaEaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaFaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaGaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaHaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaIaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaJaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaKaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaLaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaMaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaNaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaOaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaPaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaQaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaRaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaSaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaTaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaUaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaVaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaWaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaXaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaYaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaZaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0aaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1aaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2aaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa3aaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4aaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5aaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa6aaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa7aaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa8aaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa9aa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-a``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.`` 


```
0 null (false)
1 AND
2 inhibition A AND NOT-B
3 transfer (A)
4 inhibition B AND NOT-A
5 transfer (B)
6 XOR
7 OR
8 NOR
9 equivalence (XNOR), iff A then B
10 NOT-B
11 implication (if B then A)
12 NOT-A
13 implication (if A then B)
14 NAND
15 identity (true)
```


``| R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R  | R ``  
``|--- |--- |--- |--- |--  |--- |--- |--- |--- |--- |--- |--- |--  |--- |--- |---``  
``| a  | a  | a  | a  | a  | a  | a  | a  | .  | .  | .  | .  | .  | .  | .  | . ``  
``| a  | a  | a  | a  | .  | .  | .  | .  | a  | a  | a  | a  | .  | .  | .  | . ``  
``| a  | a  | .  | .  | a  | a  | .  | .  | a  | a  | .  | .  | a  | a  | .  | . ``  
``| a  | .  | a  | .  | a  | .  | a  | .  | a  | .  | a  | .  | a  | .  | a  | . `` 

``| R  | ``  
``|--- | ``  
``| a  | ``  
``| a  | ``  
``| a  | ``  
``| a  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``   



0 

null (false)

``| R  | ``  
``|--- | ``  
``| a  | ``  
``| a  | ``  
``| a  | ``  
``| a  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  


1 

(incomplete)

AND

``| R  | ``  
``|--- | ``  
``| a  | ``  
``| a  | ``  
``| a  | ``  
``| .  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb``  
`` c aacccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc``  
`` d aaaddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd``  
`` e aaaaeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee``  
`` f aaaaafffffffffffffffffffffffffffffffffffffffffffffffffffffffffff``  
`` g aaaaaagggggggggggggggggggggggggggggggggggggggggggggggggggggggggg``  
`` h aaaaaaahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh``  
`` i aaaaaaaaiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaai``  
`` j aaaaaaaaajaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaj``  
`` k aaaaaaaaaakaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaak``  
`` l aaaaaaaaaaalaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaal``  
`` m aaaaaaaaaaaamaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaam``  
`` n aaaaaaaaaaaaanaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan``  
`` o aaaaaaaaaaaaaaoaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaao``  
`` p aaaaaaaaaaaaaaapaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaap``  
`` q aaaaaaaaaaaaaaaaqaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaq``  
`` r aaaaaaaaaaaaaaaaaraaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaar``  
`` s aaaaaaaaaaaaaaaaaasaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaas``  
`` t aaaaaaaaaaaaaaaaaaataaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaat``  
`` u aaaaaaaaaaaaaaaaaaaauaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaau``  
`` v aaaaaaaaaaaaaaaaaaaaavaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaav``  
`` w aaaaaaaaaaaaaaaaaaaaaawaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaw``  
`` x aaaaaaaaaaaaaaaaaaaaaaaxaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaax``  
`` y aaaaaaaaaaaaaaaaaaaaaaaayaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaazaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaz``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaAaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaBaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaB``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaCaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaC``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaDaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaD``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaEaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaE``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaFaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaF``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaGaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaG``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaHaaaaaaaaaaaaaaaaaaaaaaaaaaaaaH``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaIaaaaaaaaaaaaaaaaaaaaaaaaaaaaI``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaJaaaaaaaaaaaaaaaaaaaaaaaaaaaJ``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaKaaaaaaaaaaaaaaaaaaaaaaaaaaK``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaLLLLLLLLLLLLLLLLLLLLLLLLLLL``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaMMMMMMMMMMMMMMMMMMMMMMMMMM``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaNNNNNNNNNNNNNNNNNNNNNNNNN``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaOOOOOOOOOOOOOOOOOOOOOOOO``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaPPPPPPPPPPPPPPPPPPPPPPP``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaQQQQQQQQQQQQQQQQQQQQQQ``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaRRRRRRRRRRRRRRRRRRRRR``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaSSSSSSSSSSSSSSSSSSSS``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaTTTTTTTTTTTTTTTTTTT``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaUUUUUUUUUUUUUUUUUU``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaVVVVVVVVVVVVVVVVV``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaWWWWWWWWWWWWWWWW``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaXXXXXXXXXXXXXXX``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaYYYYYYYYYYYYYY``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaZZZZZZZZZZZZZ``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000000000000``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa11111111111``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa2222222222``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa333333333``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa44444444``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa5555555``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa666666``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa77777``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa8888``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa999``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa--``  
`` . abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  


(incomplete)

2 inhibition A AND NOT-B

``| R  | ``  
``|--- | ``  
``| a  | ``  
``| a  | ``  
``| .  | ``  
``| a  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaba``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaacaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaadaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaeaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaafaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaagaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaahaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaiaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaajaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaakaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaalaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaamaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaanaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaoaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaapaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaqaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaraaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaasaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaataaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaauaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaavaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaawaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaxaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaayaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaazaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaAaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaBaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaCaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaDaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaEaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaFaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaGaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaHaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaIaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaJaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaKaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaLaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaMaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaNaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaOaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaPaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaQaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaRaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaSaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaTaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaUaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaVaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaWaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaXaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaYaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaZaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaa0aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaa1aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaa2aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaa3aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaa4aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaa5aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaa6aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaa7aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcbaaaa``  
`` 9 9876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcbaaa``  
`` - -9876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcbaa``  
`` . .-9876543210ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba``  

3 transfer (A)

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 1  | ``  
``| 1  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`` 

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``

``| R  | ``  
``|--- | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``  
``| 0  | ``

``   abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.``  
`` a aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` b aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` c aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` d aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` e aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` f aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` g aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` h aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` i aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` j aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` k aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` l aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` m aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` n aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` o aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` p aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` r aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` s aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` t aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` u aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` v aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` w aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` x aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` A aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` B aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` C aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` D aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` E aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` F aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` G aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` H aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` I aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` J aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` K aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` L aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` M aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` N aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` O aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` P aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Q aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` R aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` S aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` T aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` U aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` V aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` W aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` X aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Y aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` Z aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 0 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 2 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 3 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 5 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 6 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 7 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 8 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` 9 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` - aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa``  
`` . aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`` 

# Implementing hanb
# physical

# digital

hanb boards can be transmitted in various ways
and do not require receipt of complete message
in order to be utilized

# hanb text mode

classic hanb -- easy to implement on any device
with text capabilities, including screens and typewriters

# hanb graphics

--

# unicode hanb

``⬛️⬛️❄️⬛️⬛️⬛️⬛️⬛️⬛️⬛️❄️〰️❄️〰️❄️〰️🎅🏽〰️✨⬛️⬛️⬛️⬛️⬛️⬛️⬛️⬛❄️⬛️⬛️⬛️``  
``⬛️❄️❄️❄️⬛️⬛⬛️⬛️⬛️❄️〰️❄️〰️☃️〰️✨〰️✨〰️✨⬛️⬛️⬛️⬛️⬛️❄️❄️🐈❄️❄️⬛️``  
``❄️❄️💢❄️❄️⬛️⬛️⬛️❄️〰️🐈〰️❄️〰️❄️〰️🏘️〰️☃️〰️✨⬛️⬛️⬛️❄️✨✨🎁✨✨❄️``  
``❄️❄️❄️❄️❄️⬛️⬛️❄️〰️❄️〰️❄️〰️✨〰️🏘️〰️❄️🍋✨〰️✨⬛️⬛️❄️🎁🏘️🎁✨✨❄️``  
``❄️❄️❄️❄️❄️⬛️❄️〰️❄️〰️🏢〰️✨〰️🏚️〰️🏚️〰️❄️〰️✨〰️✨⬛️❄️✨🎁🎁✨✨❄️``  
``⬛️❄️❄️💢⬛️⬛️⬛️❄️〰️🏢〰️✨〰️🏚️〰️🏚️〰️❄️〰️✨〰️✨⬛️⬛️⬛️❄️❄️❄️❄️❄️⬛``  
``⬛️⬛️❄️⬛️⬛️⬛️⬛️⬛️❄️〰️🏘️〰️🏚️〰️🏚️〰️🎁〰️🎁〰️✨⬛️↖️⬛️↗️⬛️⬛️❄️⬛️⬛️⬛️``  
``⬛️⬛️⬛️⬛️⬛️⬛️⬛️⬛️⬛️❄️〰️❄️〰️🏚️〰️🎁〰️🎁〰️🎁⬛️⬛️⬅️2️⃣➡️⬛️⬛️⬛️⬛️⬛️⬛️``  
``⬛️⬛️⬛️⬛️⬛️⬛️⬛️⬛️⬛️⬛️❄️〰️❄️〰️🏚️〰️🐁〰️🎁⬛️⬛️⬛️↙️⬛️↘️⬛️⬛️⬛️⬛️⬛️⬛️``


sample character sets for each board size:

`` a  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ``  
`` b  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ``  
`` c  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ``  
`` d  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ``  
`` e  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ``  
`` f  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ``  
`` g  -- abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ `` 

`` F  -- abcdefghijklmnopqrstuvwxyz🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠🦠QRSTUVWXYZ ``  

`` L  -- 🌱🪴🌲🌳🌴🌵🌾🌿☘️🍀🍁🍂🍃🌷🌼🌻🌺🥀🌹🏵️🪷💮🌸💐🚗🚗🚗🚌🚓🏍️🚗🚗🚗🚗🚗🚗🦠🪱🪰🦟🦂🕸️🕷️🪳🦗🐞🪲🐝🐜🐛🦋🐌 ``  
`` M  -- 🚗🚌🚓🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚌🚓🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗 ``  
`` N  -- 🚗🚌🚓🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚌🚓🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗🚗 ``  
`` O  -- 🧱🪨🪵🏛️🛖🏘️🏚️🏠🏡🏢🏣🏤🏥🏦🏫🏪🏨🏩🏗️🏟️🏬🏭🏰🏯⛪🛕💒🗼🗽🧱🪨🪵🏛️🛖🏘️🏚️🏠🏡🏢🏣🏤🏥🏦🏫🏪🏨🏩🏗️🏟️🏬🏭🏰🏯⛪🛕💒🗼🗽 ``   



# genetic hanb

coming soon


# quantum hanb

coming soon using IBM Quantum

# storage benefits  

one board is 61 cells x 1 byte / cell = 61 bytes  
exponent of level of resolution

board size L at resolution  
0: 61^0 = 1 byte (component of larger board M)  
1: 61^1 = 61 bytes (L with K details)  
2: 61^2 = 3721 bytes (L with J details)  
3: 61^3 = 226981 bytes (L with I details)  
4: 61^4 = 13.8 MB (L with H details)  
5: 61^5 = 845 MB (L with G details, pixel level detail)  

hanb translation palettes save memory:  

L -> KKKKKKKKKKKKKKKKKKK  
K -> JJJJJJJJJJJJJJJJJJJ  
J -> IIIIIIIIIIIIIIIIIII  

hanb
translation palette 0 (M cell -> 61 L cells) -- 3904 bytes (61 cell maps for each of 64 possible characters)  

translation palette 1 (L cell -> 61 K cells) -- 3904 bytes   
translation palette 2 (K cell -> 61 J cells) -- 3904 bytes   
translation palette 3 (J cell -> 61 I cells) -- 3904 bytes   
translation palette 4 (I cell -> 61 H cells) -- 3904 bytes   
translation palette 5 (H cell -> 61 G cells) -- 3904 bytes     

3904 x 5 = 19520 bytes    

# 64-character strings

using 64 bytes for each cell (reserve 3 additional bytes for RGB color information):  
64^2 = 4096    
4096 x 5 = 20480 bytes   

A 3-character return value is appended to each 61-character string to allow for a base case
in situations that do not allow a board to be resolved fully:

`AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA555`

In the above example, the string of A characters represents the cell values, and the string "555" at the end is a return value.
In this way an RGB color value or other types on information can be returned with each board as an alternative to displaying the board.
 

# Use cases
# simple rpg example

generate scope replacement palettes a-z, A-Z, 0-9, - .

a -> jHtsb3VjW2bLgToIqvMyJSiz2dvYKT4MOayOO8HjWQ8Jzvnm1YzWbA3CvEoA1  
b -> pwc0SwumO0nTW9aHCtyRU8PZBDUwDYWFeEbsuFfw5Bn8W6dl8xmuGAdzYwHpx  

...

9 -> GCYLqcob3AXPStlz2oaPNvcUPz35ZTJaX506vtYZQkVKTC8s2E2AsERZDF0qr  
_ -> 6weMksPXE7kyap1Q8wxij0ttLn6gHSu9n01k0hENnCCrHf7Zs0Xy9srvnzz6b  
. -> HWwpihtNApIJwZ2wZakSWo2K9WjsxvSnlFCGfamR3GsVeie9Eo3eQhvuBoIUP  


generate character K  
wZcgQZaWrht6a6rEWLLbcxGbS6rjUPFKBtLrRt8eLNlzxJf0WNiQ5LeIWOXrc

``🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁``  
``🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁``  
``🍁🍁🌿🍁🌿🍁🌿🍁📜🍁🌿🍁🌿🍁🌿🍁🍁``  
``🍁🌿🍁🌿🍁🌿🍁📜🍁📜🍁🌿🍁🌿🍁🌿🍁``  
``🌿🍁🌿🍁🌿🍁🌿🍁🤵‍♂️🍁🧤🍁🌿🍁🌿🍁🌿``  
``🍁🌿🍁🌿🍁🧤🍁📜🍁📜🍁🌿🍁🌿🍁🌿🍁``  
``🍁🍁🌿🍁🌿🍁🌿🍁📜🍁📜🍁🌿🍁🌿🍁🍁``  
``🍁🍁🍁🌿🍁🌿🍁📜🍁📜🍁🌿🍁🌿🍁🍁🍁``  
``🍁🍁🍁🍁🌿🍁🥾🍁📜🍁🥾🍁🌿🍁🍁🍁🍁``

generate weapon J  
S5pl9pE1I37bVazv52AyZiJCnBzLeRwpelSv28oeo9MVHG7zKvnhWBi2ZoHfv

``🍁🍁🍁🍁🌿🍁🌿🍁⏸🍁🌿🍁🌿🍁🍁🍁🍁``  
``🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁``  
``🍁🍁🌿🍁🌿🍁🌿🍁⏸🍁🌿🍁🌿🍁🌿🍁🍁``  
``🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁``  
``🌿🍁🌿🍁🌿🍁🌿🍁⏸🍁🌿🍁🌿🍁🌿🍁🌿``  
``🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁``  
``🍁🍁🌿🍁🌿🍁💠🍁🔴🍁💠🍁🌿🍁🌿🍁🍁``  
``🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁``  
``🍁🍁🍁🍁🌿🍁🌿🍁💎🍁🌿🍁🌿🍁🍁🍁🍁``


generate magic ring I  
``aaHaaaaGGaaaaFFFaaaaFaaFaaaaFaaaFaaaaFaaFaaaaFaFaaaaFFaaaaaaa``  

``🍁🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁💠🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🔸🍁🔸🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌀🍁🌀🍁🌀🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🌿🍁🌿🍁🌀🍁🌿🍁🌿🍁🌀🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🌿🍁🌿🍁🌀🍁🌿🍁🌿🍁🌿🍁🌀🍁🌿🍁🌿🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🌿🍁🌿🍁🌀🍁🌿🍁🌿🍁🌀🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌀🍁🌿🍁🌀🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌀🍁🌀🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁🍁``   

generate
closely inspect H  
6J50bQJ0CRr68KkPcS8ig5H2S0fVG5iZyCuICJVzBfOVyWrCESQKov4gXDotP

-----

simulated walking map L  
aaaaaKaaaaaKKaaKKaaaaaKaaaaaaaKaaaJaaaaaaaaaaaJaaaaJaaJaaaaaa   
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗K.a.a.a.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗K.K.a.a.K.K.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.K.a.a.a.▗▗▗▗▗``  
``▗▗▗▗a.a.a.a.K.a.a.a.J.▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.a.a.a.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.J.a.a.a.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.J.a.a.J.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated hamlet overmap M  
aaaaaaaaKaaaKaaaLaaaaLaaaaaaaaaaaLaaaLaaaaaaaaaaLaaKaKaaaaaaa  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.K.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.K.a.a.a.L.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.a.L.a.a.a.a.▗▗▗▗▗``  
``▗▗▗▗a.a.a.a.a.a.a.L.a.▗▗▗▗``  
``▗▗▗▗▗a.a.L.a.a.a.a.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.a.a.L.a.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.K.a.K.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated village quest map N  
aaaaaaaaaaaaaaMaMaaaaaaaaaaaaaMaaaaaaaaaaMaaaLaaaLaaaMaaaaaaa  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.a.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.M.a.M.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.a.a.a.a.▗▗▗▗▗``  
``▗▗▗▗a.a.a.a.M.a.a.a.a.▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.a.a.M.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.L.a.a.a.L.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.M.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

``🍁🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🏘️🍁🌿🍁🏘️🍁🌿🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🏘️🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🏘️🍁🌿🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🏠🍁🌿🍁🌿🍁🌿🍁🏠🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🏘️🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁``    
``🍁🍁🍁🍁🍁🍁🍁🍁🌿🍁🌿🍁🌿🍁🌿🍁🌿🍁🍁🍁🍁🍁🍁🍁🍁🍁``   

simulated city map O  
aaaaaAAAANaaaaaNaaaaKMaKAaaaaaNaaKaaaLLaAAAAAMAAMAAAALLAAAAAA  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.N.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.a.N.a.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.K.M.a.K.A.a.▗▗▗▗▗``  
``▗▗▗▗a.a.a.a.N.a.a.K.a.▗▗▗▗``  
``▗▗▗▗▗a.a.L.L.a.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.M.A.A.M.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.L.L.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗`` 

simulated kingdom P  
aaaaaaAAMOKaaKaAAaaONNaaOaaAANOMaAaaaaaKaaaaaMAAMaaLAAOaaaaaa  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.A.A.M.O.K.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.K.a.A.A.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.O.N.N.a.a.O.a.▗▗▗▗▗``  
``▗▗▗▗a.A.A.N.O.M.a.A.a.▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.K.a.a.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.M.A.A.M.a.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.L.A.A.O.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated continent Q  
AAAAAAAAAPAAAAPAAAAAAAAAAAAAAPAAAAAAAAPAAPAAAAAAAAAAAAAAAAAA  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.P.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.P.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗A.A.A.P.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.P.A.A.P.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗`` 

simulated planet R  
AAAAAAAAQAAAQAQQAAAQAAAQAAAAAAAAAAAAAQQQAAAAAAQAAAAAAAAAAAAA  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.Q.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.Q.A.Q.Q.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.Q.A.A.A.Q.A.▗▗▗▗▗``  
``▗▗▗▗A.A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.Q.Q.Q.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.Q.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗`` 



# tourist map of city
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.L.M.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.N.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗K.K.K.K.K.K.K.K.▗▗▗▗▗``  
``▗▗▗▗A.A.M.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗a.a.L.A.L.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.M.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.N.N.L.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  

#
# model of Solar System  
``▗▗▗▗▗▗▗▗A.A.A.A.R.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.R.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.Q.A.A.R.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.R.A.A.A.▗▗▗▗▗``  
``▗▗▗▗A.A.A.A.X.A.A.A.R.▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.R.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.Q.A.A.A.▗▗▗▗▗▗▗▗``  

# model of Milky Way  (5 × 10^20 m radius)
``▗▗▗▗▗▗▗▗-.3.3.-.-.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗-.3.-.-.3.3.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗-.3.-.3.3.-.-.▗▗▗▗▗▗``  
``▗▗▗▗▗-.-.3.3.3.-.3.-.▗▗▗▗▗``  
``▗▗▗▗-.3.-.3.3.3.3.-.-.▗▗▗▗``  
``▗▗▗▗▗-.3.3.3.3.3.-.-.▗▗▗▗▗``  
``▗▗▗▗▗▗-.-.-.-.-.3.-.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗-.-.-.3.-.-.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗-.3.-.-.-.▗▗▗▗▗▗▗▗``  

``✨✨✨✨✨✨✨✨🌌✨🌟✨🌟✨🌌✨🌌✨✨✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨✨🌌✨🌟✨🌌✨🌌✨🌟✨🌟✨✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨🌌✨🌟✨🌌✨🌟✨🌟✨🌌✨🌌✨✨✨✨✨✨✨``  
``✨✨✨✨✨🌌✨🌌✨🌟✨🌟✨🌟✨🌌✨🌟✨🌌✨✨✨✨✨✨``  
``✨✨✨✨🌌✨🌟✨🌌✨🌟✨🌟✨🌟✨🌟✨🌌✨🌌✨✨✨✨✨``  
``✨✨✨✨✨🌌✨🌟✨🌟✨🌟✨🌟✨🌟✨🌌✨🌌✨✨✨✨✨✨``  
``✨✨✨✨✨✨🌌✨🌌✨🌌✨🌌✨🌌✨🌟✨🌌✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨✨🌌✨🌌✨🌌✨🌟✨🌌✨🌌✨✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨✨✨🌌✨🌟✨🌌✨🌌✨🌌✨✨✨✨✨✨✨✨✨``  

# model of Lithium atom (radius 182pm -> 1 Angstrom = 1pm so 100A -> B+) 
``▗▗▗▗▗▗▗▗-.-.-.-.-.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗-.-.-.-.-.-.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗-.-.-.-.-.-.-.▗▗▗▗▗▗``  
``▗▗▗▗▗-.-.-.v.v.-.-.-.▗▗▗▗▗``  
``▗▗▗▗-.v.-.v.v.v.-.v.-.▗▗▗▗``  
``▗▗▗▗▗-.-.-.v.v.-.-.-.▗▗▗▗▗``  
``▗▗▗▗▗▗-.-.-.-.-.-.-.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗-.-.-.-.-.-.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗-.-.v.-.-.▗▗▗▗▗▗▗▗``  

``🥛🥛🥛🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛✨🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛🎁🥛🎁🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛✨🥛🎁🥛✨🥛🎁🥛🎁🥛🎁🥛✨🥛🎁🥛✨🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛🎁🥛🎁🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛✨🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛🥛🥛✨🥛✨🥛✨🥛✨🥛✨🥛✨🥛🥛🥛🥛🥛🥛🥛🥛``   
``🥛🥛🥛🥛🥛🥛🥛🥛✨🥛✨🥛🎁🥛✨🥛✨🥛🥛🥛🥛🥛🥛🥛🥛🥛``   

# model of quark (size 'u')  
``▗▗▗▗▗▗▗▗t.t.t.t.t.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗t.t.t.t.t.t.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗t.t.t.t.t.t.t.▗▗▗▗▗▗``  
``▗▗▗▗▗t.t.t.t.t.t.t.t.▗▗▗▗▗``  
``▗▗▗▗t.t.t.t.t.t.t.t.t.▗▗▗▗``  
``▗▗▗▗▗t.t.t.t.t.t.t.t.▗▗▗▗▗``  
``▗▗▗▗▗▗t.t.t.t.t.t.t.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗t.t.t.t.t.t.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗t.t.t.t.t.▗▗▗▗▗▗▗▗``  


# hanb and base 64
https://en.wikipedia.org/wiki/Base64

ABCDEFGHIJKLMNOPQRSTUVWXTZabcdefghijklmnopqrstuvwxyz0123456789+/

abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-.



# more examples using unicode character replacement:

``🌾🌾🌾🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾🌾🌾``  
``🌾🌾🌾🌾🌾🌾🌾🌾🌺🌾🌺🌾🌺🌾🌺🌾🌺🌾🌾🌾🌾🌾🌾🌾🌾🌾``  

``🌲🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲🌲``

``🌲🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲🐈🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲❄️🌲🕳️🌲❄️🌲❄️🌲❤️🌲🐈🌲❄️🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❤️🌲❄️🌲⛳🌲❄️🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲❄️🌲❄️🌲🐈🌲❄️🌲❄️🌲🐈🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🕳️🌲🐈🌲❄️🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲❄️🌲❤️🌲⛳🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲🐈🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲🌲``

``🌲🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲❄️🌲🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲🏢🌲❄️🌲❄️🌲🚫🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲❄️🌲🕳️🌲❄️🌲❄️🌲🏘️🌲🏢🌲🚫🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲❄️🌲❄️🌲❄️🌲❄️🌲🏘️🌲❄️🌲⛳🌲🚫🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲❄️🌲❄️🌲🏢🌲❄️🌲🏚️🌲🏚️🌲❄️🌲🚫🌲🚫🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲❄️🌲❄️🌲🏚️🌲🏚️🌲🏚️🌲🕳️🌲🏢🌲🚫🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲❄️🌲🏘️🌲🏚️🌲🏚️🌲🏚️🌲☢️🌲🚫🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲🏚️🌲🏚️🌲☢️🌲☢️🌲🌲🌲🌲🌲🌲🌲🌲``  
``🌲🌲🌲🌲🌲🌲🌲🌲❄️🌲❄️🌲🏚️🌲☢️🌲☢️🌲🌲🌲🌲🌲🌲🌲🌲🌲``
 
``🌊🌊🌊🌊🌊🌊🌊🌊🏝️🌊🏝️🌊🏝️🌊⛵🌊🏝️🌊🌊🌊🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🌊🌊🏝️🌊🏝️🌊🐟🌊🏝️🌊🐳🌊🐳🌊🌊🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🌊🏝️🌊⛵🌊🏝️🌊🐳🌊🏝️🌊🐳🌊🏝️🌊🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🏝️🌊🐟🌊🐟🌊⛵🌊🐟🌊⛵🌊🏝️🌊🏝️🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🏝️🌊🏝️🌊🐟🌊🏝️🌊🏝️🌊🏝️🌊🐟🌊🏝️🌊🏝️🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊⛵🌊🏝️🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🌊🏝️🌊⛵🌊🐟🌊⛵🌊🏝️🌊🐟🌊🏝️🌊🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🌊🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🌊🌊🌊🌊🌊🌊🌊``  
``🌊🌊🌊🌊🌊🌊🌊🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🏝️🌊🌊🌊🌊🌊🌊🌊🌊🌊`` 
 
 ``✨✨✨✨✨✨✨✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨✨✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨✨✨✨✨✨✨``  
``✨✨✨✨✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨✨✨✨✨✨``  
``✨✨✨✨🇺🇸✨🇺🇸✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨🇨🇦✨✨✨✨✨``  
``✨✨✨✨✨🇺🇸✨🇺🇸✨🇨🇦✨🇺🇸✨🇺🇸✨🇨🇦✨🇨🇦✨🇺🇸✨✨✨✨✨✨``  
``✨✨✨✨✨✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨✨✨✨✨✨✨✨``  
``✨✨✨✨✨✨✨✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨🇺🇸✨✨✨✨✨✨✨✨✨`` 
 
``🌌🌌🌌🌌🌌🌌🌌🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌🌌🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌🌌⭐🌌⭐🌌⭐🌌🌎🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌⭐🌌🪐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌🌍🌌⭐🌌⭐🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌⭐🌌⭐🌌🌍🌌⭐🌌🪐🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌🌌🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌🌌🌌``  
``🌌🌌🌌🌌🌌🌌🌌🌌⭐🌌⭐🌌🪐🌌⭐🌌⭐🌌🌌🌌🌌🌌🌌🌌🌌🌌``

``🌺🌺🌺🌺🌺🌺🌺🌺⛄🌺🏔️🌺🏔️🌺🏔️🌺🏔️🌺🌺🌺🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺🌺🌺⛄🌺⛄🌺🌲🎅❄️🌺🏔️🌺🏔️🌺🌺🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺🌺⛄🌺⛄🌺🏔️🌺❄️🌺❄️🌺🏔️🌺🏔️🌺🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺⛄🌺⛄🌺🏔️🌺🌲🌺❄️🎅❄️🌺🏔️🌺🏔️🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺⛄🌺⛄🌺🏔️🎅❄️🌺❄️🌺❄️🌺❄️🌺🏔️🌺🏔️🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺⛄🌺⛄🌺❄️🌺❄️🌺❄️🌺❄️🌺❄️🌺🏔️🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺🌺⛄🌺⛄🌺❄️🎅❄️🌺❄️🌺🏔️🌺🏔️🌺🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺🌺🌺⛄🌺⛄🌺❄️🌺❄️🌺❄️🌺🏔️🌺🌺🌺🌺🌺🌺🌺🌺``  
``🌺🌺🌺🌺🌺🌺🌺🌺⛄🌺⛄🌺❄️🌺❄️🌺🏔️🌺🌺🌺🌺🌺🌺🌺🌺🌺``

``🌿🌿🌿🌿🌿🌿🌿🌿🌻🌿🌸🌿🌼🌿🌻🌿🌸🌿🌿🌿🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌿🌿🌸🌿🌼🌿🌻🌿🌸🌿🥚🌿🌻🌿🌿🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌿🌼🌿🌻🌿🌸🌿🐁🌿🌻🌿🌸🌿🌼🌿🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌻🌿🐇🌿🌼🌿🌻🌿🌸🌿🐇🌿🌻🌿🌸🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌸🌿🌼🌿🌻🌿🌸🌿🌼🌿🌻🌿🌸🌿🌼🌿🌻🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌻🌿🐁🌿🌼🌿🌻🌿🌸🌿🌼🌿🌻🌿🌸🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌿🌼🌿🌻🌿🌸🌿🌼🌿🌻🌿🌸🌿🌼🌿🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌿🌿🌸🌿🌼🌿🌻🌿🐇🌿🌼🌿🌻🌿🌿🌿🌿🌿🌿🌿🌿``    
``🌿🌿🌿🌿🌿🌿🌿🌿🌻🌿🌸🌿🌼🌿🌻🌿🌸🌿🌿🌿🌿🌿🌿🌿🌿🌿``  


[hanb Rust implementation](https://github.com/matheusfillipe/hanb) by Matheus Fillipe

[hanb IRC implementation](https://github.com/h4ks-com/hanbot) by Matheus Fillipe

[hanb HTML implementation on SDF](https://handyc.sdf.org) handyc SDF site

[hanb AI test](https://hexagon-nested-worlds.lovable.app/) AI-generated hanb test implementation by lovable

wooden hanb board by John Fisherkeller
![wooden hanb board](images/hex_board.jpg)

# hanb interaction examples

This section is still in progress and is not yet implemented anywhere.

(hello world in hanb)  
``.``   
``. -> generate universe from random seed``  
``. with seed FFFFFFFF generate universe: abdWE-99cddvDDEjfj2-WD30Jfj-SFTHFjtkedlkjelfl272-sXXjajsRF20sks3``  
``x``  
``expand largest mass in abdWE-99cddvDDEjfj2-WD30Jfj-SFTHFjtkedlkjelfl272-sXXjajsRF20sks3``  
``expanding largest mass in - to lkj5lk5t9g09figdjflkd99939eudlkj3l3lkjdkjl3jrf4880w049r878diuygd``  
``...``    
``expanding to city size O: MNaKMNaNMLMLMaMNMNMaMNKNMLaMNNMaMNLaMMNKNNMLMaMNNMNMNaNMMNMaMKLM``  
``x``  
``expand largest mass in MNaKMNaNMLMLMaMNMNMaMNKNMLaMNNMaMNLaMMNKNNMLMaMNNMNMNaNMMNMaMKLM``   
``expanding largest mass in N to MMMMMMMMMMMMMMLMMMMMMMMMMMMMMMMMMLMMMMMMMMMMMMMMMMMMMMMLMMMMMMMM``  

``#``  
``. -> generate matrix from random seed``  
``. with seed FC0F4FFF generate universe: .......49.........9................89...........39.........19HGK``  
``#``  
``. -> generate matrix from random seed``  
``. with seed FFEF0FAF generate universe: ........s........n...ee.........3e.....1n...........ou.....19212``  
``#``  
``. -> generate matrix from random seed``  
``. with seed FA0FFF3F generate universe: ....11111f.......24nA......222a2........a.....F00d.....a...19fEF``  

# Frequently Asked Questions

Q. I have a suggestion for improving hanb. How can I contribute to hanb?

A. Feedback is always appreciated. Use a pull request on this repo to send your suggestion(s) about improving hanb.


Q. Why does the actual universe have properties that are missing in hanb? / I think hanb is broken because it doesn't reliably model the actual universe.

A. This project is called a "toy universe" because it is inspired by the actual universe but is not intended to be a perfect model of reality.



