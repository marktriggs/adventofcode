 const std = @import("std");

 const Result = enum {
     Accept,
     Reject,
 };

 const Part = struct {
     x: i32,
     m: i32,
     a: i32,
     s: i32,
 };


fn ljs(part: Part) Result {
  if (part.s < 678 ){ return Result.Accept; }
  if (part.s < 945 ){ return Result.Accept; }
  if (part.x > 1416 ){ return Result.Accept; }
return Result.Accept;

}


fn cg(part: Part) Result {
  if (part.a < 257 ){ return Result.Reject; }
return Result.Reject;

}


fn pdl(part: Part) Result {
  if (part.x < 1159 ){ return nkv(part); }
  if (part.a < 2177 ){ return Result.Accept; }
return Result.Reject;

}


fn hsn(part: Part) Result {
  if (part.s < 3192 ){ return jl(part); }
  if (part.m > 2417 ){ return jvm(part); }
{ return rh(part); }

}


fn rg(part: Part) Result {
  if (part.a < 3291 ){ return dp(part); }
  if (part.s < 2621 ){ return hp(part); }
  if (part.x < 1664 ){ return jfj(part); }
{ return hc(part); }

}


fn ngf(part: Part) Result {
  if (part.x > 1374 ){ return nn(part); }
{ return rsg(part); }

}


fn dlt(part: Part) Result {
  if (part.x < 2111 ){ return Result.Accept; }
  if (part.x > 2321 ){ return zrq(part); }
return Result.Accept;

}


fn gq(part: Part) Result {
  if (part.m < 2038 ){ return Result.Accept; }
  if (part.s > 1737 ){ return Result.Reject; }
  if (part.a > 3367 ){ return Result.Reject; }
return Result.Accept;

}


fn qz(part: Part) Result {
  if (part.x > 1061 ){ return zjf(part); }
{ return gff(part); }

}


fn knx(part: Part) Result {
  if (part.m < 1553 ){ return xzx(part); }
  if (part.m > 2473 ){ return Result.Accept; }
  if (part.m < 1996 ){ return Result.Accept; }
return Result.Accept;

}


fn gff(part: Part) Result {
  if (part.a < 3519 ){ return Result.Accept; }
  if (part.s > 3476 ){ return pkg(part); }
  if (part.a < 3552 ){ return Result.Accept; }
return Result.Reject;

}


fn hj(part: Part) Result {
  if (part.a < 3430 ){ return zvp(part); }
  if (part.s > 1630 ){ return Result.Reject; }
return Result.Reject;

}


fn rbg(part: Part) Result {
  if (part.s > 719 ){ return Result.Accept; }
return Result.Accept;

}


fn rqc(part: Part) Result {
  if (part.a > 1671 ){ return Result.Reject; }
  if (part.a > 1628 ){ return Result.Accept; }
{ return slx(part); }

}


fn pz(part: Part) Result {
  if (part.m > 1626 ){ return Result.Reject; }
return Result.Reject;

}


fn tvk(part: Part) Result {
  if (part.a > 667 ){ return gch(part); }
  if (part.a < 280 ){ return pcm(part); }
{ return vnq(part); }

}


fn jcj(part: Part) Result {
  if (part.m > 2035 ){ return Result.Reject; }
  if (part.m > 872 ){ return Result.Accept; }
  if (part.x > 695 ){ return Result.Reject; }
return Result.Accept;

}


fn pk(part: Part) Result {
  if (part.s < 1448 ){ return fgs(part); }
  if (part.a < 1065 ){ return zcp(part); }
{ return nt(part); }

}


fn sd(part: Part) Result {
  if (part.m < 3026 ){ return Result.Accept; }
  if (part.m > 3670 ){ return Result.Accept; }
return Result.Reject;

}


fn lg(part: Part) Result {
  if (part.a > 3557 ){ return Result.Reject; }
  if (part.m < 3073 ){ return Result.Reject; }
return Result.Reject;

}


fn cm(part: Part) Result {
  if (part.x > 381 ){ return Result.Reject; }
  if (part.x < 148 ){ return Result.Reject; }
  if (part.s > 1125 ){ return jtp(part); }
return Result.Reject;

}


fn rd(part: Part) Result {
  if (part.x > 965 ){ return hkx(part); }
  if (part.a < 3832 ){ return sfb(part); }
  if (part.a > 3893 ){ return Result.Reject; }
return Result.Reject;

}


fn kk(part: Part) Result {
  if (part.x < 920 ){ return mnj(part); }
{ return jzn(part); }

}


fn pvf(part: Part) Result {
  if (part.m < 658 ){ return Result.Accept; }
  if (part.s < 3646 ){ return Result.Reject; }
  if (part.m > 1057 ){ return Result.Accept; }
return Result.Accept;

}


fn gch(part: Part) Result {
  if (part.m < 2239 ){ return vzm(part); }
  if (part.x > 2975 ){ return kcn(part); }
  if (part.s < 1492 ){ return ksk(part); }
{ return kdz(part); }

}


fn dc(part: Part) Result {
  if (part.m > 1223 ){ return Result.Reject; }
return Result.Accept;

}


fn fbc(part: Part) Result {
  if (part.x > 1013 ){ return Result.Accept; }
return Result.Accept;

}


fn zqx(part: Part) Result {
  if (part.a < 265 ){ return Result.Reject; }
  if (part.a < 347 ){ return Result.Accept; }
return Result.Reject;

}


fn rsg(part: Part) Result {
  if (part.x > 757 ){ return Result.Reject; }
  if (part.m < 1548 ){ return Result.Accept; }
return Result.Reject;

}


fn ln(part: Part) Result {
  if (part.x > 1177 ){ return Result.Reject; }
return Result.Accept;

}


fn hx(part: Part) Result {
  if (part.s > 645 ){ return Result.Accept; }
  if (part.a > 2218 ){ return rz(part); }
{ return jvr(part); }

}


fn cjk(part: Part) Result {
  if (part.s > 3381 ){ return Result.Reject; }
  if (part.s < 3093 ){ return Result.Reject; }
  if (part.m < 1396 ){ return Result.Accept; }
return Result.Reject;

}


fn db(part: Part) Result {
  if (part.x < 2268 ){ return Result.Reject; }
  if (part.x < 2490 ){ return Result.Reject; }
return Result.Accept;

}


fn kr(part: Part) Result {
  if (part.x > 1435 ){ return Result.Reject; }
  if (part.m > 2091 ){ return mqj(part); }
  if (part.x > 858 ){ return Result.Accept; }
{ return dx(part); }

}


fn qpb(part: Part) Result {
  if (part.a > 3866 ){ return Result.Reject; }
  if (part.a > 3841 ){ return Result.Accept; }
  if (part.s > 2792 ){ return Result.Reject; }
return Result.Accept;

}


fn fb(part: Part) Result {
  if (part.s > 1967 ){ return nh(part); }
  if (part.a > 755 ){ return Result.Reject; }
  if (part.x > 299 ){ return kl(part); }
return Result.Reject;

}


fn ndp(part: Part) Result {
  if (part.s > 3566 ){ return Result.Reject; }
return Result.Accept;

}


fn qhp(part: Part) Result {
  if (part.m < 2678 ){ return Result.Accept; }
  if (part.x > 475 ){ return Result.Reject; }
return Result.Reject;

}


fn sr(part: Part) Result {
  if (part.m > 2266 ){ return Result.Accept; }
{ return gqv(part); }

}


fn gm(part: Part) Result {
  if (part.a < 3339 ){ return ph(part); }
  if (part.s > 2935 ){ return Result.Reject; }
{ return qd(part); }

}


fn rl(part: Part) Result {
  if (part.x < 1298 ){ return bdm(part); }
return Result.Accept;

}


fn cgx(part: Part) Result {
  if (part.m > 2248 ){ return Result.Reject; }
return Result.Accept;

}


fn gqr(part: Part) Result {
  if (part.a > 3424 ){ return tr(part); }
  if (part.s < 3065 ){ return hq(part); }
{ return mmd(part); }

}


fn sg(part: Part) Result {
  if (part.a < 3842 ){ return md(part); }
  if (part.m < 1618 ){ return nxr(part); }
{ return zsz(part); }

}


fn gc(part: Part) Result {
  if (part.a > 2664 ){ return Result.Reject; }
  if (part.a < 2647 ){ return Result.Reject; }
return Result.Reject;

}


fn pq(part: Part) Result {
  if (part.m < 3719 ){ return vv(part); }
{ return vb(part); }

}


fn nrk(part: Part) Result {
  if (part.s > 3098 ){ return Result.Reject; }
return Result.Reject;

}


fn hc(part: Part) Result {
  if (part.m > 2338 ){ return xqz(part); }
{ return mh(part); }

}


fn cjm(part: Part) Result {
  if (part.m < 1986 ){ return Result.Reject; }
  if (part.a < 3564 ){ return Result.Reject; }
  if (part.s > 3701 ){ return Result.Accept; }
return Result.Reject;

}


fn blz(part: Part) Result {
  if (part.x < 642 ){ return fz(part); }
return Result.Reject;

}


fn trr(part: Part) Result {
  if (part.a < 2776 ){ return tgz(part); }
  if (part.a < 3101 ){ return Result.Accept; }
  if (part.x > 1202 ){ return Result.Reject; }
{ return pc(part); }

}


fn bts(part: Part) Result {
  if (part.m > 3295 ){ return Result.Reject; }
  if (part.m < 2936 ){ return Result.Reject; }
  if (part.s < 2046 ){ return Result.Accept; }
return Result.Reject;

}


fn nh(part: Part) Result {
  if (part.x < 224 ){ return Result.Reject; }
  if (part.s > 2192 ){ return Result.Reject; }
return Result.Accept;

}


fn ng(part: Part) Result {
  if (part.s > 1399 ){ return vdd(part); }
  if (part.x > 1547 ){ return kqb(part); }
{ return srp(part); }

}


fn hl(part: Part) Result {
  if (part.s > 2791 ){ return nxn(part); }
{ return ncq(part); }

}


fn shh(part: Part) Result {
  if (part.s < 2467 ){ return Result.Accept; }
  if (part.s > 2535 ){ return Result.Reject; }
  if (part.s > 2507 ){ return Result.Accept; }
return Result.Reject;

}


fn jsb(part: Part) Result {
  if (part.m > 2967 ){ return Result.Reject; }
  if (part.x > 2398 ){ return Result.Accept; }
  if (part.x > 1994 ){ return Result.Accept; }
return Result.Accept;

}


fn rnd(part: Part) Result {
  if (part.x < 280 ){ return Result.Reject; }
  if (part.a < 3085 ){ return Result.Reject; }
return Result.Reject;

}


fn jl(part: Part) Result {
  if (part.s > 2887 ){ return Result.Accept; }
  if (part.m > 1687 ){ return smb(part); }
return Result.Accept;

}


fn rz(part: Part) Result {
  if (part.s > 400 ){ return Result.Reject; }
  if (part.s < 220 ){ return Result.Reject; }
  if (part.m < 1785 ){ return Result.Accept; }
return Result.Accept;

}


fn zq(part: Part) Result {
  if (part.m < 3210 ){ return Result.Reject; }
  if (part.s > 3283 ){ return cnq(part); }
{ return tvz(part); }

}


fn dm(part: Part) Result {
  if (part.a > 3435 ){ return Result.Accept; }
  if (part.x < 695 ){ return Result.Reject; }
return Result.Reject;

}


fn bb(part: Part) Result {
  if (part.a < 3756 ){ return Result.Reject; }
  if (part.s > 3508 ){ return Result.Accept; }
  if (part.a > 3873 ){ return Result.Reject; }
return Result.Accept;

}


fn vmm(part: Part) Result {
  if (part.x > 448 ){ return Result.Reject; }
  if (part.a > 3903 ){ return Result.Accept; }
  if (part.a > 3885 ){ return Result.Reject; }
return Result.Reject;

}


fn zrq(part: Part) Result {
  if (part.m < 3492 ){ return Result.Reject; }
  if (part.m > 3751 ){ return Result.Accept; }
  if (part.a < 3745 ){ return Result.Reject; }
return Result.Reject;

}


fn nhd(part: Part) Result {
  if (part.x < 2569 ){ return Result.Reject; }
  if (part.m < 2472 ){ return Result.Accept; }
return Result.Accept;

}


fn sv(part: Part) Result {
  if (part.m < 221 ){ return Result.Reject; }
  if (part.a > 3355 ){ return Result.Reject; }
return Result.Accept;

}


fn fpn(part: Part) Result {
  if (part.s > 1460 ){ return Result.Accept; }
return Result.Accept;

}


fn sfb(part: Part) Result {
  if (part.a > 3717 ){ return Result.Reject; }
return Result.Accept;

}


fn pmc(part: Part) Result {
  if (part.a > 462 ){ return pjd(part); }
{ return tlb(part); }

}


fn bpq(part: Part) Result {
  if (part.s < 1116 ){ return qv(part); }
  if (part.a > 3589 ){ return hrm(part); }
  if (part.m > 563 ){ return Result.Reject; }
{ return spx(part); }

}


fn gmx(part: Part) Result {
  if (part.a < 3009 ){ return tqn(part); }
  if (part.s < 1628 ){ return qt(part); }
  if (part.s < 1822 ){ return mf(part); }
{ return xk(part); }

}


fn vhg(part: Part) Result {
  if (part.x < 362 ){ return Result.Reject; }
return Result.Reject;

}


fn tmh(part: Part) Result {
  if (part.m < 1402 ){ return cz(part); }
  if (part.a > 200 ){ return Result.Reject; }
  if (part.m > 1416 ){ return Result.Reject; }
return Result.Accept;

}


fn td(part: Part) Result {
  if (part.s < 1396 ){ return bc(part); }
{ return ht(part); }

}


