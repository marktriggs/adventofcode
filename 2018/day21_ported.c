#include <stdio.h>

/* Instruction pointer is register 5 */
/* We can control r0 */

/*
0.  r3 = 123
1.  r3 = r3 & 456
2.  r3 = if r3 == 72 { 1 } else { 0 }
3.  IP = r3 + IP
4.  IP = 0
5.  r3 = 0
6.  r2 = r3 | 65536
7.  r3 = 832312
8.  r1 = r2 & 255
9.  r3 = r3 + r1
10. r3 = r3 & 16777215
11. r3 = r3 * 65899
12. r3 = r3 & 16777215
13. r1 = if 256 > r2 { 1 } else { 0 }
14. IP = r1 + IP
15. IP = IP + 1
16. IP = 27
17. r1 = 0
18. r4 = r1 + 1
19. r4 = r4 * 256
20. r4 = if r4 > r2 { 1 } else { 0 }
21. IP = r4 + IP
22. IP = IP + 1
23. IP = 25
24. r1 = r1 + 1
25. IP = 17
26. r2 = r1
27. IP = 7
28. r1 = if r3 == r0 { 1 } else { 0 }
29. IP = r1 + IP
30. IP = 5
*/


int main() {
    int r0, r1, r2, r3, r4;

    /* Part 1: Lower bound */
    /* r0 = 212115; */

    r3 = 123 & 456;
    if (r3 == 72) {
        r3 = 1;
    } else {
        r3 = 0;
        main();                  /* Loop forever */
    }

    /* Starting out (L5) */
    r3 = 0;
    while (1) {
        r2 = r3 | 65536;
        r3 = 832312;

        while (1) {
            r1 = r2 & 255;
            r3 = r3 + r1;
            r3 = r3 & 16777215;
            r3 = r3 * 65899;
            r3 = r3 & 16777215;
            if (256 > r2) {
                break;
            }
            r1 = 0;

            while (1) {
                r4 = r1 + 1;
                r4 = r4 * 256;

                if (r4 > r2) {
                    break;
                }

                r1 = r1 + 1;
            }

            r2 = r1;
        }

        if (r3 == r0) {
            /* FINISHED */
            return 0;
        }

        printf("Around again with r3=%d\n", r3);
    }
}
