#include <stdio.h>

/*
# Register 3 = IP
ip = 0
r1 = r2 = r4 = r5 = 0
r0 = 1


0.  ip = ip + 16 # goto INIT
1.  r1 = 1
2.  r5 = 1
3.  r2 = r1 * r5
4.  r2 = if r2 == r4 { 1 } else { 0 }
5.  ip = r2 + ip
6.  ip = ip + 1
7.  r0 = r1 + r0
8.  r5 = r5 + 1
9.  r2 = if r5 > r4 { 1 } else { 0 }
10. ip = ip + r2
11. ip = 2
12. r1 = r1 + 1
13. r2 = if r1 > r4 { 1 } else { 0 }
14. ip = r2 + ip
15. ip = 1
16. ip = ip * ip
17. r4 = r4 + 2  # INIT
18. r4 = r4 * r4
19. r4 = ip * r4
20. r4 = r4 * 11 # r4 is a bigass number
21. r2 = r2 + 5
22. r2 = r2 * ip
23. r2 = r2 + 1 # r2 is a bigass number
24. r4 = r4 + r2 # r4 is even more bigass!
25. ip = ip + r0 # skip next on first iteration
26. ip = 0
27. r2 = ip       # r2 is 27
28. r2 = r2 * ip  # r2 is 27 * 28
29. r2 = ip + r2  # r2 is 29 + (27 * 28)
30. r2 = ip * r2  # r2 is 30 * (29 + (27 * 28))
31. r2 = r2 * 14  # r2 is (30 * (29 + (27 * 28))) * 14
32. r2 = r2 * ip  # r2 is ((30 * (29 + (27 * 28))) * 14) * 32
33. r4 = r4 + r2  # r4 is ? + ((30 * (29 + (27 * 28))) * 14) * 32
34. r0 = 0        #r0 = 0
35. ip = 0



first iteration: set up r2 and r4 to huge numbers
second iteration: enlarge them a little further?
third iteration: enter main loop
*/

int main() {
/*
  1.  r1 = 1
  2.  r5 = 1
  3.  r2 = r1 * r5
  4.  r2 = if r2 == r4 { 1 } else { 0 }
  5.  ip = r2 + ip
  6.  ip = ip + 1
  7.  r0 = r1 + r0
  8.  r5 = r5 + 1
  9.  r2 = if r5 > r4 { 1 } else { 0 }
  10. ip = ip + r2
  11. ip = 2
  12. r1 = r1 + 1
  13. r2 = if r1 > r4 { 1 } else { 0 }
  14. ip = r2 + ip
  15. ip = 1
  16. ip = ip * ip
*/

/* main program section */
/* i[0, 0, 10550400, 0, 10551347, 0] */

int r0, r1, r2, r4, r5;
r0 = r1 = r2 = r4 = r5 = 0;
/* [18, 11, 1, 256, 10, 11] */

/* why not... */
r4 = 50;  /* r0=18 r1=11 r2=1 r4=10 r5=11 */

/* Real desired value */
/* r4 = 10551347; */

r1 = 1;
while (1) {
    r5 = 1;

    while (1) {
        if ((r1 * r5) == r4) {
            printf("%d * %d == %d!\n", r1, r5, r4);
            printf("add %d\n", r1);
            r0 = r1 + r0;
        }

        r5 = r5 + 1;

        if (r5 > r4) {
            break;
        }
    }

    r1 = r1 + 1;

    if (r1 > r4) {
        break;
    }
}

/* DONE */
printf("r0=%d r1=%d r2=%d r4=%d r5=%d\n", r0, r1, r2, r4, r5);

return 0;
}


/*
total = 0
(1..10551347).each do |divisor|
  if 10551347 % divisor == 0
    total += divisor
  end
end

total # => 10695960
*/
  


/* start: */
/* r1 = 1; */
/* restart_outer: */
/* r5 = 1; */
/* restart_inner: */
/* r2 = r1 * r5; */
/* r2 = (r2 == r4); */
/* if (r2 == 1) { */
/*     goto skip1; */
/* } */
/* goto skip2; */
/* skip1: */
/*     r0 = r1 + r0; */
/* skip2: */
/* r5 = r5 + 1; */
/*  */
/* r2 = (r5 > r4); */
/*  */
/* if (r2 == 1) { */
/*     goto skip3; */
/* } */
/* goto restart_inner; */
/* skip3: */
/* r1 = r1 + 1; */
/*  */
/* r2 = (r1 > r4); */
/*  */
/* if (r2 == 1) { */
/*     goto skip4; */
/* } */
/*  */
/* goto restart_outer; */
/*  */
/* skip4: */
/* /\* DONE *\/ */
/* printf("r0=%d r1=%d r2=%d r4=%d r5=%d\n", r0, r1, r2, r4, r5); */
/*  */
/* return 0; */