fn sxn(part: Part) Result {
  if (part.x < 3136 ){ return Result.Reject; }
  if (part.x < 3534 ){ return Result.Reject; }
return Result.Accept;

}


fn gh(part: Part) Result {
  if (part.x > 3453 ){ return Result.Accept; }
  if (part.s > 2897 ){ return bxl(part); }
return Result.Reject;

}


fn fl(part: Part) Result {
  if (part.m < 3303 ){ return Result.Reject; }
  if (part.a > 882 ){ return Result.Accept; }
  if (part.x < 1389 ){ return Result.Accept; }
return Result.Reject;

}


fn zvp(part: Part) Result {
  if (part.x > 1197 ){ return Result.Reject; }
  if (part.s > 1721 ){ return Result.Reject; }
  if (part.s < 808 ){ return Result.Accept; }
return Result.Reject;

}


fn tm(part: Part) Result {
  if (part.x > 1475 ){ return Result.Reject; }
  if (part.m < 2652 ){ return Result.Accept; }
return Result.Reject;

}


fn rv(part: Part) Result {
  if (part.a > 3608 ){ return Result.Reject; }
  if (part.m < 499 ){ return Result.Reject; }
  if (part.a > 3448 ){ return Result.Accept; }
return Result.Reject;

}


fn kv(part: Part) Result {
  if (part.s > 2921 ){ return Result.Reject; }
  if (part.a > 3628 ){ return Result.Reject; }
return Result.Accept;

}


fn mkg(part: Part) Result {
  if (part.s > 2246 ){ return Result.Reject; }
  if (part.s < 2196 ){ return Result.Reject; }
return Result.Reject;

}


fn dg(part: Part) Result {
  if (part.s > 3246 ){ return Result.Accept; }
  if (part.a > 1678 ){ return nhd(part); }
  if (part.a > 1615 ){ return Result.Reject; }
{ return jf(part); }

}


fn glb(part: Part) Result {
  if (part.s > 1964 ){ return jmb(part); }
  if (part.a < 2217 ){ return ksj(part); }
{ return ft(part); }

}


fn sz(part: Part) Result {
  if (part.s < 3286 ){ return qnn(part); }
  if (part.s > 3700 ){ return Result.Accept; }
  if (part.x > 1213 ){ return Result.Accept; }
return Result.Accept;

}


fn lsl(part: Part) Result {
  if (part.m < 870 ){ return Result.Reject; }
return Result.Reject;

}


fn jvm(part: Part) Result {
  if (part.a > 1426 ){ return Result.Reject; }
  if (part.s > 3572 ){ return Result.Accept; }
{ return jjl(part); }

}


fn zm(part: Part) Result {
  if (part.x > 2104 ){ return hnt(part); }
{ return cjm(part); }

}


fn sb(part: Part) Result {
  if (part.s < 2350 ){ return bm(part); }
{ return mc(part); }

}


fn jvr(part: Part) Result {
  if (part.m < 2080 ){ return Result.Reject; }
return Result.Reject;

}


fn kcn(part: Part) Result {
  if (part.s < 1037 ){ return bs(part); }
  if (part.s < 1802 ){ return bzv(part); }
  if (part.x > 3329 ){ return bts(part); }
return Result.Reject;

}


fn fdj(part: Part) Result {
  if (part.s > 681 ){ return Result.Accept; }
  if (part.a > 1556 ){ return Result.Accept; }
  if (part.x < 2478 ){ return Result.Reject; }
return Result.Reject;

}


fn dl(part: Part) Result {
  if (part.a < 428 ){ return Result.Accept; }
  if (part.a < 455 ){ return Result.Accept; }
  if (part.a > 478 ){ return Result.Accept; }
return Result.Reject;

}


fn xld(part: Part) Result {
  if (part.a > 3094 ){ return ljs(part); }
  if (part.x > 1625 ){ return rbg(part); }
{ return qf(part); }

}


fn hq(part: Part) Result {
  if (part.x < 969 ){ return Result.Accept; }
return Result.Accept;

}


fn zn(part: Part) Result {
  if (part.m > 1368 ){ return Result.Accept; }
return Result.Accept;

}


fn mm(part: Part) Result {
  if (part.s < 2797 ){ return Result.Accept; }
return Result.Accept;

}


fn jtp(part: Part) Result {
  if (part.m > 1676 ){ return Result.Accept; }
  if (part.s < 1331 ){ return Result.Reject; }
  if (part.m < 651 ){ return Result.Reject; }
return Result.Accept;

}


fn tql(part: Part) Result {
  if (part.x > 700 ){ return vn(part); }
  if (part.a > 2976 ){ return df(part); }
  if (part.s > 3447 ){ return sq(part); }
{ return tsn(part); }

}


fn flc(part: Part) Result {
  if (part.x > 1241 ){ return Result.Reject; }
{ return ksl(part); }

}


fn jsr(part: Part) Result {
  if (part.s < 3776 ){ return Result.Reject; }
  if (part.a > 608 ){ return Result.Reject; }
return Result.Reject;

}


fn lnt(part: Part) Result {
  if (part.m < 759 ){ return Result.Reject; }
  if (part.s < 733 ){ return Result.Reject; }
return Result.Reject;

}


fn zsg(part: Part) Result {
  if (part.x < 1980 ){ return Result.Accept; }
  if (part.x > 2805 ){ return Result.Reject; }
  if (part.s < 2749 ){ return Result.Reject; }
return Result.Reject;

}


fn mt(part: Part) Result {
  if (part.s < 3386 ){ return Result.Accept; }
  if (part.x > 2520 ){ return Result.Reject; }
  if (part.s < 3724 ){ return Result.Reject; }
{ return mb(part); }

}


fn tgq(part: Part) Result {
  if (part.m > 2109 ){ return Result.Reject; }
  if (part.a > 3378 ){ return Result.Accept; }
return Result.Accept;

}


fn rxt(part: Part) Result {
  if (part.m < 1595 ){ return Result.Accept; }
  if (part.s < 3515 ){ return Result.Accept; }
return Result.Accept;

}


fn ct(part: Part) Result {
  if (part.s < 3605 ){ return vpv(part); }
{ return kqc(part); }

}


fn rvq(part: Part) Result {
  if (part.m < 3050 ){ return Result.Reject; }
{ return lvl(part); }

}


fn fk(part: Part) Result {
  if (part.s > 1367 ){ return Result.Accept; }
return Result.Accept;

}


fn qv(part: Part) Result {
  if (part.a > 3588 ){ return Result.Reject; }
  if (part.x > 758 ){ return Result.Reject; }
return Result.Reject;

}


fn fr(part: Part) Result {
  if (part.x > 2235 ){ return zgd(part); }
  if (part.s < 3567 ){ return bxj(part); }
{ return ldt(part); }

}


fn xpp(part: Part) Result {
  if (part.a > 3454 ){ return Result.Reject; }
  if (part.m > 1570 ){ return Result.Reject; }
return Result.Reject;

}


fn dbp(part: Part) Result {
  if (part.m > 3199 ){ return nq(part); }
  if (part.a < 3025 ){ return Result.Accept; }
  if (part.s < 605 ){ return Result.Accept; }
return Result.Reject;

}


fn spx(part: Part) Result {
  if (part.m > 349 ){ return Result.Reject; }
  if (part.m > 169 ){ return Result.Reject; }
return Result.Reject;

}


fn gpb(part: Part) Result {
  if (part.s < 924 ){ return Result.Accept; }
return Result.Accept;

}


fn lvm(part: Part) Result {
  if (part.a > 1551 ){ return kt(part); }
{ return hsn(part); }

}


fn hnt(part: Part) Result {
  if (part.x < 2313 ){ return Result.Reject; }
  if (part.m > 2024 ){ return Result.Reject; }
  if (part.a < 3756 ){ return Result.Accept; }
return Result.Reject;

}


fn zc(part: Part) Result {
  if (part.s < 1907 ){ return Result.Reject; }
  if (part.s < 2212 ){ return Result.Accept; }
  if (part.s < 2279 ){ return tbv(part); }
return Result.Reject;

}


fn dj(part: Part) Result {
  if (part.a < 984 ){ return shh(part); }
  if (part.m < 1843 ){ return cc(part); }
return Result.Accept;

}


fn gk(part: Part) Result {
  if (part.a < 3211 ){ return Result.Accept; }
return Result.Reject;

}


fn zzs(part: Part) Result {
  if (part.x > 3608 ){ return Result.Reject; }
  if (part.x > 3547 ){ return Result.Reject; }
  if (part.x < 3531 ){ return Result.Reject; }
return Result.Reject;

}


fn sj(part: Part) Result {
  if (part.m < 736 ){ return bv(part); }
{ return mt(part); }

}


fn qtt(part: Part) Result {
  if (part.a > 3426 ){ return Result.Reject; }
  if (part.s > 2343 ){ return Result.Reject; }
return Result.Accept;

}


fn xk(part: Part) Result {
  if (part.m > 1954 ){ return Result.Reject; }
  if (part.x > 147 ){ return tb(part); }
  if (part.s > 1982 ){ return Result.Accept; }
return Result.Reject;

}


fn tcq(part: Part) Result {
  if (part.a < 3492 ){ return Result.Accept; }
  if (part.m > 3560 ){ return Result.Reject; }
  if (part.m > 3445 ){ return Result.Reject; }
{ return pfl(part); }

}


fn gqn(part: Part) Result {
  if (part.m < 1012 ){ return Result.Reject; }
  if (part.a > 3809 ){ return Result.Accept; }
  if (part.m > 1538 ){ return Result.Reject; }
return Result.Reject;

}


fn tl(part: Part) Result {
  if (part.x < 1387 ){ return Result.Accept; }
  if (part.s > 507 ){ return Result.Accept; }
  if (part.a > 1154 ){ return hlp(part); }
return Result.Reject;

}


fn nx(part: Part) Result {
  if (part.m > 3530 ){ return Result.Reject; }
  if (part.x > 1938 ){ return Result.Accept; }
  if (part.x < 869 ){ return Result.Accept; }
return Result.Accept;

}


fn kdz(part: Part) Result {
  if (part.m < 3373 ){ return jsb(part); }
{ return rr(part); }

}


fn ml(part: Part) Result {
  if (part.x < 2518 ){ return Result.Reject; }
  if (part.x > 3029 ){ return gq(part); }
{ return tgq(part); }

}


fn kt(part: Part) Result {
  if (part.a > 1756 ){ return hmd(part); }
  if (part.s < 2998 ){ return rqc(part); }
  if (part.s > 3418 ){ return fbb(part); }
{ return dg(part); }

}


fn fcd(part: Part) Result {
  if (part.s > 2103 ){ return Result.Reject; }
  if (part.x > 1291 ){ return Result.Accept; }
  if (part.x > 1090 ){ return Result.Reject; }
return Result.Accept;

}


fn jmb(part: Part) Result {
  if (part.s < 2218 ){ return Result.Accept; }
return Result.Accept;

}


fn xh(part: Part) Result {
  if (part.m > 2615 ){ return zvk(part); }
  if (part.s > 1163 ){ return kb(part); }
  if (part.x > 1182 ){ return tn(part); }
{ return djl(part); }

}


fn kfk(part: Part) Result {
  if (part.m > 2088 ){ return Result.Reject; }
  if (part.x > 3568 ){ return Result.Reject; }
  if (part.s > 2931 ){ return Result.Reject; }
return Result.Accept;

}


fn jlf(part: Part) Result {
  if (part.a < 2894 ){ return bn(part); }
{ return gs(part); }

}


fn dx(part: Part) Result {
  if (part.a < 3584 ){ return Result.Accept; }
  if (part.m < 1879 ){ return Result.Reject; }
  if (part.s > 950 ){ return Result.Reject; }
return Result.Reject;

}


fn cjl(part: Part) Result {
  if (part.s > 3051 ){ return Result.Reject; }
  if (part.s > 2781 ){ return Result.Reject; }
return Result.Accept;

}


fn qjm(part: Part) Result {
  if (part.s < 2864 ){ return Result.Reject; }
  if (part.s < 3058 ){ return Result.Accept; }
  if (part.s < 3168 ){ return Result.Accept; }
return Result.Reject;

}


fn hls(part: Part) Result {
  if (part.m > 1767 ){ return Result.Reject; }
  if (part.s > 547 ){ return Result.Accept; }
return Result.Reject;

}


fn xkt(part: Part) Result {
  if (part.x < 582 ){ return Result.Reject; }
  if (part.a < 3359 ){ return Result.Accept; }
{ return sln(part); }

}


fn sn(part: Part) Result {
  if (part.x < 349 ){ return Result.Reject; }
  if (part.m < 1786 ){ return Result.Reject; }
  if (part.s < 3265 ){ return Result.Accept; }
return Result.Accept;

}


fn qzd(part: Part) Result {
  if (part.x < 3510 ){ return tc(part); }
  if (part.a > 1081 ){ return zv(part); }
  if (part.m < 2187 ){ return xnm(part); }
{ return prg(part); }

}


fn ss(part: Part) Result {
  if (part.s < 1133 ){ return dd(part); }
  if (part.x > 761 ){ return pl(part); }
  if (part.x < 318 ){ return gmx(part); }
{ return txc(part); }

}


fn tpr(part: Part) Result {
  if (part.m > 1351 ){ return tk(part); }
  if (part.m < 552 ){ return fbc(part); }
  if (part.s > 3364 ){ return zg(part); }
{ return gqr(part); }

}


fn zvk(part: Part) Result {
  if (part.x > 1284 ){ return fl(part); }
{ return zk(part); }

}


fn tjz(part: Part) Result {
  if (part.s < 1934 ){ return mvj(part); }
  if (part.a > 774 ){ return pd(part); }
return Result.Accept;

}


