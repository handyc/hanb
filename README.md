# hanb

* hanb is a simple language for creating model universes at any scale  
* hanb can be used with or without a computer  
* if you know the alphabet then you can use hanb  

the basis of the hanb system is a 61-cell hexagonal board:

``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗A.A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  

each cell contains a single character from a set of 64 total characters:

a-z, A-Z, 0-9, _ and .

boards can be written in a compressed form as 61-character strings, e.g.:

tFZ2fD1LbWJYCkaB_feYB7wkEVjINP._taGiY8iAef7R1noBhM_uz.Bdwgmld

lmXDMBOHISwvumSev7rZtf449JmaUfPIN0EGWHK_yu782UobkdRNNhrkkDVTS

acuXsBJ18dV_dXTD3pi8hDD6_bDFIQ94wi9vFNbZyCa3ZL8NNN6Bb2zVzoqRE

fMTNqDeeIKrPQAqveHdY5.BZRDtHxMDlotxyR3y2Cy_5VnrnDpxX6Ssk1LRps

19i7oMvDURxQzfHiVVCbA4dL278J_XvPfgL.rTOgcQrrK03nlCBJ3RMXHdOyP

aYfqpEOK1s5gtDE_ocemVoUH45Ef88Pp9POLKF7cH97WADGBa2dBC6kL6wljU

t7t82Ry18okmA1HCr0R8zi0HncVQyGtTo5xGo_44PXA4NRK0ND_sbVrT8fJMt

Each board represents a finite space at a particular scale between 0 and 63,
also represented by the above character set, such that 'a' is a board filled
with quantum foam and '.' is a board containing one or more universes

therefore we call 'a' a quantum foam container or a quantum foam particle
and '.' a universe container or universe

hanb board sizes increase by a factor of 10 with each character, e.g.

1x10^-10 meters or 1 Angstrom is represented by A  
...  
1x10^-4 meters or 1 micrometer is represented by G  
1x10^-3 meters or 1 millimeter is represented by H  
1x10^-2 meters or 1 centimeter is represented by I  
...  
1x10^0 meters or 1 meter is represented by K  
and so on up to the maximum size of '.'  

hanb is based on our actual universe but is highly simplified to allow for different kinds of
relationships to be tested at different scales without requiring a large amount of storage space

hanb can be used for a variety of modeling applications, including computer games,
social relationship simulations, terrain visualizations and other types of projects requiring
large dynamic spaces in limited memory

a -- quantum foam particle  
b -- Planck particle  
c  
d  
e  
f  
g  
h  
i  
j  
k  
l  
m  
n  
o  
p  
q  
r  
s  
t  
u  
v  
w  
x  
y  
z  
A -- Angstrom  
B --  
C --  
D --  
E --    
F --    
G -- pixel  
H --  
I --   
J --   
K -- person  
L -- lot  
M -- hamlet  
N -- neighborhood  
O -- borough  
P -- province  
Q -- continent  
R -- planet  
S -- planet-moon system  
T  
U  
V  
W  
X -- star system  
Y  
Z  
0  
1  
2  
3  
4  
5  
6 -- galaxy  
7  
8  
9  
_ -- filament  
. -- universe container  

time in hanb

a -- Planck time particle  
b --   
c  
d  
e  
f  
g  
h  
i  
j  
k  
l  
m  
n  
o  
p  
q  
r  
s  
t  
u  
v  
w  
x  
y  
z -- 1 million yrs

A -- <1 billion yrs  

B --  >1 billion yrs

C -- 1 trillion yrs 
D -- 1 quadrillion yrs 
E --    
F --    
G --   
H --  
I --   
J --   
K --   
L --1e100 yrs   
M --   
N --   
O --  
P --   
Q --  
R --   
S -- 
T  
U  
V  1e1000 yrs = heat death
W  
X -- 
Y  
Z  
0  
1  
2  
3  
4  
5  
6 -- 
7  
8  
9  
_ --   
. -- 

# hanb replacement language
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

# Implementing hanb
# physical
# digital
# hanb text mode
# hanb graphics
# unicode hanb
# genetic hanb

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

using 64 bytes for each cell (reserve 3 additional bytes for RGB color information):  
64^2 = 4096    
4096 x 5 = 20480 bytes    

# Use cases
# simple rpg example

generate scope replacement palettes a-z, A-Z, 0-9, _ .

a -> jHtsb3VjW2bLgToIqvMyJSiz2dvYKT4MOayOO8HjWQ8Jzvnm1YzWbA3CvEoA1  
b -> pwc0SwumO0nTW9aHCtyRU8PZBDUwDYWFeEbsuFfw5Bn8W6dl8xmuGAdzYwHpx  

...

9 -> GCYLqcob3AXPStlz2oaPNvcUPz35ZTJaX506vtYZQkVKTC8s2E2AsERZDF0qr  
_ -> 6weMksPXE7kyap1Q8wxij0ttLn6gHSu9n01k0hENnCCrHf7Zs0Xy9srvnzz6b  
. -> HWwpihtNApIJwZ2wZakSWo2K9WjsxvSnlFCGfamR3GsVeie9Eo3eQhvuBoIUP  


