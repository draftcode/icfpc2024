use std::rc::Rc;

use anyhow::bail;
use num_bigint::BigInt;

use crate::{
    base94::{decode_base94, decode_char, encode_base94, encode_str},
    expr::{BinOp, Expr, UnOp},
};

#[derive(Default, Clone, Debug)]
struct Stats {
    beta_reductions: usize,
}

pub fn eval(e: &Expr) -> anyhow::Result<Expr> {
    reduce_to_nf(e, &mut Stats::default())
}

fn reduce_to_nf(e: &Expr, stats: &mut Stats) -> anyhow::Result<Expr> {
    log::trace!("eval: {e}");

    Ok(match e {
        Expr::Un(op, e) => {
            let e = reduce_to_nf(e.as_ref(), stats)?;
            match op {
                UnOp::Neg => match e {
                    Expr::Int(n) => Expr::Int(Rc::new(-n.as_ref().clone())),
                    _ => bail!("Invalid operator for neg: {e:?}"),
                },
                UnOp::Not => match e {
                    Expr::Bool(b) => Expr::Bool(!b),
                    _ => bail!("Invalid operator for not: {e:?}"),
                },
                UnOp::StrToInt => match e {
                    Expr::String(s) => Expr::Int(str_to_int(&s).into()),
                    _ => bail!("Invalid operator for str_to_int: {e:?}"),
                },
                UnOp::IntToStr => match e {
                    Expr::Int(n) => Expr::String(int_to_str(&n).into()),
                    _ => bail!("Invalid operator for int_to_str: {e:?}"),
                },
            }
        }
        Expr::Bin(op, l, r) => {
            if matches!(op, BinOp::App) {
                log::trace!("app: {l}, {r}");
                let f = reduce_to_nf(l.as_ref(), stats)?;
                match f {
                    Expr::Lambda(v, e) => {
                        stats.beta_reductions += 1;
                        return reduce_to_nf(
                            &beta_reduction(e.as_ref(), v, r.as_ref(), &mut vec![])?,
                            stats,
                        );
                    }
                    _ => bail!("Invalid operator for app: {f}"),
                }
            }
            if matches!(op, BinOp::AppV) {
                log::trace!("app: {l}, {r}");
                let f = reduce_to_nf(l.as_ref(), stats)?;
                // It's okay to eval the rhs because it's call-by-value.
                let g = reduce_to_nf(r.as_ref(), stats)?;
                match f {
                    Expr::Lambda(v, e) => {
                        stats.beta_reductions += 1;
                        return reduce_to_nf(
                            &beta_reduction(e.as_ref(), v, &g, &mut vec![])?,
                            stats,
                        );
                    }
                    _ => bail!("Invalid operator for appv: {f}"),
                }
            }

            let l = reduce_to_nf(l.as_ref(), stats)?;
            let r = reduce_to_nf(r.as_ref(), stats)?;
            match (op, &l, &r) {
                (BinOp::Add, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() + n2.as_ref()).into()),
                    _ => bail!("Invalid operator for add:\nl = {l}\nr = {r}"),
                },
                (BinOp::Sub, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() - n2.as_ref()).into()),
                    _ => bail!("Invalid operator for sub: {op} {l} {r}"),
                },
                (BinOp::Mul, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() * n2.as_ref()).into()),
                    _ => bail!("Invalid operator for mul: {op} {l} {r}"),
                },
                (BinOp::Div, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() / n2.as_ref()).into()),
                    _ => bail!("Invalid operator for div: {op} {l} {r}"),
                },
                (BinOp::Mod, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Int((n1.as_ref() % n2.as_ref()).into()),
                    _ => bail!("Invalid operator for mod: {l} {r}"),
                },
                (BinOp::Lt, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 < n2),
                    _ => bail!("Invalid operator for lt: \n{l} \n{r}"),
                },
                (BinOp::Gt, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 > n2),
                    _ => bail!("Invalid operator for gt: {l} {r}"),
                },
                (BinOp::Eq, l, r) => match (l, r) {
                    (Expr::Int(n1), Expr::Int(n2)) => Expr::Bool(n1 == n2),
                    (Expr::String(n1), Expr::String(n2)) => Expr::Bool(n1 == n2),
                    (Expr::Bool(n1), Expr::Bool(n2)) => Expr::Bool(n1 == n2),
                    _ => bail!("Invalid operator for eq: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Or, l, r) => match (l, r) {
                    (Expr::Bool(b1), Expr::Bool(b2)) => Expr::Bool(*b1 || *b2),
                    _ => bail!("Invalid operator for or: {op:?} {l:?} {r:?}"),
                },
                (BinOp::And, l, r) => match (l, r) {
                    (Expr::Bool(b1), Expr::Bool(b2)) => Expr::Bool(*b1 && *b2),
                    _ => bail!("Invalid operator for and: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Concat, l, r) => match (l, r) {
                    (Expr::String(s1), Expr::String(s2)) => {
                        Expr::String((s1.as_ref().clone() + s2.as_ref()).into())
                    }
                    _ => bail!("Invalid operator for concat: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Take, l, r) => match (l, r) {
                    (Expr::Int(n), Expr::String(s)) => Expr::String(
                        s.chars()
                            .take(n.as_ref().try_into().unwrap())
                            .collect::<String>()
                            .into(),
                    ),
                    _ => bail!("Invalid operator for take: {op:?} {l:?} {r:?}"),
                },
                (BinOp::Drop, l, r) => match (l, r) {
                    (Expr::Int(n), Expr::String(s)) => Expr::String(
                        s.chars()
                            .skip(n.as_ref().try_into().unwrap())
                            .collect::<String>()
                            .into(),
                    ),
                    _ => bail!("Invalid operator for drop: {op:?} {l:?} {r:?}"),
                },
                _ => unreachable!(),
            }
        }
        Expr::If(cond, th, el) => {
            let cond = reduce_to_nf(cond.as_ref(), stats)?;
            match cond {
                Expr::Bool(true) => reduce_to_nf(th.as_ref(), stats)?,
                Expr::Bool(false) => reduce_to_nf(el.as_ref(), stats)?,
                _ => bail!("Invalid condition: {cond:?}"),
            }
        }
        // Expr::Lambda(v, e) => Expr::Lambda(*v, reduce(e.as_ref(), env)?.into()),
        _ => e.clone(),
    })
}