fn cjn(part: Part) Result {
  if (part.m > 63 ){ return Result.Reject; }
  if (part.x > 2722 ){ return Result.Reject; }
  if (part.x > 2262 ){ return Result.Reject; }
return Result.Reject;

}


fn vb(part: Part) Result {
  if (part.s < 2093 ){ return Result.Accept; }
  if (part.s < 2443 ){ return Result.Reject; }
return Result.Reject;

}


fn jdf(part: Part) Result {
  if (part.s > 2258 ){ return Result.Reject; }
  if (part.m < 3176 ){ return Result.Reject; }
return Result.Accept;

}


fn tp(part: Part) Result {
  if (part.m < 2941 ){ return vr(part); }
  if (part.m > 3545 ){ return rp(part); }
  if (part.a < 2546 ){ return Result.Accept; }
return Result.Reject;

}


fn psh(part: Part) Result {
  if (part.x < 2525 ){ return Result.Reject; }
return Result.Reject;

}


fn bn(part: Part) Result {
  if (part.m < 809 ){ return blf(part); }
{ return vx(part); }

}


fn hmd(part: Part) Result {
  if (part.s < 2914 ){ return lgz(part); }
  if (part.s < 3327 ){ return Result.Accept; }
return Result.Accept;

}


fn zk(part: Part) Result {
  if (part.x > 1080 ){ return ln(part); }
return Result.Accept;

}


fn pcm(part: Part) Result {
  if (part.x < 3181 ){ return qb(part); }
{ return mq(part); }

}


fn hkh(part: Part) Result {
  if (part.x > 820 ){ return flc(part); }
  if (part.s > 2964 ){ return zz(part); }
{ return snj(part); }

}


fn qhx(part: Part) Result {
  if (part.s < 1186 ){ return llv(part); }
{ return knx(part); }

}


fn xq(part: Part) Result {
  if (part.x > 732 ){ return Result.Reject; }
  if (part.a > 497 ){ return Result.Reject; }
return Result.Reject;

}


fn md(part: Part) Result {
  if (part.s < 746 ){ return vhg(part); }
  if (part.s > 1108 ){ return Result.Accept; }
{ return zn(part); }

}


fn jfj(part: Part) Result {
  if (part.a < 3585 ){ return pmn(part); }
  if (part.m < 2102 ){ return kq(part); }
{ return kk(part); }

}


fn tn(part: Part) Result {
  if (part.x > 1417 ){ return nf(part); }
  if (part.x < 1275 ){ return rbj(part); }
  if (part.x < 1348 ){ return cj(part); }
{ return tl(part); }

}


fn jtb(part: Part) Result {
  if (part.s > 2342 ){ return Result.Accept; }
return Result.Accept;

}


fn htb(part: Part) Result {
  if (part.x > 1845 ){ return qhx(part); }
  if (part.s > 1570 ){ return fxj(part); }
  if (part.x > 642 ){ return hzj(part); }
{ return sg(part); }

}


fn xzx(part: Part) Result {
  if (part.a < 3864 ){ return Result.Reject; }
return Result.Accept;

}


fn ctx(part: Part) Result {
  if (part.s < 1747 ){ return Result.Reject; }
  if (part.s > 2039 ){ return qtt(part); }
  if (part.a < 3465 ){ return sv(part); }
{ return qcb(part); }

}


fn cp(part: Part) Result {
  if (part.s > 2998 ){ return Result.Reject; }
{ return bg(part); }

}


fn jdn(part: Part) Result {
  if (part.a > 1087 ){ return Result.Accept; }
  if (part.m > 1336 ){ return rb(part); }
  if (part.s > 368 ){ return Result.Accept; }
return Result.Accept;

}


fn kqc(part: Part) Result {
  if (part.s > 3820 ){ return Result.Accept; }
  if (part.s > 3746 ){ return jsr(part); }
  if (part.a < 661 ){ return Result.Reject; }
return Result.Reject;

}


fn sdp(part: Part) Result {
  if (part.m < 134 ){ return cjn(part); }
{ return gjc(part); }

}


fn grm(part: Part) Result {
  if (part.s < 1777 ){ return Result.Accept; }
  if (part.s > 2023 ){ return Result.Reject; }
return Result.Accept;

}


fn rh(part: Part) Result {
  if (part.a < 1429 ){ return Result.Reject; }
  if (part.m > 1202 ){ return xbt(part); }
return Result.Accept;

}


fn sx(part: Part) Result {
  if (part.s < 2106 ){ return Result.Reject; }
  if (part.a < 2817 ){ return Result.Accept; }
return Result.Accept;

}


fn bv(part: Part) Result {
  if (part.m < 392 ){ return Result.Reject; }
  if (part.m < 603 ){ return rv(part); }
  if (part.a < 3626 ){ return Result.Accept; }
return Result.Reject;

}


fn gp(part: Part) Result {
  if (part.m < 2914 ){ return Result.Reject; }
return Result.Reject;

}


fn kl(part: Part) Result {
  if (part.a > 671 ){ return Result.Accept; }
  if (part.m < 2467 ){ return Result.Accept; }
return Result.Accept;

}


fn zz(part: Part) Result {
  if (part.x < 401 ){ return Result.Reject; }
return Result.Reject;

}


fn bkr(part: Part) Result {
  if (part.s < 3356 ){ return hcj(part); }
  if (part.s > 3613 ){ return cvt(part); }
  if (part.s > 3493 ){ return csj(part); }
{ return mx(part); }

}


fn tq(part: Part) Result {
  if (part.m < 2648 ){ return Result.Accept; }
return Result.Accept;

}


fn xkg(part: Part) Result {
  if (part.m > 298 ){ return Result.Reject; }
return Result.Accept;

}


fn spc(part: Part) Result {
  if (part.m < 2028 ){ return Result.Reject; }
return Result.Accept;

}


fn smb(part: Part) Result {
  if (part.a < 1426 ){ return Result.Accept; }
return Result.Accept;

}


fn pl(part: Part) Result {
  if (part.m > 1383 ){ return trr(part); }
  if (part.m > 495 ){ return rl(part); }
{ return pv(part); }

}


fn kqb(part: Part) Result {
  if (part.x < 3061 ){ return xqj(part); }
  if (part.a > 2190 ){ return hls(part); }
{ return bt(part); }

}


fn xqz(part: Part) Result {
  if (part.x > 2594 ){ return kgc(part); }
{ return bxc(part); }

}


fn hxk(part: Part) Result {
  if (part.m < 2514 ){ return Result.Reject; }
  if (part.a < 2582 ){ return Result.Accept; }
  if (part.x < 244 ){ return Result.Accept; }
return Result.Reject;

}


fn fv(part: Part) Result {
  if (part.x > 471 ){ return Result.Reject; }
  if (part.x < 224 ){ return Result.Accept; }
  if (part.a > 3501 ){ return Result.Reject; }
return Result.Reject;

}


fn dpq(part: Part) Result {
  if (part.x < 3568 ){ return Result.Reject; }
  if (part.m < 2617 ){ return Result.Reject; }
  if (part.s > 1524 ){ return Result.Accept; }
return Result.Reject;

}


fn csj(part: Part) Result {
  if (part.x > 3460 ){ return Result.Accept; }
  if (part.m < 1399 ){ return Result.Reject; }
  if (part.s < 3546 ){ return Result.Reject; }
return Result.Accept;

}


fn in(part: Part) Result {
  if (part.a < 1956 ){ return sb(part); }
{ return rg(part); }

}


fn sk(part: Part) Result {
  if (part.s < 1232 ){ return Result.Accept; }
  if (part.x > 1125 ){ return Result.Accept; }
return Result.Reject;

}


fn qcc(part: Part) Result {
  if (part.s < 2678 ){ return Result.Reject; }
return Result.Reject;

}


fn vsr(part: Part) Result {
  if (part.x > 2791 ){ return bkr(part); }
  if (part.m > 1454 ){ return qhv(part); }
{ return zjg(part); }

}


fn sm(part: Part) Result {
  if (part.m > 871 ){ return Result.Accept; }
{ return nrk(part); }

}


fn zjg(part: Part) Result {
  if (part.a > 3658 ){ return Result.Accept; }
return Result.Reject;

}


fn pjd(part: Part) Result {
  if (part.a < 570 ){ return Result.Reject; }
  if (part.s < 3022 ){ return spc(part); }
{ return ppj(part); }

}


fn zlt(part: Part) Result {
  if (part.x < 2479 ){ return Result.Reject; }
  if (part.x < 2620 ){ return jdf(part); }
  if (part.a < 2991 ){ return Result.Accept; }
{ return xc(part); }

}


fn cmn(part: Part) Result {
  if (part.x > 2531 ){ return Result.Accept; }
{ return jhc(part); }

}


fn kxh(part: Part) Result {
  if (part.s < 3653 ){ return Result.Accept; }
return Result.Accept;

}


fn fpm(part: Part) Result {
  if (part.m > 1170 ){ return sk(part); }
{ return fk(part); }

}


fn rlk(part: Part) Result {
  if (part.a > 3634 ){ return bjh(part); }
return Result.Reject;

}


fn kgc(part: Part) Result {
  if (part.s < 3202 ){ return gh(part); }
  if (part.x > 3150 ){ return xzg(part); }
  if (part.m > 3300 ){ return jgc(part); }
{ return rpz(part); }

}


fn kzf(part: Part) Result {
  if (part.s > 1677 ){ return Result.Reject; }
  if (part.a > 3194 ){ return Result.Accept; }
return Result.Reject;

}


fn mqj(part: Part) Result {
  if (part.s > 1334 ){ return Result.Reject; }
return Result.Accept;

}


fn nkv(part: Part) Result {
  if (part.x < 1052 ){ return Result.Reject; }
  if (part.m > 1697 ){ return Result.Accept; }
  if (part.m > 777 ){ return Result.Reject; }
return Result.Reject;

}


fn ksk(part: Part) Result {
  if (part.a < 1132 ){ return Result.Reject; }
  if (part.m < 2906 ){ return gpb(part); }
{ return fdj(part); }

}


fn zkf(part: Part) Result {
  if (part.s > 3613 ){ return Result.Accept; }
return Result.Accept;

}


fn lfd(part: Part) Result {
  if (part.s > 581 ){ return kdx(part); }
{ return zj(part); }

}


fn lt(part: Part) Result {
  if (part.a > 368 ){ return Result.Reject; }
return Result.Reject;

}


fn ck(part: Part) Result {
  if (part.s > 964 ){ return xzk(part); }
  if (part.m > 785 ){ return Result.Accept; }
  if (part.x > 1423 ){ return Result.Reject; }
return Result.Accept;

}


fn csx(part: Part) Result {
  if (part.a > 1114 ){ return tfl(part); }
  if (part.s < 2655 ){ return dj(part); }
  if (part.x > 1068 ){ return ljf(part); }
{ return hnc(part); }

}


fn jbp(part: Part) Result {
  if (part.s < 2130 ){ return eric(part); }
  if (part.m < 2593 ){ return dc(part); }
  if (part.m > 3491 ){ return Result.Reject; }
{ return mkg(part); }

}


fn bsv(part: Part) Result {
  if (part.s < 1794 ){ return Result.Reject; }
  if (part.a < 3778 ){ return Result.Reject; }
  if (part.m > 865 ){ return Result.Reject; }
return Result.Reject;

}


fn slp(part: Part) Result {
  if (part.x < 225 ){ return qvl(part); }
  if (part.x < 484 ){ return Result.Accept; }
  if (part.x > 544 ){ return thh(part); }
{ return gqn(part); }

}


fn bc(part: Part) Result {
  if (part.x < 677 ){ return Result.Accept; }
  if (part.x < 715 ){ return Result.Reject; }
return Result.Accept;

}


fn fbb(part: Part) Result {
  if (part.a > 1662 ){ return Result.Reject; }
  if (part.a > 1596 ){ return psh(part); }
{ return cgx(part); }

}


fn gs(part: Part) Result {
  if (part.a < 3106 ){ return Result.Reject; }
  if (part.a < 3226 ){ return Result.Reject; }
  if (part.x > 2681 ){ return jjm(part); }
return Result.Reject;

}


fn jr(part: Part) Result {
  if (part.m > 2097 ){ return Result.Accept; }
return Result.Accept;

}


fn bzv(part: Part) Result {
  if (part.s > 1539 ){ return Result.Reject; }
  if (part.m > 3394 ){ return Result.Accept; }
  if (part.s > 1339 ){ return Result.Accept; }
return Result.Reject;

}


fn cr(part: Part) Result {
  if (part.m < 901 ){ return Result.Accept; }
return Result.Reject;

}


fn tbs(part: Part) Result {
  if (part.m > 3290 ){ return Result.Accept; }
  if (part.x < 3434 ){ return Result.Reject; }
return Result.Reject;

}


fn nxr(part: Part) Result {
  if (part.s < 960 ){ return Result.Reject; }
  if (part.m < 838 ){ return Result.Accept; }
return Result.Reject;

}


fn vd(part: Part) Result {
  if (part.m > 1343 ){ return Result.Reject; }
  if (part.s < 3441 ){ return Result.Accept; }
return Result.Reject;

}


fn mn(part: Part) Result {
  if (part.s < 686 ){ return Result.Reject; }
  if (part.m < 3160 ){ return Result.Reject; }
  if (part.a > 3425 ){ return Result.Accept; }
return Result.Reject;

}


fn ccl(part: Part) Result {
  if (part.a < 3457 ){ return Result.Reject; }
  if (part.a < 3466 ){ return Result.Reject; }
  if (part.s < 3848 ){ return Result.Reject; }
return Result.Reject;

}


fn jgc(part: Part) Result {
  if (part.a > 3715 ){ return Result.Accept; }
  if (part.m < 3679 ){ return Result.Accept; }
  if (part.m < 3808 ){ return vxd(part); }
{ return qsv(part); }

}


