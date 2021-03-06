## Calculator

Code challenge

An incorrect implementation of a simple calculator, where the following operators are supported and have ordered precedence:
``` 
*
/
+
-
```
(Although not BODMAS, this was more challenging to solve!)

```
**************************************
Expression involving bracket example:
**************************************

Given:
1 + 2 * (3 + 4) - 5 * (6 * (7 + 8)) =
1 + 2 * 7 - 5 * 90 = (15 - 450) = -435

1) Tokenize brackets
1 + 2 * a - 5 * b
a = 3 + 4
b = 6 * c
c = 7 + 8

2) Build binary expression groups 
[{1+2} {2*a} {a-5} {5*b}]
          a = [{3+4}]
                      b = [{6*c}]
                              c = [{7+8}]

3) Evalutate 
[{1+2} {2*7} {a-5} {5*b}]
                      b = [{6*c}]
                              c = [{7+8}]

[{1+2} {2*7} {a-5} {5*b}]
                      b = [{6*15}]

[{1+2} {2*7} {a-5} {5*90}]

*
[{1+2} {2*7} {a-5}  {5*90}]
[{1+14}      {14-450}]

+
[{1+14} {14-450}]
[{15-450}]

-
[{15-450}]

**************************************
Expressions without bracket examples:
**************************************

Given:
1 + 2 * 3 + 4 - 5 * 6 + 7 = (11 - 37) = -26

1) Build binary expression groups 
[{1+2} {2*3} {3+4} {4-5} {5*6} {6+7}]

2) Evalutate
*
[{1+2} {2*3} {3+4} {4-5} {5*6} {6+7}]
[{1+6}       {6+4} {4-5} {5*6} {6+7}]
[{1+6}       {6+4} {4-30}      {30+7}]

+
[{1+6} {6+4} {4-30} {30+7}]
[      {7+4} {4-30} {30+7}]
[            {11-30}{30+7}]
[            {11-37}      ]

-
[{11-37}]

**************************************
Given: 
1 + 2 - 3 * 4 * 5 - 6 + 7 = (1 + 2 - 60 - 6 + 7) = (3 - 60 - 13) = -70

1) Build binary expression groups 
[{1+2} {2-3} {3*4} {4*5} {5-6} {6+7}]

2) Evaluate
*
[{1+2} {2-3} {3*4} {4*5} {5-6} {6+7}]
[{1+2} {2-12}      {12*5} {5-6} {6+7}]
[{1+2} {2-60}            {60-6} {6+7}]

+
[{1+2} {2-60} {60-6} {6+7}]
[      {3-60} {60-6} {6+7}]
[      {3-60} {60-13}     ]

-
[{3-60} {60-13}]
[       {-57-13}]

**************************************
Given:
1 * 2 + 3 + 4 - 5 - 6 * 7 = (9 - 5 - 42) = -38

1) Build binary expression groups 
[{1*2} {2+3} {3+4} {4-5} {5-6} {6*7}]

2) Evaluate
*
[{1*2} {2+3} {3+4} {4-5} {5-6} {6*7}]
[      {2+3} {3+4} {4-5} {5-6} {6*7}]
[      {2+3} {3+4} {4-5} {5-42}     ]

+
[{2+3} {3+4} {4-5} {5-42}]
[      {5+4} {4-5} {5-42}]
[            {9-5} {5-42}]

-
[{9-5} {5-42}]
[      {4-42}]
[{4-42}]
```