fn str_to_int(s: &str) -> BigInt {
    let s = encode_str(s).unwrap();
    let mut ret = BigInt::from(0);
    for c in s.chars() {
        ret = ret * 94 + decode_base94(c).unwrap();
    }
    ret
}

fn int_to_str(n: &BigInt) -> String {
    let zero = BigInt::from(0);

    let mut s = String::new();
    let mut n = n.clone();
    while n > zero {
        let n2 = &n / 94;
        let r: BigInt = &n - &n2 * 94;
        s.push(decode_char(encode_base94(r.try_into().unwrap()).unwrap()).unwrap());
        n = n2;
    }
    s.chars().rev().collect::<String>()
}

fn beta_reduction(e: &Expr, v: usize, arg: &Expr, shadow: &mut Vec<usize>) -> anyhow::Result<Expr> {
    Ok(match e {
        Expr::Var(w) if v == *w && !shadow.contains(w) => arg.clone(),
        Expr::Un(op, e) => Expr::Un(*op, beta_reduction(e.as_ref(), v, arg, shadow)?.into()),
        Expr::Bin(op, l, r) => Expr::Bin(
            *op,
            beta_reduction(l.as_ref(), v, arg, shadow)?.into(),
            beta_reduction(r.as_ref(), v, arg, shadow)?.into(),
        ),
        Expr::If(cond, th, el) => Expr::If(
            beta_reduction(cond.as_ref(), v, arg, shadow)?.into(),
            beta_reduction(th.as_ref(), v, arg, shadow)?.into(),
            beta_reduction(el.as_ref(), v, arg, shadow)?.into(),
        ),
        Expr::Lambda(w, e) => {
            shadow.push(*w);
            let e = beta_reduction(e.as_ref(), v, arg, shadow)?;
            shadow.pop();
            Expr::Lambda(*w, e.into())
        }
        _ => e.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        assert_eq!(str_to_int("test"), 15818151.into());
        assert_eq!(int_to_str(&15818151.into()), "test");
    }

    #[test]
    fn language_test() {
        let expr: Expr = r#"? B= B$ B$ B$ B$ L$ L$ L$ L# v$ I" I# I$ I% I$ ? B= B$ L$ v$ I+ I+ ? B= BD I$ S4%34 S4 ? B= BT I$ S4%34 S4%3 ? B= B. S4% S34 S4%34 ? U! B& T F ? B& T T ? U! B| F F ? B| F T ? B< U- I$ U- I# ? B> I$ I# ? B= U- I" B% U- I$ I# ? B= I" B% I( I$ ? B= U- I" B/ U- I$ I# ? B= I# B/ I( I$ ? B= I' B* I# I$ ? B= I$ B+ I" I# ? B= U$ I4%34 S4%34 ? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.'5!'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4"#.parse().unwrap();
        assert_eq!(
            eval(&expr).unwrap(),
            Expr::String(
                "Self-check OK, send `solve language_test 4w3s0m3` to claim points for it"
                    .to_string()
                    .into()
            )
        );
    }

    #[test]
    fn beta_reductions() {
        let expr: Expr = r#"B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%"#.parse().unwrap();
        let mut stats = Stats::default();
        assert_eq!(
            reduce_to_nf(&expr, &mut stats).unwrap(),
            Expr::Int(BigInt::from(16).into())
        );
        assert_eq!(stats.beta_reductions, 109);
    }

    // #[test]
    // fn slow_eval() {
    //     let expr: Expr = r#"B. S3/,6%},!-"$!-!.V]} B$ B$ B$ L& B$ L8 B$ v& B$ v8 v8 L8 B$ v& B$ v8 v8 L& L# L3 ? B= v# I! v3 B$ B$ v& B/ v# I% B. v3 BT I" BD B% v# I% SFLO> I*]%[Q|~gDDjPpYOigya1+^(F=HN,FJL\,'S-t\:%66snklKUW>~O/0>"cZLOrL^CrUQLWA/!']zMc:*,`,$"UMJQOtY<"~,~=K:X;Jpvrcv=-t'l'?Z2tEC[lF<wMZqnK]Bd5'{2{$t#HH&j6'\Kpcj<D!Kju+7l|*RVW$5n?79wV}N9Z,G}QLB?zT$dOn;Wp<*9A/!tNnYu{U~O6yT3,QL*=(!xpggn+r-ke]I>5|u0)'O$*-z8ri()5~qNo~"U\[f\Da1NkX;O2qj\l|"3Na1F2,)B#A,|y]ptuR[_82(wtDs/nwSsR<>O):'v`d3>NUW6_Wx\=knU')-"w{?DGgGe{4&7[QZ}o?]1A2%l_n#qRyx'kuE`I63TPp0TRDcKgI/w?][2.n(iZL^=80}6B^9H>6okOwZpWkE&3f2&8iAH'YFK~&n(G57Y.[7[e;y>nK?#egP\VW>Z`rJKF6too<D=e1^*r2xALmkL&ipuvFA`CM"a6t:Qzo#|Yc9bb^1X"R;g,X^8/B}_KDnp2*%$y(WBWQGL4tlV}[lycx"NGZ0kGu#~Jm4oQc4D\z5n<'07K6*7?`5c>3frJ6K5f+qz6)jMMXqaCUod|8%T2/n[=6QXv':h$d0F|9DB\oV:CR3I!l$~bsDJY[_E:98wqsfCB5x.z\Emm2>L5Zqd$zjJmW3#KFha6|wj?ArEUixah{>7y_/ic#4so{zJZViFKhuCLz~!r1{m&Inhn@^gmOXX\<RUd!Wn!3N/$=##r7_ob,Vo>;UBvh>ooWwcn@AZgLrS_glq.t$d+Ud^hJ^Cj|R`C[{6AOwMm85Z~39:(8(|%Va_(wSH[eTv1O:Qq4yggW[n"!C|twz:U|/L~S:w+Zeir|9vh]a>2diRwwy&.>vk-=WDybc|(oGo4yM?Hk#WVe=`<](},^;`cARfD4uK-%6lAxuN~Hv"%/6RZF:l'41.j`du3yaqX2Fn6B>?#D@/L<mtYB=qR[-:'D;)njeU!W6sQd?:PR*-"B1@?.~A)-U/,lZNcpSNo%0FF+|7l!sEWBtU.}`~ir^v_~S_\O/^e^3Ywx)Ek^"1XUYVXpWxjAlmsD:;\)?U8Vd8vbSsWj`;2<9IUDPJ-4!7\sw&&^'=:}"b/Nj)%pT}+W$Wa_q7l1fu(P0Zn$81$zh6#F9kV_fm%8Ns:!,xg,wfhwDTnz;;)Lf?eYW=%iqF?iR$3R}U6^l8Fj7:v*Ho,s"$z#j#[{UTfY7]6V<7=*@Pf%,k'G{bh{Q^Rps\,"Ii(/'/0;H#t%UYGus@TNZO]cI<4jcJ&r+cv($9)$o0./JsZPE-{)3k$Ob2hDtMsg_0TgL:NB{oM*(]`KDSE*8`)<K4{pQkG)PT&BA`w&|2Z)C[t4$|z#KI84~z?Q_6!(C\F+b\B6hG#'zQ')ROv1a,H_fo|ji=v]6l5/kW3k?0!zP^]|}'qv-nz@\^^El=n\F=GD5YLS>d}$V'(EDNM`'-:sgp.z]>66TPG+g2DnRxO-)3or^0'|VsfDUotX9}<B7iXC~@O~|:jF6[?Xec<F6jga9Mb&d@ihD;0cc:?$BVtqF5:,-UWa<QS|nx3{ecs/!SfmAB)K=92Fucfv|4Y@Jt9-h,QFH~{.ebAr9]*}>P)!V_\R<'W"N`hL}Ecr1<_>x6zZ2<*|a4#^:K5K&K`7a?;3BKK|;tVT$UnUh2{F32Q4C)wOX2,>hTTPQ+Fv0meL!mx(;,:o{&jX]y=rap4u'm4#zptVOd^y1uI@d%csv9a-6~Dc:ryz<t/AEKJ_6|I;^YTL(j`-0IumZ%}W1BLA4@IHw--@-JM8jb>:^6Lws0uy@+2=DF[s6}WK3!O2qHODL!~7kjnu7s+jy%$@SN(~9d`|?H5sjpXbfH>P>:qYunA55re_?<C;\]mJx3Y>3nx3sY6v5q8sF=aIMq8v+Qt2}nYrz3p|8/Hc\&zID?mLg>j<rP'(rLJX,0a?b;x`<.v&LvM7TxP`#sN}&?Y,YV0v|tqsmUoT_hB0SofRf53(z<9Ij^_P0IM"H_,B4V($|<UCmir`a**IM8DQ%u&D{koEkh"/&v!]>Z3lG1-+r+zU;2-hGD"NJ9yR{:a}t]"U:Ym\oQ+x?f?'ZF<:N:n\ru'*PS~l(1Bae2_a1QW<sTl/uSu$*RY]AUVuY-UJ&P3~d98.gGVGkv.++66sQ4IEPR8r3X2XqD\x$Fe'O9MdKprvhUHsXZ2!XKQ6B~d+k6Knj!qik"rL+^SJt^Ew2)8dU|*!cXwQ>?:+Ym2PIFAb"`At=fr(0<HKI8)4TVC(Vi9XCo(emu'[)5WqBna0(!I=\&_1}a0j(,;9eI2BM@+z9tG;_'lTV1O:%Eyxxw2>|Wn:nL!\vDD;[vE^AYB#klP@V};GPr\D5>^TYC=Co/WX_:Cyc7!+1s5"I&gi!c`N,HL$XPO:{{}s5ci,(}mob.V:]=G{Y8Z"`q&+EY"bWPIw@$ywwRS9W(Md~n>T(6\3snN!+:^)6C|EWfuSQ[BpT3^l8Es~,P;R9OM>agSYkKzIt7hy/`iVusPrk|D7|)"y]y77@v/LW"KN$Aj0xL9e~$6D^X6Ie`UM~:I3DzEj@d{<-'[POhL?4So\elLyvt,eTohGb?)$_)B1'p|Vmw"ejBt[TBM,sD7&}YCB1EZQj@TZEGT&o6<_=F8k>s?i>Tf%4Nozr.>+R&sZ67!giPF5*p[qB0{FsS0<tn_#GDF4'?2O66adwsWl?dyr4B%IH7|QHKYdy;nQMoMsJKB+1[g<M+K+'#&"Y:aPn!hxtDAhJmF_=%']+$ERntOS32FBS=")HLVG5l.BKFAe"KYE:(x6:=A?@"{]g:SC|ad5Hl$J2`-n<:T<5$r~ozEpdK%tb~Scz$p@#^W_@*ty|^5d>N|i4Ea?v*yj:qpca_7~_)5.'x;IHTk:mn9n"u0B25z+pRwf{t,{C7XGko}mw$_w!,^d~oew^f,?R8$t7L!z)FMnNK^>"_yD(6)uWao3BVjm7fS+3zVAQl+UE22knOQWC8Jb7tXo7KBGcvB#rC\oq/PqO1:]LPZ<7K#0/losnT,2e_Mx<a!xLAK4t]_2$8Y`rh[(m[/Nm-45*E.Ua~n=C-ZI"1:6w.Y!lKASt/[H6U,~>ZIS\XwdE^c3EY1;5XjH;t="f,m.:d]zD1py"BZ*I|4!/0/@1BE/uv+{C+%tzif$!l@9*~,TVc1pPhM~y*fT2B#=(Tj1=|6j5AghDJNf/B?lU0j[(Xg6KtIB's[y|D.BYNX9Am'~>lg]NKaBF;@wi14Ce"k%:6tZ7Ks2K54qMUf,|3uu)Yw;~n3S)sJBc(m)$+I~d9X<.v%X4wXZLAL9AMot65(Sthq6:=m[0hj)=E*N^GI4\Ub8\XzGf7n}P;~T:#TUe@.2U&.|_SW|_6%gh"pgqID1Y1"aeyDgs0'4.Bgpi#B"C@zw`dBXu&>?R^14+u\y\k0Gil?af%a~}(fx$R#3qP0$)J+x:/5'5UnX`z}91bB?@UlnLL%IzG5ZM45$j1RLXl*u]%M4u$~>S3NYXSUwX]Om*{y7@NgwF0OoYLl-_^owt2zQ>k^=0gT${Ys2Pv60cdO4?LzOw#,1b4$db)[tf_G^hb~%n4gBH@VrYAqkop4PYD,>UFCHI;@w&[O<dz@CNL@~?]vEzBDP@WYev.7`vCqV[6E["(i%/vH0>+U]{cF;vW[tIjBrN5\.Z|`u.+XnmDkf.R]-,n7h~o);)'L:3'Jgu</FMAv[{}HQ=t^X\m?InR\Pe2mE.`}*D';&rM,a\:WH6V@%.?HqIt:'5`l+axO668u%Q2SB/3Yd@p\4ak>]|(>|>k+>B5"!k3EmvC9Ed_X7Q#.;B>GCw}$zJQz{O*7DMH?zLnR+bB~Zj8%c_|nfQoZsov`3KRTF>_)*pzc>WlS;l%L{'uJ9j'v=K.^YH%Ckx:JmpO7AG]]e,M:81u/,owldge;]UL|cdmz04Xyx/~@j@r-DQa*-?9bw^b]|<"yUMNvQ`vg&}TF8.6I}pOlI{t.$}AM;A@kS3W,-VMa,M)MN7)/45qYLHd7!IXOe,A\~m']e(jrec0^sf]O-|CddG;A~~nFRPq\])9r@!{XAv\m|>lt}zvuaEqjR.(%|0qkn8%-%94=5oGG;:5YUu1P[O,\H)U)L*xN#i&l{K2MA%y8z|mG$>2@P[=9L~bbH~3dIAkp<OwH[FuBf}(Kh9d/*KA/k5e)zT=y/[$Ii;Q05u<;vhS0VK_"19O6-%vq/8L;ljf>^AK}BA/qd8Z&4Q@.;:289~>[E}[ol[g0XrC#2E!"]4&h7)@S[3j,st|kL?EfUF78T@Vza56sfU'L4d{Xhym:HXYDO\c<yg~O,1~dNL?^e_-g,Mp\,MlH>kbL~JtEq!iof{;?]Rfp6vJq0Mgu`(xe$_gELgv5b%XAm<>b"!,o$a)HnW(<Y@h';G8tBtBsj]J)!`_j5}o6S>}\,h>*p@P_p{r?od2U,m!PZP!^N#1<N3'i?of&GE^3l)DO""jUW8eSVmI,tlFsF`W+U?1UB2lC'{z S"#.parse().unwrap();
    //     eval(&expr).unwrap();
    // }
}