fn hkz(part: Part) Result {
  if (part.a > 914 ){ return Result.Reject; }
return Result.Reject;

}


fn ztm(part: Part) Result {
  if (part.x < 1659 ){ return cd(part); }
  if (part.m > 1673 ){ return rmb(part); }
  if (part.x > 2680 ){ return ndj(part); }
{ return dkc(part); }

}


fn xs(part: Part) Result {
  if (part.s < 698 ){ return Result.Reject; }
return Result.Reject;

}


fn lqj(part: Part) Result {
  if (part.x < 2698 ){ return Result.Accept; }
  if (part.a < 3502 ){ return Result.Accept; }
  if (part.a < 3573 ){ return xjc(part); }
{ return phd(part); }

}


fn trf(part: Part) Result {
  if (part.x > 1590 ){ return Result.Accept; }
  if (part.x > 591 ){ return zqx(part); }
{ return gl(part); }

}


fn flk(part: Part) Result {
  if (part.x > 1543 ){ return Result.Accept; }
  if (part.m > 3077 ){ return Result.Reject; }
return Result.Reject;

}


fn jrx(part: Part) Result {
  if (part.x > 991 ){ return Result.Reject; }
  if (part.m > 228 ){ return Result.Accept; }
  if (part.x < 558 ){ return Result.Reject; }
{ return xq(part); }

}


fn pc(part: Part) Result {
  if (part.m > 3083 ){ return Result.Reject; }
  if (part.s > 1723 ){ return Result.Accept; }
return Result.Reject;

}


fn nxn(part: Part) Result {
  if (part.x < 1563 ){ return Result.Reject; }
return Result.Reject;

}


fn llv(part: Part) Result {
  if (part.m > 1476 ){ return kcs(part); }
return Result.Reject;

}


fn xzg(part: Part) Result {
  if (part.x > 3604 ){ return pjh(part); }
  if (part.a < 3650 ){ return tbs(part); }
  if (part.a < 3852 ){ return zkf(part); }
{ return jv(part); }

}


fn fck(part: Part) Result {
  if (part.m < 898 ){ return Result.Accept; }
  if (part.s > 1000 ){ return Result.Reject; }
return Result.Accept;

}


fn zbs(part: Part) Result {
  if (part.s < 1558 ){ return gt(part); }
return Result.Accept;

}


fn dcj(part: Part) Result {
  if (part.a > 416 ){ return Result.Reject; }
  if (part.s < 3371 ){ return nx(part); }
return Result.Reject;

}


fn tdv(part: Part) Result {
  if (part.x < 653 ){ return slp(part); }
{ return krn(part); }

}


fn xg(part: Part) Result {
  if (part.a < 3721 ){ return Result.Reject; }
return Result.Accept;

}


fn qb(part: Part) Result {
  if (part.x > 2328 ){ return Result.Reject; }
{ return mgd(part); }

}


fn tlb(part: Part) Result {
  if (part.s < 3324 ){ return qjm(part); }
{ return xpg(part); }

}


fn phd(part: Part) Result {
  if (part.x > 3311 ){ return Result.Accept; }
  if (part.a > 3597 ){ return Result.Accept; }
return Result.Accept;

}


fn qn(part: Part) Result {
  if (part.m > 2725 ){ return Result.Reject; }
  if (part.x > 3045 ){ return tq(part); }
{ return gdq(part); }

}


fn ksx(part: Part) Result {
  if (part.x > 3314 ){ return Result.Reject; }
  if (part.a > 3118 ){ return Result.Accept; }
return Result.Reject;

}


fn bz(part: Part) Result {
  if (part.s > 1202 ){ return fx(part); }
  if (part.m < 3060 ){ return Result.Accept; }
{ return mn(part); }

}


fn qm(part: Part) Result {
  if (part.a < 1033 ){ return Result.Accept; }
  if (part.s < 455 ){ return Result.Accept; }
return Result.Accept;

}


fn lgs(part: Part) Result {
  if (part.a > 1323 ){ return Result.Reject; }
  if (part.m > 1667 ){ return Result.Reject; }
  if (part.s > 1125 ){ return Result.Accept; }
return Result.Accept;

}


fn xqj(part: Part) Result {
  if (part.s > 719 ){ return Result.Accept; }
  if (part.m < 2334 ){ return Result.Reject; }
  if (part.x < 2369 ){ return sd(part); }
return Result.Reject;

}


fn hn(part: Part) Result {
  if (part.m < 2879 ){ return Result.Accept; }
  if (part.m < 3555 ){ return fp(part); }
return Result.Accept;

}


fn eric(part: Part) Result {
  if (part.m < 2496 ){ return Result.Reject; }
  if (part.x < 305 ){ return Result.Reject; }
return Result.Accept;

}


fn km(part: Part) Result {
  if (part.s < 1129 ){ return Result.Reject; }
  if (part.s < 1956 ){ return Result.Reject; }
return Result.Accept;

}


fn pfl(part: Part) Result {
  if (part.s < 608 ){ return Result.Accept; }
return Result.Reject;

}


fn zjf(part: Part) Result {
  if (part.s < 3517 ){ return Result.Reject; }
return Result.Reject;

}


fn nrd(part: Part) Result {
  if (part.x > 514 ){ return Result.Accept; }
  if (part.a > 2615 ){ return Result.Reject; }
return Result.Accept;

}


fn bjh(part: Part) Result {
  if (part.a < 3790 ){ return Result.Reject; }
  if (part.a > 3908 ){ return Result.Reject; }
return Result.Accept;

}


fn cj(part: Part) Result {
  if (part.m < 1048 ){ return jd(part); }
  if (part.x < 1318 ){ return Result.Accept; }
{ return qm(part); }

}


fn qj(part: Part) Result {
  if (part.x > 2393 ){ return Result.Accept; }
  if (part.x < 2306 ){ return Result.Reject; }
return Result.Reject;

}


fn bj(part: Part) Result {
  if (part.x > 2231 ){ return qj(part); }
  if (part.m < 3336 ){ return tlk(part); }
return Result.Reject;

}


fn mx(part: Part) Result {
  if (part.s < 3403 ){ return Result.Accept; }
  if (part.m > 1473 ){ return Result.Reject; }
  if (part.x < 3353 ){ return Result.Reject; }
return Result.Accept;

}


fn qnl(part: Part) Result {
  if (part.x < 645 ){ return Result.Reject; }
return Result.Reject;

}


fn bxc(part: Part) Result {
  if (part.s < 3182 ){ return bj(part); }
  if (part.m > 2939 ){ return dlt(part); }
{ return fr(part); }

}


fn ld(part: Part) Result {
  if (part.m < 2418 ){ return js(part); }
  if (part.m < 3245 ){ return dcv(part); }
{ return mj(part); }

}


fn zhq(part: Part) Result {
  if (part.x > 1338 ){ return flk(part); }
  if (part.a < 3958 ){ return Result.Accept; }
{ return rft(part); }

}


fn vxd(part: Part) Result {
  if (part.a > 3538 ){ return Result.Accept; }
  if (part.a > 3435 ){ return Result.Reject; }
  if (part.x < 2898 ){ return Result.Reject; }
return Result.Reject;

}


fn prg(part: Part) Result {
  if (part.x < 3701 ){ return Result.Accept; }
  if (part.x < 3802 ){ return Result.Reject; }
  if (part.a > 923 ){ return qkk(part); }
return Result.Reject;

}


fn tlk(part: Part) Result {
  if (part.x < 1951 ){ return Result.Accept; }
return Result.Accept;

}


fn ft(part: Part) Result {
  if (part.x < 2160 ){ return Result.Reject; }
return Result.Accept;

}


fn ts(part: Part) Result {
  if (part.m < 2542 ){ return Result.Accept; }
  if (part.a > 3465 ){ return gqz(part); }
{ return cjl(part); }

}


fn pjh(part: Part) Result {
  if (part.m > 3374 ){ return Result.Accept; }
  if (part.s > 3506 ){ return Result.Reject; }
return Result.Reject;

}


fn qvl(part: Part) Result {
  if (part.m < 1213 ){ return Result.Accept; }
  if (part.s > 3748 ){ return Result.Reject; }
return Result.Accept;

}


fn tng(part: Part) Result {
  if (part.s < 3427 ){ return Result.Reject; }
  if (part.x > 280 ){ return Result.Accept; }
return Result.Accept;

}


fn rmb(part: Part) Result {
  if (part.m > 3185 ){ return Result.Reject; }
return Result.Reject;

}


fn cf(part: Part) Result {
  if (part.s < 433 ){ return Result.Reject; }
  if (part.a > 545 ){ return Result.Accept; }
  if (part.s > 524 ){ return Result.Reject; }
return Result.Accept;

}


fn cd(part: Part) Result {
  if (part.a > 1023 ){ return Result.Reject; }
{ return xt(part); }

}


fn sq(part: Part) Result {
  if (part.s > 3800 ){ return Result.Accept; }
  if (part.a > 2788 ){ return Result.Accept; }
  if (part.x > 324 ){ return nrd(part); }
return Result.Accept;

}


fn fjr(part: Part) Result {
  if (part.s > 2817 ){ return tql(part); }
{ return rn(part); }

}


fn tvz(part: Part) Result {
  if (part.s > 2907 ){ return Result.Reject; }
  if (part.x < 1406 ){ return Result.Reject; }
return Result.Reject;

}


fn lv(part: Part) Result {
  if (part.x > 2755 ){ return ksx(part); }
  if (part.a < 3080 ){ return zlt(part); }
{ return fg(part); }

}


fn qd(part: Part) Result {
  if (part.s < 2827 ){ return Result.Reject; }
  if (part.m < 3387 ){ return Result.Accept; }
  if (part.s > 2880 ){ return Result.Accept; }
return Result.Reject;

}


fn fp(part: Part) Result {
  if (part.s > 563 ){ return Result.Accept; }
  if (part.s > 328 ){ return Result.Reject; }
  if (part.a < 3777 ){ return Result.Reject; }
return Result.Accept;

}


fn ttr(part: Part) Result {
  if (part.s < 1609 ){ return Result.Accept; }
return Result.Reject;

}


fn pkg(part: Part) Result {
  if (part.s < 3788 ){ return Result.Accept; }
return Result.Reject;

}


fn rn(part: Part) Result {
  if (part.s < 2437 ){ return dgm(part); }
  if (part.x > 1172 ){ return dgt(part); }
  if (part.m > 2399 ){ return jvf(part); }
{ return txl(part); }

}


fn lhk(part: Part) Result {
  if (part.a < 2938 ){ return Result.Reject; }
  if (part.s > 2649 ){ return Result.Reject; }
  if (part.x > 600 ){ return Result.Accept; }
return Result.Reject;

}


fn mbv(part: Part) Result {
  if (part.s > 488 ){ return Result.Reject; }
return Result.Reject;

}


fn jgn(part: Part) Result {
  if (part.x < 1025 ){ return Result.Reject; }
return Result.Accept;

}


fn rb(part: Part) Result {
  if (part.s > 249 ){ return Result.Reject; }
  if (part.s < 150 ){ return Result.Reject; }
  if (part.a > 376 ){ return Result.Reject; }
return Result.Accept;

}


fn mmd(part: Part) Result {
  if (part.x > 985 ){ return Result.Accept; }
  if (part.m < 1056 ){ return Result.Accept; }
  if (part.m > 1218 ){ return Result.Reject; }
return Result.Reject;

}


fn vzm(part: Part) Result {
  if (part.a > 1483 ){ return Result.Reject; }
  if (part.x > 3114 ){ return Result.Accept; }
  if (part.a > 1161 ){ return dtd(part); }
return Result.Accept;

}


fn khg(part: Part) Result {
  if (part.s > 1259 ){ return Result.Accept; }
return Result.Accept;

}


fn zcp(part: Part) Result {
  if (part.a < 538 ){ return cg(part); }
  if (part.x < 478 ){ return fb(part); }
{ return tjz(part); }

}


fn mr(part: Part) Result {
  if (part.s > 3767 ){ return Result.Accept; }
  if (part.m < 3324 ){ return Result.Accept; }
  if (part.m > 3758 ){ return Result.Accept; }
return Result.Accept;

}


fn ps(part: Part) Result {
  if (part.s > 3118 ){ return ndp(part); }
  if (part.m > 2406 ){ return gm(part); }
{ return xkt(part); }

}


fn kb(part: Part) Result {
  if (part.s > 1672 ){ return lff(part); }
  if (part.s > 1412 ){ return xv(part); }
{ return mkm(part); }

}


fn gcc(part: Part) Result {
  if (part.a > 3486 ){ return Result.Accept; }
  if (part.m > 646 ){ return Result.Reject; }
  if (part.x > 324 ){ return Result.Reject; }
return Result.Accept;

}


fn vx(part: Part) Result {
  if (part.x < 2670 ){ return Result.Reject; }
  if (part.s > 1615 ){ return Result.Reject; }
return Result.Accept;

}


fn vjv(part: Part) Result {
  if (part.a < 3927 ){ return vmm(part); }
  if (part.s < 3271 ){ return Result.Reject; }
  if (part.x < 507 ){ return vz(part); }
return Result.Reject;

}


fn ldt(part: Part) Result {
  if (part.x < 1959 ){ return Result.Reject; }
  if (part.a < 3595 ){ return Result.Accept; }
  if (part.m < 2652 ){ return Result.Accept; }
return Result.Accept;

}


fn gqv(part: Part) Result {
  if (part.x > 2422 ){ return Result.Accept; }
return Result.Reject;

}


fn bqt(part: Part) Result {
  if (part.x > 721 ){ return Result.Reject; }
  if (part.s > 1726 ){ return fv(part); }
  if (part.a > 3499 ){ return fck(part); }
{ return gcc(part); }

}