generate character K  
wZcgQZaWrht6a6rEWLLbcxGbS6rjUPFKBtLrRt8eLNlzxJf0WNiQ5LeIWOXrc

generate weapon J  
S5pl9pE1I37bVazv52AyZiJCnBzLeRwpelSv28oeo9MVHG7zKvnhWBi2ZoHfv

generate magic ring I  
yRjHTPofk90aE76QYtwvptU0zH0SAHOi3oPzOSk2xDnxlpccvakfmn4RHUjcb

generate
closely inspect H  
6J50bQJ0CRr68KkPcS8ig5H2S0fVG5iZyCuICJVzBfOVyWrCESQKov4gXDotP

-----

simulated walking map L  
aaaaaKaaaaaKKaaKKaaaaaKaaaaaaaKaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa   
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗K.a.a.a.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗K.K.a.a.K.K.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.K.a.a.a.▗▗▗▗``  
``▗▗▗▗a.a.a.a.K.a.a.a.J.▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.a.a.a.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.J.a.a.a.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.J.a.a.J.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated hamlet overmap M  
lmXDMBOHISwvumSev7rZtf449JmaUfPIN0EGWHK_yu782UobkdRNNhrkkDVTS  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.K.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.K.a.a.a.L.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.a.L.a.a.a.a.▗▗▗▗``  
``▗▗▗▗a.a.a.a.a.a.a.L.a.▗▗▗▗``  
``▗▗▗▗▗a.a.L.a.a.a.a.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.a.a.L.a.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.K.a.K.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated village quest map N  
acuXsBJ18dV_dXTD3pi8hDD6_bDFIQ94wi9vFNbZyCa3ZL8NNN6Bb2zVzoqRE  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.a.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.M.a.M.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.a.a.a.a.▗▗▗▗``  
``▗▗▗▗a.a.a.a.M.a.a.a.a.▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.a.a.M.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.L.a.a.a.L.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.a.a.M.a.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated city map O  
fMTNqDeeIKrPQAqveHdY5.BZRDtHxMDlotxyR3y2Cy_5VnrnDpxX6Ssk1LRps  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.N.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.a.a.N.a.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.a.K.M.a.K.A.a.▗▗▗▗``  
``▗▗▗▗a.a.a.a.N.a.a.K.a.▗▗▗▗``  
``▗▗▗▗▗a.a.L.L.a.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.M.A.A.M.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.L.L.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗`` 

simulated kingdom P  
19i7oMvDURxQzfHiVVCbA4dL278J_XvPfgL.rTOgcQrrK03nlCBJ3RMXHdOyP  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.A.A.M.O.K.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.K.a.A.A.a.▗▗▗▗▗▗``  
``▗▗▗▗▗a.O.N.N.a.a.O.a.▗▗▗▗``  
``▗▗▗▗a.A.A.N.O.M.a.A.a.▗▗▗▗``  
``▗▗▗▗▗a.a.a.a.K.a.a.a.▗▗▗▗▗``  
``▗▗▗▗▗▗a.a.M.A.A.M.a.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗a.L.A.A.O.a.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗a.a.a.a.a.▗▗▗▗▗▗▗▗`` 

simulated continent Q  
aYfqpEOK1s5gtDE_ocemVoUH45Ef88Pp9POLKF7cH97WADGBa2dBC6kL6wljU  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.P.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.P.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗A.A.A.P.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.P.A.A.P.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗`` 

simulated planet R  
t7t82Ry18okmA1HCr0R8zi0HncVQyGtTo5xGo_44PXA4NRK0ND_sbVrT8fJMt  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.Q.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.Q.A.Q.Q.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.Q.A.A.A.Q.A.▗▗▗▗``  
``▗▗▗▗A.A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.Q.Q.Q.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.Q.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗`` 



# tourist map of city
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.L.M.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.N.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗K.K.K.K.K.K.K.K.▗▗▗▗``  
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
``▗▗▗▗▗A.A.A.A.R.A.A.A.▗▗▗▗``  
``▗▗▗▗A.A.A.A.X.A.A.A.R.▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.R.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.Q.A.A.A.▗▗▗▗▗▗▗▗``  

# model of Milky Way  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗A.A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  

# model of Oxygen atom  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗A.A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  

# model of quark  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗A.A.A.A.A.A.A.A.A.▗▗▗▗``  
``▗▗▗▗▗A.A.A.A.A.A.A.A.▗▗▗▗▗``  
``▗▗▗▗▗▗A.A.A.A.A.A.A.▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗A.A.A.A.A.A.▗▗▗▗▗▗▗``  
``▗▗▗▗▗▗▗▗A.A.A.A.A.▗▗▗▗▗▗▗▗``  


# hanb and base 64
https://en.wikipedia.org/wiki/Base64

ABCDEFGHIJKLMNOPQRSTUVWXTZabcdefghijklmnopqrstuvwxyz0123456789+/

abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_.