fn fxj(part: Part) Result {
  if (part.m < 1753 ){ return nng(part); }
  if (part.s > 2037 ){ return rd(part); }
{ return blz(part); }

}


fn fm(part: Part) Result {
  if (part.x < 3344 ){ return Result.Reject; }
  if (part.m < 2096 ){ return Result.Accept; }
  if (part.s < 1520 ){ return Result.Accept; }
return Result.Reject;

}


fn ksj(part: Part) Result {
  if (part.s > 1619 ){ return Result.Reject; }
  if (part.a < 2071 ){ return Result.Accept; }
  if (part.m < 3366 ){ return Result.Reject; }
return Result.Reject;

}


fn xp(part: Part) Result {
  if (part.m < 841 ){ return Result.Reject; }
  if (part.s > 2782 ){ return Result.Reject; }
return Result.Accept;

}


fn jvf(part: Part) Result {
  if (part.m < 3084 ){ return lhk(part); }
  if (part.a < 2805 ){ return gdx(part); }
  if (part.x < 584 ){ return Result.Accept; }
return Result.Accept;

}


fn tr(part: Part) Result {
  if (part.m > 926 ){ return Result.Accept; }
  if (part.m > 715 ){ return Result.Reject; }
return Result.Accept;

}


fn xx(part: Part) Result {
  if (part.a < 2562 ){ return Result.Reject; }
  if (part.x < 210 ){ return Result.Accept; }
  if (part.s > 2650 ){ return Result.Reject; }
return Result.Accept;

}


fn fg(part: Part) Result {
  if (part.s < 2357 ){ return db(part); }
return Result.Reject;

}


fn mj(part: Part) Result {
  if (part.s > 1442 ){ return pq(part); }
{ return tcq(part); }

}


fn mnj(part: Part) Result {
  if (part.a < 3751 ){ return sp(part); }
  if (part.a < 3862 ){ return jj(part); }
{ return vjv(part); }

}


fn rf(part: Part) Result {
  if (part.a < 3143 ){ return jcj(part); }
  if (part.a < 3223 ){ return kzf(part); }
return Result.Accept;

}


fn vdd(part: Part) Result {
  if (part.s > 2404 ){ return mvq(part); }
  if (part.m > 2462 ){ return glb(part); }
  if (part.a > 2266 ){ return zc(part); }
{ return ngf(part); }

}


fn vpv(part: Part) Result {
  if (part.x > 1794 ){ return sxn(part); }
return Result.Reject;

}


fn nng(part: Part) Result {
  if (part.s < 1956 ){ return bsv(part); }
return Result.Reject;

}


fn bxj(part: Part) Result {
  if (part.a < 3760 ){ return Result.Reject; }
return Result.Reject;

}


fn kc(part: Part) Result {
  if (part.m < 365 ){ return Result.Reject; }
  if (part.a < 303 ){ return Result.Accept; }
return Result.Accept;

}


fn gnz(part: Part) Result {
  if (part.a < 3530 ){ return bqt(part); }
{ return bpq(part); }

}


fn rmd(part: Part) Result {
  if (part.x < 457 ){ return Result.Reject; }
  if (part.s < 2349 ){ return Result.Accept; }
return Result.Reject;

}


fn lff(part: Part) Result {
  if (part.m > 1299 ){ return fcd(part); }
  if (part.s > 2022 ){ return Result.Accept; }
return Result.Reject;

}


fn hkk(part: Part) Result {
  if (part.a > 2726 ){ return Result.Reject; }
  if (part.m > 2110 ){ return Result.Accept; }
return Result.Reject;

}


fn fnv(part: Part) Result {
  if (part.x < 1108 ){ return Result.Reject; }
return Result.Reject;

}


fn hs(part: Part) Result {
  if (part.s > 1630 ){ return Result.Reject; }
return Result.Reject;

}


fn mhq(part: Part) Result {
  if (part.x > 1097 ){ return Result.Reject; }
  if (part.s > 2746 ){ return Result.Reject; }
return Result.Reject;

}


fn bjd(part: Part) Result {
  if (part.s < 2983 ){ return Result.Reject; }
  if (part.m > 1257 ){ return Result.Reject; }
  if (part.x < 1606 ){ return Result.Accept; }
return Result.Accept;

}


fn bdm(part: Part) Result {
  if (part.x < 1093 ){ return Result.Reject; }
  if (part.s > 1562 ){ return Result.Reject; }
return Result.Accept;

}


fn nt(part: Part) Result {
  if (part.s < 1871 ){ return xd(part); }
{ return jbp(part); }

}


fn bp(part: Part) Result {
  if (part.m > 2851 ){ return Result.Accept; }
return Result.Reject;

}


fn tg(part: Part) Result {
  if (part.s < 554 ){ return Result.Accept; }
return Result.Accept;

}


fn djl(part: Part) Result {
  if (part.a > 1217 ){ return mbv(part); }
  if (part.x < 1017 ){ return qq(part); }
  if (part.m < 1343 ){ return krr(part); }
{ return rqk(part); }

}


fn zjm(part: Part) Result {
  if (part.a < 2755 ){ return Result.Accept; }
  if (part.s < 1895 ){ return Result.Reject; }
return Result.Reject;

}


fn mvq(part: Part) Result {
  if (part.m < 2343 ){ return Result.Reject; }
  if (part.s < 3313 ){ return zsg(part); }
{ return mr(part); }

}


fn qq(part: Part) Result {
  if (part.a > 744 ){ return Result.Accept; }
  if (part.x > 932 ){ return Result.Reject; }
  if (part.m > 1338 ){ return lt(part); }
return Result.Reject;

}


fn dcv(part: Part) Result {
  if (part.x < 1885 ){ return dm(part); }
  if (part.m > 2933 ){ return bz(part); }
  if (part.m < 2601 ){ return lqj(part); }
{ return qn(part); }

}


fn bg(part: Part) Result {
  if (part.s < 2818 ){ return Result.Accept; }
return Result.Reject;

}


fn hp(part: Part) Result {
  if (part.a > 3634 ){ return htb(part); }
  if (part.m > 1588 ){ return ld(part); }
{ return bd(part); }

}


fn dd(part: Part) Result {
  if (part.m > 2431 ){ return dbp(part); }
  if (part.x < 1116 ){ return lfd(part); }
  if (part.a > 2843 ){ return xld(part); }
{ return fh(part); }

}


fn xc(part: Part) Result {
  if (part.x < 2692 ){ return Result.Reject; }
return Result.Reject;

}


fn kq(part: Part) Result {
  if (part.s < 3415 ){ return hkh(part); }
{ return tdv(part); }

}


fn tc(part: Part) Result {
  if (part.a > 964 ){ return pz(part); }
  if (part.x < 3150 ){ return nr(part); }
  if (part.s > 2586 ){ return Result.Accept; }
return Result.Reject;

}


fn jd(part: Part) Result {
  if (part.a > 1212 ){ return Result.Accept; }
  if (part.s > 686 ){ return Result.Reject; }
  if (part.a < 735 ){ return Result.Accept; }
return Result.Reject;

}


fn tsn(part: Part) Result {
  if (part.s < 3080 ){ return hkk(part); }
  if (part.a < 2685 ){ return hxk(part); }
{ return sn(part); }

}


fn ncq(part: Part) Result {
  if (part.m < 1020 ){ return kzx(part); }
return Result.Reject;

}


fn mp(part: Part) Result {
  if (part.m > 3139 ){ return Result.Accept; }
  if (part.x > 328 ){ return Result.Accept; }
  if (part.s < 3643 ){ return Result.Reject; }
return Result.Reject;

}


fn bxl(part: Part) Result {
  if (part.m < 3137 ){ return Result.Reject; }
  if (part.a > 3567 ){ return Result.Reject; }
  if (part.x < 2974 ){ return Result.Accept; }
return Result.Accept;

}


fn gqz(part: Part) Result {
  if (part.x > 1101 ){ return Result.Reject; }
  if (part.m > 3173 ){ return Result.Reject; }
  if (part.a < 3472 ){ return Result.Accept; }
return Result.Accept;

}


fn gdq(part: Part) Result {
  if (part.s > 1442 ){ return Result.Reject; }
  if (part.x < 2555 ){ return Result.Reject; }
return Result.Reject;

}


fn kdx(part: Part) Result {
  if (part.a < 2790 ){ return Result.Accept; }
  if (part.s < 869 ){ return Result.Reject; }
return Result.Accept;

}


fn mz(part: Part) Result {
  if (part.m < 2010 ){ return Result.Accept; }
  if (part.m < 2695 ){ return Result.Accept; }
return Result.Accept;

}


fn qcb(part: Part) Result {
  if (part.x < 2654 ){ return Result.Reject; }
  if (part.m < 314 ){ return Result.Reject; }
return Result.Reject;

}


fn pm(part: Part) Result {
  if (part.a > 69 ){ return Result.Reject; }
  if (part.a < 41 ){ return Result.Reject; }
  if (part.m < 3275 ){ return Result.Accept; }
return Result.Reject;

}


fn zbz(part: Part) Result {
  if (part.s < 1657 ){ return lnt(part); }
  if (part.a > 3350 ){ return Result.Accept; }
  if (part.a > 3329 ){ return lp(part); }
{ return gkc(part); }

}


fn chl(part: Part) Result {
  if (part.a < 3397 ){ return zbz(part); }
  if (part.m < 774 ){ return zbs(part); }
{ return zhm(part); }

}


fn gz(part: Part) Result {
  if (part.x > 3482 ){ return Result.Reject; }
  if (part.x > 3220 ){ return Result.Reject; }
  if (part.x > 3163 ){ return Result.Reject; }
return Result.Reject;

}


fn gt(part: Part) Result {
  if (part.a < 3431 ){ return Result.Accept; }
  if (part.a < 3449 ){ return Result.Accept; }
  if (part.a > 3460 ){ return Result.Accept; }
return Result.Reject;

}


fn cz(part: Part) Result {
  if (part.a > 258 ){ return Result.Accept; }
return Result.Accept;

}


fn js(part: Part) Result {
  if (part.a < 3502 ){ return ml(part); }
{ return kr(part); }

}


fn pd(part: Part) Result {
  if (part.a > 942 ){ return Result.Accept; }
return Result.Accept;

}


fn qhv(part: Part) Result {
  if (part.a < 3605 ){ return xpp(part); }
  if (part.a < 3843 ){ return rxt(part); }
  if (part.a < 3938 ){ return npv(part); }
return Result.Accept;

}


fn qnn(part: Part) Result {
  if (part.s < 2954 ){ return Result.Accept; }
return Result.Reject;

}


fn fx(part: Part) Result {
  if (part.x > 2949 ){ return Result.Accept; }
  if (part.m > 3131 ){ return Result.Reject; }
return Result.Reject;

}


fn htx(part: Part) Result {
  if (part.a < 505 ){ return dl(part); }
  if (part.s > 987 ){ return fm(part); }
  if (part.x > 3418 ){ return rj(part); }
{ return hz(part); }

}


fn npv(part: Part) Result {
  if (part.a > 3889 ){ return Result.Reject; }
return Result.Reject;

}


fn dkc(part: Part) Result {
  if (part.a > 1047 ){ return pks(part); }
return Result.Accept;

}


fn xn(part: Part) Result {
  if (part.x > 700 ){ return Result.Reject; }
  if (part.m < 2927 ){ return Result.Reject; }
  if (part.s < 3017 ){ return Result.Accept; }
return Result.Accept;

}


fn dp(part: Part) Result {
  if (part.a < 2504 ){ return ng(part); }
  if (part.x > 1978 ){ return ftv(part); }
  if (part.s > 2201 ){ return fjr(part); }
{ return ss(part); }

}


fn lgz(part: Part) Result {
  if (part.x < 1512 ){ return Result.Accept; }
  if (part.a > 1867 ){ return Result.Accept; }
  if (part.m < 1805 ){ return Result.Reject; }
return Result.Accept;

}


fn nsh(part: Part) Result {
  if (part.m > 3148 ){ return dcj(part); }
{ return bp(part); }

}


fn rp(part: Part) Result {
  if (part.a > 2563 ){ return Result.Accept; }
  if (part.s > 2602 ){ return Result.Accept; }
  if (part.s < 1080 ){ return Result.Reject; }
return Result.Reject;

}


fn nf(part: Part) Result {
  if (part.m > 1513 ){ return Result.Accept; }
  if (part.s > 461 ){ return Result.Accept; }
  if (part.m > 887 ){ return nc(part); }
{ return brq(part); }

}


fn ph(part: Part) Result {
  if (part.x > 774 ){ return Result.Reject; }
  if (part.s > 2895 ){ return Result.Accept; }
  if (part.s > 2739 ){ return Result.Accept; }
return Result.Reject;

}


fn mh(part: Part) Result {
  if (part.m < 1145 ){ return sj(part); }
  if (part.m < 1701 ){ return vsr(part); }
  if (part.s > 3347 ){ return xcq(part); }
{ return rsn(part); }

}


fn xl(part: Part) Result {
  if (part.s > 1517 ){ return Result.Accept; }
  if (part.a < 3094 ){ return Result.Accept; }
return Result.Reject;

}


fn ppj(part: Part) Result {
  if (part.s < 3652 ){ return Result.Reject; }
  if (part.s < 3817 ){ return Result.Reject; }
  if (part.m > 2022 ){ return Result.Reject; }
return Result.Reject;

}


fn txl(part: Part) Result {
  if (part.a > 2915 ){ return Result.Reject; }
  if (part.x > 529 ){ return Result.Accept; }
  if (part.a < 2678 ){ return xx(part); }
{ return lsl(part); }

}


fn pnk(part: Part) Result {
  if (part.a > 3367 ){ return hj(part); }
  if (part.m < 688 ){ return khg(part); }
{ return fpm(part); }

}


fn pv(part: Part) Result {
  if (part.x < 1358 ){ return Result.Accept; }
  if (part.x < 1594 ){ return Result.Reject; }
{ return xkg(part); }

}


fn zg(part: Part) Result {
  if (part.s > 3710 ){ return Result.Accept; }
return Result.Accept;

}


fn hg(part: Part) Result {
  if (part.m < 539 ){ return nsv(part); }
  if (part.a < 431 ){ return jx(part); }
  if (part.s < 3147 ){ return hl(part); }
{ return ct(part); }

}


fn pg(part: Part) Result {
  if (part.s > 3013 ){ return ztm(part); }
  if (part.x > 2614 ){ return qzd(part); }
{ return csx(part); }

}


fn jx(part: Part) Result {
  if (part.m < 1090 ){ return sm(part); }
  if (part.m < 1301 ){ return sqf(part); }
  if (part.m < 1351 ){ return trf(part); }
{ return tmh(part); }

}


fn jhc(part: Part) Result {
  if (part.s > 1197 ){ return Result.Reject; }
  if (part.s < 789 ){ return Result.Accept; }
return Result.Reject;

}


fn qhj(part: Part) Result {
  if (part.x > 1201 ){ return mz(part); }
  if (part.a > 2291 ){ return Result.Reject; }
  if (part.x > 1086 ){ return Result.Reject; }
{ return jgn(part); }

}


fn zgd(part: Part) Result {
  if (part.s > 3475 ){ return Result.Reject; }
return Result.Accept;

}


fn vz(part: Part) Result {
  if (part.x < 219 ){ return Result.Reject; }
  if (part.a < 3966 ){ return Result.Reject; }
  if (part.s < 3695 ){ return Result.Accept; }
return Result.Accept;

}


fn bd(part: Part) Result {
  if (part.x > 1506 ){ return qrv(part); }
  if (part.a > 3470 ){ return gnz(part); }
  if (part.x > 664 ){ return pnk(part); }
{ return chl(part); }

}


fn dtd(part: Part) Result {
  if (part.a > 1301 ){ return Result.Accept; }
return Result.Reject;

}


fn rr(part: Part) Result {
  if (part.s < 1922 ){ return Result.Accept; }
return Result.Reject;

}


fn tfl(part: Part) Result {
  if (part.s < 2579 ){ return ckg(part); }
  if (part.m > 2087 ){ return mm(part); }
  if (part.a > 1185 ){ return xp(part); }
{ return mhq(part); }

}


fn srp(part: Part) Result {
  if (part.x < 933 ){ return hx(part); }
  if (part.s > 539 ){ return pdl(part); }
{ return qhj(part); }

}


fn ndv(part: Part) Result {
  if (part.x > 2832 ){ return kfk(part); }
  if (part.s < 3023 ){ return jr(part); }
  if (part.s > 3177 ){ return Result.Accept; }
return Result.Accept;

}


fn sh(part: Part) Result {
  if (part.m < 1728 ){ return Result.Accept; }
  if (part.s > 159 ){ return Result.Accept; }
  if (part.m > 2111 ){ return Result.Accept; }
return Result.Reject;

}


fn hcj(part: Part) Result {
  if (part.m > 1483 ){ return Result.Accept; }
  if (part.x > 3225 ){ return Result.Accept; }
return Result.Accept;

}


fn dgt(part: Part) Result {
  if (part.m > 2124 ){ return Result.Reject; }
return Result.Accept;

}


fn gjc(part: Part) Result {
  if (part.m > 175 ){ return Result.Reject; }
  if (part.m > 159 ){ return Result.Reject; }
  if (part.a > 269 ){ return Result.Accept; }
return Result.Accept;

}


fn qk(part: Part) Result {
  if (part.m < 1145 ){ return Result.Reject; }
  if (part.s < 2588 ){ return Result.Reject; }
return Result.Reject;

}


fn bt(part: Part) Result {
  if (part.a > 2101 ){ return Result.Accept; }
  if (part.s < 638 ){ return gz(part); }
{ return vqz(part); }

}


fn mkm(part: Part) Result {
  if (part.a > 857 ){ return Result.Reject; }
return Result.Accept;

}


fn zj(part: Part) Result {
  if (part.x > 416 ){ return Result.Accept; }
return Result.Accept;

}


fn qkk(part: Part) Result {
  if (part.m > 3182 ){ return Result.Accept; }
  if (part.s > 2641 ){ return Result.Accept; }
return Result.Accept;

}


fn vqz(part: Part) Result {
  if (part.s < 932 ){ return Result.Reject; }
return Result.Reject;

}


fn cnq(part: Part) Result {
  if (part.s < 3619 ){ return Result.Reject; }
  if (part.a < 3853 ){ return Result.Reject; }
return Result.Accept;

}


fn kzx(part: Part) Result {
  if (part.m > 780 ){ return Result.Reject; }
  if (part.m < 668 ){ return Result.Accept; }
return Result.Reject;

}


fn nsv(part: Part) Result {
  if (part.x < 2006 ){ return jrx(part); }
  if (part.m > 218 ){ return kc(part); }
{ return sdp(part); }

}


fn hz(part: Part) Result {
  if (part.s < 472 ){ return Result.Accept; }
  if (part.m < 1542 ){ return Result.Accept; }
  if (part.x > 3169 ){ return Result.Accept; }
return Result.Reject;

}


fn hrg(part: Part) Result {
  if (part.a < 2648 ){ return Result.Reject; }
return Result.Accept;

}


fn jjm(part: Part) Result {
  if (part.x > 3459 ){ return Result.Reject; }
  if (part.m < 1335 ){ return Result.Reject; }
  if (part.a > 3253 ){ return Result.Reject; }
return Result.Accept;

}


fn xzk(part: Part) Result {
  if (part.a > 3768 ){ return Result.Reject; }
return Result.Accept;

}


fn zhm(part: Part) Result {
  if (part.x < 428 ){ return Result.Reject; }
  if (part.a > 3440 ){ return km(part); }
  if (part.a > 3417 ){ return Result.Accept; }
{ return gtg(part); }

}


fn bq(part: Part) Result {
  if (part.m > 1929 ){ return Result.Reject; }
return Result.Accept;

}


fn ctk(part: Part) Result {
  if (part.a < 2613 ){ return Result.Accept; }
  if (part.x < 1420 ){ return Result.Accept; }
return Result.Accept;

}


fn ckg(part: Part) Result {
  if (part.s < 2437 ){ return Result.Reject; }
  if (part.s < 2496 ){ return Result.Reject; }
return Result.Reject;

}


fn csd(part: Part) Result {
  if (part.m > 2858 ){ return Result.Accept; }
  if (part.x > 3097 ){ return dpq(part); }
  if (part.a > 2777 ){ return sx(part); }
return Result.Reject;

}


fn krr(part: Part) Result {
  if (part.m < 558 ){ return Result.Accept; }
  if (part.s < 689 ){ return Result.Reject; }
  if (part.s > 962 ){ return szq(part); }
return Result.Reject;

}


fn xd(part: Part) Result {
  if (part.x < 494 ){ return Result.Reject; }
{ return hs(part); }

}


fn thh(part: Part) Result {
  if (part.a < 3831 ){ return Result.Reject; }
  if (part.x > 590 ){ return Result.Reject; }
  if (part.m > 1134 ){ return Result.Accept; }
return Result.Reject;

}


fn bm(part: Part) Result {
  if (part.x > 1575 ){ return tvk(part); }
  if (part.x > 884 ){ return xh(part); }
{ return pk(part); }

}


fn ffk(part: Part) Result {
  if (part.s > 3601 ){ return ccl(part); }
  if (part.x > 1104 ){ return vd(part); }
return Result.Accept;

}


fn nr(part: Part) Result {
  if (part.m < 1540 ){ return Result.Reject; }
  if (part.a > 885 ){ return Result.Reject; }
return Result.Reject;

}


fn brq(part: Part) Result {
  if (part.s < 305 ){ return Result.Accept; }
  if (part.x < 1475 ){ return Result.Reject; }
return Result.Accept;

}


fn rkk(part: Part) Result {
  if (part.x > 1356 ){ return Result.Accept; }
  if (part.a > 2646 ){ return Result.Accept; }
  if (part.m < 1940 ){ return Result.Reject; }
return Result.Accept;

}


fn vnq(part: Part) Result {
  if (part.x > 2914 ){ return htx(part); }
{ return sr(part); }

}


fn bs(part: Part) Result {
  if (part.m > 3156 ){ return Result.Accept; }
  if (part.a > 1197 ){ return Result.Reject; }
return Result.Reject;

}


fn cxn(part: Part) Result {
  if (part.m < 2622 ){ return pmc(part); }
{ return nsh(part); }

}


fn vpq(part: Part) Result {
  if (part.s < 3245 ){ return Result.Accept; }
return Result.Reject;

}


fn mq(part: Part) Result {
  if (part.a > 148 ){ return Result.Reject; }
  if (part.m > 2248 ){ return pm(part); }
return Result.Reject;

}


fn mqq(part: Part) Result {
  if (part.s > 3294 ){ return ffk(part); }
{ return ts(part); }

}


fn ksl(part: Part) Result {
  if (part.m > 859 ){ return Result.Reject; }
return Result.Accept;

}


fn jv(part: Part) Result {
  if (part.x < 3441 ){ return Result.Reject; }
return Result.Accept;

}


fn tbv(part: Part) Result {
  if (part.x > 1532 ){ return Result.Reject; }
  if (part.m > 1280 ){ return Result.Reject; }
  if (part.a < 2392 ){ return Result.Reject; }
return Result.Accept;

}


fn fz(part: Part) Result {
  if (part.a < 3784 ){ return Result.Reject; }
  if (part.s < 1780 ){ return Result.Accept; }
return Result.Reject;

}


fn dgm(part: Part) Result {
  if (part.a > 2823 ){ return Result.Reject; }
  if (part.m < 2459 ){ return jtb(part); }
  if (part.x > 722 ){ return ctk(part); }
{ return rmd(part); }

}


fn pmn(part: Part) Result {
  if (part.a < 3406 ){ return ps(part); }
  if (part.a > 3477 ){ return qz(part); }
  if (part.a > 3446 ){ return mqq(part); }
{ return tpr(part); }

}


fn hzj(part: Part) Result {
  if (part.m > 1470 ){ return hn(part); }
{ return ck(part); }

}


fn ftv(part: Part) Result {
  if (part.m < 2191 ){ return jlf(part); }
  if (part.a < 2851 ){ return xqx(part); }
{ return lv(part); }

}


fn snj(part: Part) Result {
  if (part.a < 3828 ){ return Result.Reject; }
  if (part.a > 3932 ){ return Result.Reject; }
  if (part.m < 1119 ){ return Result.Reject; }
{ return qpb(part); }

}


fn vv(part: Part) Result {
  if (part.a > 3516 ){ return Result.Reject; }
  if (part.x < 2342 ){ return Result.Accept; }
  if (part.x > 3068 ){ return Result.Accept; }
return Result.Accept;

}


fn vr(part: Part) Result {
  if (part.x > 2831 ){ return Result.Reject; }
  if (part.x < 2429 ){ return Result.Accept; }
  if (part.m < 2558 ){ return Result.Accept; }
return Result.Reject;

}


fn kf(part: Part) Result {
  if (part.a > 150 ){ return Result.Reject; }
  if (part.a < 86 ){ return Result.Accept; }
return Result.Reject;

}


fn qqk(part: Part) Result {
  if (part.m > 897 ){ return Result.Accept; }
  if (part.x > 1230 ){ return Result.Reject; }
  if (part.s > 748 ){ return Result.Accept; }
return Result.Reject;

}


fn df(part: Part) Result {
  if (part.m > 2252 ){ return rnd(part); }
  if (part.x > 283 ){ return Result.Reject; }
  if (part.a < 3152 ){ return Result.Accept; }
return Result.Reject;

}


fn tb(part: Part) Result {
  if (part.a < 3117 ){ return Result.Accept; }
  if (part.x > 237 ){ return Result.Accept; }
  if (part.s > 1987 ){ return Result.Reject; }
return Result.Reject;

}


fn rbj(part: Part) Result {
  if (part.s < 438 ){ return sh(part); }
  if (part.a < 1236 ){ return qqk(part); }
{ return xs(part); }

}


fn mf(part: Part) Result {
  if (part.m > 2334 ){ return Result.Accept; }
  if (part.a > 3133 ){ return Result.Reject; }
return Result.Reject;

}


fn sln(part: Part) Result {
  if (part.m < 1070 ){ return Result.Accept; }
  if (part.a < 3383 ){ return Result.Reject; }
  if (part.s < 2877 ){ return Result.Reject; }
return Result.Accept;

}


fn tk(part: Part) Result {
  if (part.x < 797 ){ return tng(part); }
  if (part.a < 3422 ){ return Result.Reject; }
  if (part.a < 3433 ){ return Result.Reject; }
return Result.Accept;

}


fn tqn(part: Part) Result {
  if (part.a < 2753 ){ return bq(part); }
return Result.Reject;

}


fn gkc(part: Part) Result {
  if (part.m < 980 ){ return Result.Reject; }
return Result.Accept;

}


fn rft(part: Part) Result {
  if (part.x < 1179 ){ return Result.Accept; }
return Result.Reject;

}


fn nn(part: Part) Result {
  if (part.m < 1586 ){ return Result.Accept; }
  if (part.a < 2076 ){ return Result.Accept; }
  if (part.m < 2065 ){ return Result.Reject; }
return Result.Reject;

}


fn rqk(part: Part) Result {
  if (part.s > 570 ){ return Result.Accept; }
  if (part.s < 277 ){ return fnv(part); }
{ return cf(part); }

}


fn nc(part: Part) Result {
  if (part.x < 1488 ){ return Result.Accept; }
  if (part.m < 1200 ){ return Result.Accept; }
return Result.Reject;

}


fn ndj(part: Part) Result {
  if (part.a < 1037 ){ return hkz(part); }
  if (part.a > 1126 ){ return pvf(part); }
{ return thp(part); }

}


fn fgs(part: Part) Result {
  if (part.s < 723 ){ return jdn(part); }
  if (part.a < 1225 ){ return cm(part); }
{ return vgm(part); }

}


fn hnc(part: Part) Result {
  if (part.m > 1825 ){ return Result.Reject; }
  if (part.s > 2872 ){ return Result.Accept; }
{ return qnl(part); }

}


fn rsn(part: Part) Result {
  if (part.a < 3610 ){ return ndv(part); }
{ return cp(part); }

}


fn szq(part: Part) Result {
  if (part.a < 658 ){ return Result.Accept; }
  if (part.x < 1082 ){ return Result.Accept; }
  if (part.x < 1136 ){ return Result.Reject; }
return Result.Reject;

}


fn pks(part: Part) Result {
  if (part.s < 3647 ){ return Result.Reject; }
  if (part.x < 2072 ){ return Result.Reject; }
  if (part.m < 1014 ){ return Result.Reject; }
return Result.Accept;

}


fn krn(part: Part) Result {
  if (part.s > 3752 ){ return Result.Accept; }
return Result.Accept;

}


fn slx(part: Part) Result {
  if (part.a < 1578 ){ return Result.Accept; }
  if (part.m > 2547 ){ return Result.Reject; }
return Result.Accept;

}


fn jf(part: Part) Result {
  if (part.a > 1594 ){ return Result.Reject; }
return Result.Accept;

}


fn xz(part: Part) Result {
  if (part.s < 2031 ){ return Result.Accept; }
  if (part.s < 3118 ){ return Result.Reject; }
return Result.Reject;

}


fn jjl(part: Part) Result {
  if (part.x < 1605 ){ return Result.Accept; }
  if (part.x > 3094 ){ return Result.Reject; }
  if (part.m < 3286 ){ return Result.Accept; }
return Result.Reject;

}


fn sqf(part: Part) Result {
  if (part.s > 3365 ){ return kf(part); }
  if (part.m > 1213 ){ return bjd(part); }
return Result.Accept;

}


fn cvt(part: Part) Result {
  if (part.m > 1450 ){ return Result.Reject; }
  if (part.a < 3534 ){ return Result.Accept; }
  if (part.m < 1256 ){ return Result.Accept; }
return Result.Accept;

}


fn gtg(part: Part) Result {
  if (part.s > 1303 ){ return Result.Reject; }
  if (part.x < 583 ){ return Result.Reject; }
return Result.Reject;

}


fn rj(part: Part) Result {
  if (part.s < 546 ){ return Result.Accept; }
return Result.Reject;

}


fn xbt(part: Part) Result {
  if (part.a > 1482 ){ return Result.Reject; }
  if (part.x < 2601 ){ return Result.Accept; }
return Result.Reject;

}


fn tgz(part: Part) Result {
  if (part.x > 1352 ){ return Result.Accept; }
  if (part.m > 3071 ){ return Result.Reject; }
return Result.Reject;

}


fn xt(part: Part) Result {
  if (part.m < 2085 ){ return Result.Reject; }
  if (part.m < 3250 ){ return Result.Accept; }
  if (part.s < 3518 ){ return Result.Accept; }
return Result.Reject;

}


fn vgm(part: Part) Result {
  if (part.a > 1704 ){ return Result.Accept; }
  if (part.a > 1468 ){ return Result.Accept; }
{ return lgs(part); }

}


fn gdx(part: Part) Result {
  if (part.x > 546 ){ return Result.Accept; }
  if (part.s > 2668 ){ return Result.Accept; }
return Result.Accept;

}


fn lp(part: Part) Result {
  if (part.a > 3340 ){ return Result.Accept; }
  if (part.a < 3335 ){ return Result.Accept; }
return Result.Accept;

}


fn jzn(part: Part) Result {
  if (part.a < 3782 ){ return sz(part); }
  if (part.a > 3900 ){ return zhq(part); }
  if (part.x > 1239 ){ return zq(part); }
{ return rvq(part); }

}


fn gl(part: Part) Result {
  if (part.m > 1324 ){ return Result.Reject; }
return Result.Accept;

}


fn sp(part: Part) Result {
  if (part.a > 3654 ){ return gp(part); }
  if (part.s > 3290 ){ return kxh(part); }
  if (part.x > 502 ){ return xn(part); }
{ return kv(part); }

}


fn rpz(part: Part) Result {
  if (part.m < 2858 ){ return xg(part); }
  if (part.x > 2804 ){ return bb(part); }
{ return lg(part); }

}


fn mc(part: Part) Result {
  if (part.a > 1304 ){ return lvm(part); }
  if (part.a > 784 ){ return pg(part); }
  if (part.m > 1444 ){ return cxn(part); }
{ return hg(part); }

}


fn ljf(part: Part) Result {
  if (part.x < 1600 ){ return Result.Accept; }
return Result.Accept;

}


fn zsz(part: Part) Result {
  if (part.s < 876 ){ return Result.Reject; }
  if (part.x < 406 ){ return Result.Reject; }
return Result.Reject;

}


fn xv(part: Part) Result {
  if (part.s < 1540 ){ return fpn(part); }
return Result.Reject;

}


fn thp(part: Part) Result {
  if (part.s < 3511 ){ return Result.Accept; }
  if (part.m > 610 ){ return Result.Accept; }
return Result.Reject;

}


fn qrv(part: Part) Result {
  if (part.m > 561 ){ return cmn(part); }
{ return ctx(part); }

}


fn fh(part: Part) Result {
  if (part.x > 1582 ){ return cr(part); }
  if (part.m < 1449 ){ return tg(part); }
{ return rkk(part); }

}


fn xnm(part: Part) Result {
  if (part.m > 860 ){ return Result.Reject; }
  if (part.m > 471 ){ return Result.Accept; }
  if (part.x > 3801 ){ return Result.Accept; }
{ return zzs(part); }

}


fn mb(part: Part) Result {
  if (part.s > 3818 ){ return Result.Reject; }
  if (part.m > 883 ){ return Result.Reject; }
return Result.Reject;

}


fn glh(part: Part) Result {
  if (part.x > 126 ){ return Result.Reject; }
  if (part.s < 1318 ){ return Result.Accept; }
return Result.Accept;

}


fn txc(part: Part) Result {
  if (part.x < 563 ){ return ttr(part); }
  if (part.a > 2924 ){ return rf(part); }
  if (part.s > 1546 ){ return tt(part); }
{ return td(part); }

}


fn hrm(part: Part) Result {
  if (part.x < 645 ){ return Result.Reject; }
  if (part.a < 3607 ){ return Result.Accept; }
return Result.Reject;

}


fn hkx(part: Part) Result {
  if (part.m < 3112 ){ return Result.Reject; }
  if (part.x < 1316 ){ return Result.Accept; }
  if (part.m < 3464 ){ return Result.Accept; }
return Result.Accept;

}


fn zv(part: Part) Result {
  if (part.a < 1194 ){ return qcc(part); }
  if (part.x < 3761 ){ return Result.Accept; }
  if (part.m < 2135 ){ return qk(part); }
return Result.Reject;

}


fn ht(part: Part) Result {
  if (part.s < 1496 ){ return Result.Reject; }
  if (part.a < 2659 ){ return Result.Reject; }
  if (part.x > 693 ){ return Result.Accept; }
return Result.Accept;

}


fn kcs(part: Part) Result {
  if (part.m < 2473 ){ return Result.Accept; }
  if (part.s < 669 ){ return Result.Reject; }
return Result.Accept;

}


fn xqx(part: Part) Result {
  if (part.a > 2697 ){ return csd(part); }
  if (part.a < 2620 ){ return tp(part); }
{ return nds(part); }

}


fn blf(part: Part) Result {
  if (part.s > 1904 ){ return Result.Reject; }
  if (part.x > 2898 ){ return Result.Reject; }
  if (part.s < 1014 ){ return Result.Reject; }
return Result.Accept;

}


fn qsv(part: Part) Result {
  if (part.m > 3905 ){ return Result.Reject; }
return Result.Accept;

}


fn hlp(part: Part) Result {
  if (part.m < 1671 ){ return Result.Reject; }
  if (part.m > 2278 ){ return Result.Reject; }
return Result.Accept;

}


fn nds(part: Part) Result {
  if (part.x < 2838 ){ return xz(part); }
  if (part.x > 3332 ){ return hrg(part); }
  if (part.m > 3252 ){ return Result.Reject; }
{ return gc(part); }

}


fn xjc(part: Part) Result {
  if (part.x > 3254 ){ return Result.Reject; }
  if (part.a > 3537 ){ return Result.Accept; }
  if (part.s > 1282 ){ return Result.Reject; }
return Result.Reject;

}


fn nq(part: Part) Result {
  if (part.a < 2889 ){ return Result.Reject; }
return Result.Accept;

}


fn tt(part: Part) Result {
  if (part.x > 687 ){ return zjm(part); }
{ return grm(part); }

}


fn xcq(part: Part) Result {
  if (part.x < 2451 ){ return zm(part); }
{ return rlk(part); }

}


fn jj(part: Part) Result {
  if (part.s > 3409 ){ return mp(part); }
  if (part.m > 3099 ){ return Result.Accept; }
  if (part.a > 3811 ){ return Result.Accept; }
{ return qhp(part); }

}


fn lvl(part: Part) Result {
  if (part.a > 3838 ){ return Result.Reject; }
  if (part.x < 1075 ){ return Result.Reject; }
return Result.Accept;

}


fn xpg(part: Part) Result {
  if (part.x < 2284 ){ return Result.Reject; }
  if (part.s < 3556 ){ return Result.Reject; }
  if (part.a > 179 ){ return Result.Reject; }
return Result.Accept;

}


fn mgd(part: Part) Result {
  if (part.a < 135 ){ return Result.Reject; }
  if (part.x < 1914 ){ return Result.Reject; }
return Result.Reject;

}


fn mvj(part: Part) Result {
  if (part.x > 616 ){ return Result.Reject; }
  if (part.s < 1708 ){ return Result.Accept; }
  if (part.m > 1421 ){ return Result.Reject; }
return Result.Accept;

}


fn vn(part: Part) Result {
  if (part.a < 2844 ){ return tm(part); }
  if (part.m > 2427 ){ return vpq(part); }
  if (part.x > 1273 ){ return cjk(part); }
return Result.Accept;

}


fn qt(part: Part) Result {
  if (part.s < 1416 ){ return glh(part); }
  if (part.a > 3147 ){ return gk(part); }
  if (part.m < 1773 ){ return Result.Accept; }
{ return xl(part); }

}


fn qf(part: Part) Result {
  if (part.s < 492 ){ return Result.Reject; }
  if (part.m > 874 ){ return Result.Reject; }
return Result.Reject;

}


fn cc(part: Part) Result {
  if (part.a < 1033 ){ return Result.Reject; }
  if (part.a < 1079 ){ return Result.Reject; }
return Result.Accept;

}
 pub fn main() !void {
 var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
 var allocator = arena.allocator();
 var parts = std.ArrayList(Part).init(allocator);
try parts.append(Part {.x=5,.m=2118,.a=201,.s=321,});
try parts.append(Part {.x=321,.m=76,.a=1226,.s=2871,});
try parts.append(Part {.x=164,.m=1611,.a=596,.s=1494,});
try parts.append(Part {.x=3007,.m=811,.a=95,.s=2830,});
try parts.append(Part {.x=92,.m=902,.a=984,.s=1868,});
try parts.append(Part {.x=1042,.m=1973,.a=351,.s=288,});
try parts.append(Part {.x=1363,.m=1227,.a=26,.s=196,});
try parts.append(Part {.x=2917,.m=2066,.a=30,.s=1576,});
try parts.append(Part {.x=937,.m=113,.a=1007,.s=1285,});
try parts.append(Part {.x=1505,.m=1125,.a=4,.s=830,});
try parts.append(Part {.x=128,.m=1152,.a=386,.s=250,});
try parts.append(Part {.x=180,.m=688,.a=2793,.s=821,});
try parts.append(Part {.x=607,.m=1381,.a=289,.s=1681,});
try parts.append(Part {.x=77,.m=1612,.a=228,.s=1905,});
try parts.append(Part {.x=826,.m=296,.a=20,.s=24,});
try parts.append(Part {.x=477,.m=2231,.a=2286,.s=2701,});
try parts.append(Part {.x=2208,.m=1967,.a=2428,.s=3497,});
try parts.append(Part {.x=409,.m=111,.a=1246,.s=1230,});
try parts.append(Part {.x=934,.m=879,.a=829,.s=128,});
try parts.append(Part {.x=405,.m=1474,.a=1235,.s=26,});
try parts.append(Part {.x=3682,.m=906,.a=156,.s=2226,});
try parts.append(Part {.x=520,.m=888,.a=1881,.s=331,});
try parts.append(Part {.x=2324,.m=1510,.a=996,.s=123,});
try parts.append(Part {.x=57,.m=1073,.a=101,.s=136,});
try parts.append(Part {.x=5,.m=234,.a=154,.s=609,});
try parts.append(Part {.x=3349,.m=2414,.a=397,.s=780,});
try parts.append(Part {.x=1405,.m=63,.a=1861,.s=398,});
try parts.append(Part {.x=42,.m=1938,.a=1380,.s=601,});
try parts.append(Part {.x=87,.m=1540,.a=62,.s=784,});
try parts.append(Part {.x=1690,.m=202,.a=1916,.s=1497,});
try parts.append(Part {.x=1798,.m=77,.a=31,.s=854,});
try parts.append(Part {.x=424,.m=6,.a=1831,.s=509,});
try parts.append(Part {.x=1549,.m=65,.a=835,.s=810,});
try parts.append(Part {.x=1799,.m=1266,.a=116,.s=610,});
try parts.append(Part {.x=346,.m=160,.a=206,.s=616,});
try parts.append(Part {.x=446,.m=289,.a=550,.s=2183,});
try parts.append(Part {.x=78,.m=53,.a=786,.s=456,});
try parts.append(Part {.x=321,.m=2769,.a=1034,.s=1189,});
try parts.append(Part {.x=318,.m=772,.a=859,.s=1610,});
try parts.append(Part {.x=609,.m=1131,.a=2213,.s=2060,});
try parts.append(Part {.x=14,.m=693,.a=141,.s=556,});
try parts.append(Part {.x=434,.m=362,.a=56,.s=3333,});
try parts.append(Part {.x=441,.m=2672,.a=1232,.s=3554,});
try parts.append(Part {.x=2052,.m=721,.a=66,.s=2589,});
try parts.append(Part {.x=118,.m=3058,.a=698,.s=14,});
try parts.append(Part {.x=3201,.m=2647,.a=1180,.s=796,});
try parts.append(Part {.x=995,.m=1832,.a=374,.s=2871,});
try parts.append(Part {.x=1988,.m=544,.a=2863,.s=2337,});
try parts.append(Part {.x=242,.m=271,.a=130,.s=3453,});
try parts.append(Part {.x=2170,.m=115,.a=1790,.s=524,});
try parts.append(Part {.x=114,.m=602,.a=1689,.s=223,});
try parts.append(Part {.x=944,.m=164,.a=1861,.s=1399,});
try parts.append(Part {.x=915,.m=249,.a=42,.s=280,});
try parts.append(Part {.x=931,.m=1104,.a=2584,.s=1109,});
try parts.append(Part {.x=362,.m=135,.a=343,.s=2218,});
try parts.append(Part {.x=1588,.m=2648,.a=222,.s=803,});
try parts.append(Part {.x=9,.m=229,.a=245,.s=132,});
try parts.append(Part {.x=419,.m=623,.a=1306,.s=269,});
try parts.append(Part {.x=408,.m=1095,.a=333,.s=1776,});
try parts.append(Part {.x=2671,.m=2385,.a=154,.s=2100,});
try parts.append(Part {.x=2393,.m=444,.a=168,.s=173,});
try parts.append(Part {.x=308,.m=3768,.a=2449,.s=111,});
try parts.append(Part {.x=936,.m=162,.a=2501,.s=263,});
try parts.append(Part {.x=1310,.m=63,.a=1840,.s=3,});
try parts.append(Part {.x=543,.m=118,.a=1648,.s=2254,});
try parts.append(Part {.x=627,.m=975,.a=829,.s=1781,});
try parts.append(Part {.x=326,.m=66,.a=906,.s=1687,});
try parts.append(Part {.x=54,.m=2177,.a=2296,.s=840,});
try parts.append(Part {.x=1191,.m=12,.a=1534,.s=1855,});
try parts.append(Part {.x=1637,.m=67,.a=338,.s=557,});
try parts.append(Part {.x=1170,.m=1757,.a=1260,.s=62,});
try parts.append(Part {.x=379,.m=1550,.a=413,.s=792,});
try parts.append(Part {.x=48,.m=232,.a=654,.s=2021,});
try parts.append(Part {.x=49,.m=1107,.a=575,.s=473,});
try parts.append(Part {.x=435,.m=179,.a=522,.s=398,});
try parts.append(Part {.x=1088,.m=324,.a=1314,.s=1170,});
try parts.append(Part {.x=2111,.m=604,.a=233,.s=2104,});
try parts.append(Part {.x=975,.m=514,.a=1656,.s=2497,});
try parts.append(Part {.x=505,.m=2371,.a=464,.s=1852,});
try parts.append(Part {.x=2737,.m=3240,.a=2930,.s=208,});
try parts.append(Part {.x=448,.m=545,.a=2013,.s=1723,});
try parts.append(Part {.x=433,.m=1715,.a=2258,.s=428,});
try parts.append(Part {.x=26,.m=936,.a=772,.s=154,});
try parts.append(Part {.x=567,.m=1171,.a=3128,.s=1885,});
try parts.append(Part {.x=66,.m=1459,.a=161,.s=264,});
try parts.append(Part {.x=140,.m=51,.a=2669,.s=1308,});
try parts.append(Part {.x=122,.m=1719,.a=538,.s=2677,});
try parts.append(Part {.x=94,.m=295,.a=160,.s=14,});
try parts.append(Part {.x=260,.m=343,.a=2222,.s=284,});
try parts.append(Part {.x=1028,.m=1924,.a=2256,.s=1355,});
try parts.append(Part {.x=520,.m=144,.a=2535,.s=295,});
try parts.append(Part {.x=1896,.m=632,.a=2268,.s=1871,});
try parts.append(Part {.x=997,.m=743,.a=1614,.s=62,});
try parts.append(Part {.x=2107,.m=595,.a=704,.s=677,});
try parts.append(Part {.x=897,.m=1203,.a=103,.s=229,});
try parts.append(Part {.x=35,.m=3297,.a=907,.s=110,});
try parts.append(Part {.x=2228,.m=1416,.a=189,.s=303,});
try parts.append(Part {.x=1616,.m=1902,.a=132,.s=3325,});
try parts.append(Part {.x=940,.m=1188,.a=1240,.s=352,});
try parts.append(Part {.x=771,.m=1691,.a=3374,.s=124,});
try parts.append(Part {.x=270,.m=616,.a=431,.s=252,});
try parts.append(Part {.x=74,.m=2778,.a=1795,.s=1103,});
try parts.append(Part {.x=1000,.m=14,.a=174,.s=888,});
try parts.append(Part {.x=1166,.m=314,.a=83,.s=244,});
try parts.append(Part {.x=2197,.m=33,.a=535,.s=650,});
try parts.append(Part {.x=216,.m=642,.a=317,.s=145,});
try parts.append(Part {.x=9,.m=87,.a=3379,.s=12,});
try parts.append(Part {.x=867,.m=1388,.a=608,.s=356,});
try parts.append(Part {.x=295,.m=53,.a=260,.s=90,});
try parts.append(Part {.x=250,.m=658,.a=509,.s=3474,});
try parts.append(Part {.x=47,.m=2804,.a=1133,.s=1418,});
try parts.append(Part {.x=2,.m=618,.a=613,.s=1353,});
try parts.append(Part {.x=130,.m=550,.a=290,.s=650,});
try parts.append(Part {.x=2223,.m=126,.a=874,.s=2509,});
try parts.append(Part {.x=1446,.m=143,.a=654,.s=1724,});
try parts.append(Part {.x=1428,.m=145,.a=62,.s=602,});
try parts.append(Part {.x=1011,.m=198,.a=2858,.s=3174,});
try parts.append(Part {.x=1124,.m=1919,.a=2093,.s=1390,});
try parts.append(Part {.x=374,.m=1515,.a=348,.s=1604,});
try parts.append(Part {.x=1538,.m=1634,.a=1084,.s=1500,});
try parts.append(Part {.x=994,.m=883,.a=2485,.s=146,});
try parts.append(Part {.x=764,.m=961,.a=470,.s=247,});
try parts.append(Part {.x=852,.m=74,.a=1651,.s=842,});
try parts.append(Part {.x=57,.m=2045,.a=827,.s=796,});
try parts.append(Part {.x=583,.m=896,.a=164,.s=2476,});
try parts.append(Part {.x=335,.m=213,.a=1232,.s=86,});
try parts.append(Part {.x=283,.m=581,.a=292,.s=769,});
try parts.append(Part {.x=233,.m=1817,.a=2053,.s=232,});
try parts.append(Part {.x=593,.m=833,.a=534,.s=43,});
try parts.append(Part {.x=3529,.m=529,.a=1767,.s=924,});
try parts.append(Part {.x=81,.m=273,.a=50,.s=298,});
try parts.append(Part {.x=145,.m=1805,.a=3553,.s=51,});
try parts.append(Part {.x=2679,.m=1888,.a=754,.s=504,});
try parts.append(Part {.x=2461,.m=860,.a=357,.s=133,});
try parts.append(Part {.x=31,.m=1688,.a=1473,.s=3375,});
try parts.append(Part {.x=354,.m=2261,.a=1669,.s=30,});
try parts.append(Part {.x=329,.m=3020,.a=1684,.s=848,});
try parts.append(Part {.x=1221,.m=196,.a=373,.s=974,});
try parts.append(Part {.x=1592,.m=937,.a=666,.s=30,});
try parts.append(Part {.x=2803,.m=640,.a=2495,.s=1399,});
try parts.append(Part {.x=44,.m=2525,.a=578,.s=328,});
try parts.append(Part {.x=769,.m=596,.a=823,.s=672,});
try parts.append(Part {.x=1064,.m=158,.a=2518,.s=107,});
try parts.append(Part {.x=1713,.m=122,.a=91,.s=2413,});
try parts.append(Part {.x=883,.m=164,.a=385,.s=379,});
try parts.append(Part {.x=259,.m=2014,.a=647,.s=1729,});
try parts.append(Part {.x=42,.m=196,.a=627,.s=1264,});
try parts.append(Part {.x=1414,.m=946,.a=1490,.s=283,});
try parts.append(Part {.x=1557,.m=155,.a=1663,.s=685,});
try parts.append(Part {.x=519,.m=464,.a=320,.s=377,});
try parts.append(Part {.x=66,.m=21,.a=2331,.s=1480,});
try parts.append(Part {.x=1907,.m=1660,.a=1534,.s=1090,});
try parts.append(Part {.x=331,.m=138,.a=218,.s=102,});
try parts.append(Part {.x=16,.m=667,.a=1766,.s=2864,});
try parts.append(Part {.x=1634,.m=359,.a=2500,.s=686,});
try parts.append(Part {.x=2535,.m=38,.a=117,.s=11,});
try parts.append(Part {.x=454,.m=319,.a=2368,.s=107,});
try parts.append(Part {.x=1905,.m=3317,.a=114,.s=37,});
try parts.append(Part {.x=95,.m=224,.a=47,.s=1384,});
try parts.append(Part {.x=2754,.m=716,.a=285,.s=740,});
try parts.append(Part {.x=116,.m=971,.a=187,.s=55,});
try parts.append(Part {.x=249,.m=1481,.a=7,.s=203,});
try parts.append(Part {.x=643,.m=118,.a=23,.s=37,});
try parts.append(Part {.x=497,.m=164,.a=1581,.s=93,});
try parts.append(Part {.x=1777,.m=147,.a=89,.s=425,});
try parts.append(Part {.x=1141,.m=1825,.a=920,.s=414,});
try parts.append(Part {.x=5,.m=2033,.a=31,.s=19,});
try parts.append(Part {.x=1231,.m=430,.a=41,.s=2704,});
try parts.append(Part {.x=1648,.m=232,.a=311,.s=317,});
try parts.append(Part {.x=21,.m=2483,.a=2128,.s=335,});
try parts.append(Part {.x=2972,.m=19,.a=989,.s=1718,});
try parts.append(Part {.x=359,.m=1688,.a=203,.s=68,});
try parts.append(Part {.x=127,.m=352,.a=2675,.s=10,});
try parts.append(Part {.x=655,.m=2323,.a=541,.s=1543,});
try parts.append(Part {.x=113,.m=2137,.a=54,.s=2284,});
try parts.append(Part {.x=136,.m=195,.a=1882,.s=428,});
try parts.append(Part {.x=77,.m=346,.a=2270,.s=221,});
try parts.append(Part {.x=768,.m=822,.a=717,.s=999,});
try parts.append(Part {.x=3247,.m=123,.a=3202,.s=522,});
try parts.append(Part {.x=23,.m=1279,.a=977,.s=1157,});
try parts.append(Part {.x=947,.m=99,.a=110,.s=1272,});
try parts.append(Part {.x=120,.m=641,.a=2692,.s=15,});
try parts.append(Part {.x=1556,.m=972,.a=1378,.s=158,});
try parts.append(Part {.x=3040,.m=457,.a=1261,.s=70,});
try parts.append(Part {.x=768,.m=83,.a=346,.s=191,});
try parts.append(Part {.x=25,.m=714,.a=1284,.s=344,});
try parts.append(Part {.x=828,.m=616,.a=769,.s=433,});
try parts.append(Part {.x=708,.m=8,.a=882,.s=425,});
try parts.append(Part {.x=3591,.m=470,.a=1502,.s=542,});
try parts.append(Part {.x=2025,.m=250,.a=608,.s=882,});
try parts.append(Part {.x=2403,.m=1376,.a=2727,.s=498,});
try parts.append(Part {.x=473,.m=1777,.a=432,.s=1408,});
try parts.append(Part {.x=532,.m=1199,.a=461,.s=281,});
try parts.append(Part {.x=2896,.m=620,.a=445,.s=518,});
try parts.append(Part {.x=2630,.m=900,.a=3201,.s=285,});
try parts.append(Part {.x=1756,.m=1064,.a=158,.s=146,});
try parts.append(Part {.x=2356,.m=1674,.a=6,.s=1561,});
try parts.append(Part {.x=246,.m=1122,.a=3613,.s=847,});
try parts.append(Part {.x=1471,.m=2135,.a=1866,.s=2240,});
try parts.append(Part {.x=4,.m=2292,.a=1876,.s=2328,});
 var total: isize = 0;
 for (parts.items) |part| {
   if (in(part) == Result.Accept) {
     total += part.x;
     total += part.m;
     total += part.a;
     total += part.s;
   }
 }

 std.debug.print("The grand total was: {d}\n", .{total});
}
