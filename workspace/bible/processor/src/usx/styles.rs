#![allow(non_camel_case_types)]

use std::fmt;
use std::str::FromStr;

const ENUM_ERROR: &str = "Invalid";

macro_rules! define_string_convertible_enum {
    ($name:ident, [$($variants:ident),*]) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($variants),*
        }

        impl $name {

            pub fn to_str_name() -> Vec<String> {
                vec![$(stringify!($variants).to_string().replace("CHAP", "")),+]
            }
        }

        impl FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($variants) => Ok($name::$variants),)*
                    _ => Err(format!("{ENUM_ERROR} {}: {}", stringify!($name), s)),
                }
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                match s {
                    $(stringify!($variants) => $name::$variants,)*
                    _ => panic!("{}", format!("{ENUM_ERROR} {}: {}", stringify!($name), s)),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        $name::$variants => write!(f, stringify!($variants)),
                    )*
                }
            }
        }
    };
}

// Define enums using the new macro
define_string_convertible_enum!(
    BookHeaderStyle,
    [ide, h, h1, h2, h3, toc1, toc2, toc3, toca1, toca2, toca3, rem, usfm]
);
define_string_convertible_enum!(
    BookTitleStyle,
    [mt, mt1, mt2, mt3, mt4, imt, imt1, imt2, rem]
);
define_string_convertible_enum!(
    BookIntroductionStyle,
    [
        imt, imt1, imt2, imt3, imt4, ib, ie, ili, ili1, ili2, im, imi, imq, io, io1, io2, io3, io4,
        iot, ip, ipi, ipr, iq, iq1, iq2, iq3, is, is1, is2, imte, imte1, imte2, iex, rem
    ]
);
define_string_convertible_enum!(
    BookIntroductionEndTitleStyle,
    [mt, mt1, mt2, mt3, mt4, imt, imt1, imt2]
);
define_string_convertible_enum!(BookChapterLabelStyle, [cl]);
define_string_convertible_enum!(CrossReferenceStyle, [x, ex]);
define_string_convertible_enum!(
    CrossReferenceCharStyle,
    [xo, xop, xt, xta, xk, xq, xot, xnt, xdc]
);
define_string_convertible_enum!(IntroCharStyle, [Ior, Iqt]);
define_string_convertible_enum!(
    CharStyle,
    [
        va, vp, ca, qac, qs, add, addpn, bk, dc, efm, fm, k, nd, ndx, ord, pn, png, pro, qt, rq,
        sig, sls, tl, wg, wh, wa, wj, xt, jmp, no, it, bd, bdit, em, sc, sup
    ]
);
define_string_convertible_enum!(FootnoteStyle, [f, fe, ef]);
define_string_convertible_enum!(
    FootnoteCharStyle,
    [fr, cat, ft, fk, fq, fqa, fl, fw, Fp, fv, fdc, xt, it, bd, bdit, em, sc]
);
define_string_convertible_enum!(FootnoteVerseStyle, [fv]);
define_string_convertible_enum!(SidebarStyle, [esb]);
define_string_convertible_enum!(
    ListStyle,
    [lh, li, li1, li2, li3, li4, lf, lim, lim1, lim2, lim3, lim4]
);
define_string_convertible_enum!(
    ParaStyle,
    [
        restore, cls, iex, ip, lit, m, mi, nb, p, pb, pc, pi, pi1, pi2, pi3, po, pr, pmo, pm, pmc,
        pmr, ph, ph1, ph2, ph3, q, q1, q2, q3, q4, qa, qc, qr, qm, qm1, qm2, qm3, qd, b, d, ms,
        ms1, ms2, ms3, mr, r, s, s1, s2, s3, s4, sr, sp, sd, sd1, sd2, sd3, sd4, ts, cp, cl, cd,
        mte, mte1, mte2, p1, p2, k1, k2, rem
    ]
);
define_string_convertible_enum!(VerseStartStyle, [v]);
define_string_convertible_enum!(
    ListCharStyle,
    [litl, lik, liv, liv1, liv2, liv3, liv4, liv5]
);

define_string_convertible_enum!(
    BookIdentificationCode,
    [
        ChapGEN, ChapEXO, ChapLEV, ChapNUM, ChapDEU, ChapJOS, ChapJDG, ChapRUT, Chap1SA, Chap2SA,
        Chap1KI, Chap2KI, Chap1CH, Chap2CH, ChapEZR, ChapNEH, ChapEST, ChapJOB, ChapPSA, ChapPRO,
        ChapECC, ChapSNG, ChapISA, ChapJER, ChapLAM, ChapEZK, ChapDAN, ChapHOS, ChapJOL, ChapAMO,
        ChapOBA, ChapJON, ChapMIC, ChapNAM, ChapHAB, ChapZEP, ChapHAG, ChapZEC, ChapMAL, ChapMAT,
        ChapMRK, ChapLUK, ChapJHN, ChapACT, ChapROM, Chap1CO, Chap2CO, ChapGAL, ChapEPH, ChapPHP,
        ChapCOL, Chap1TH, Chap2TH, Chap1TI, Chap2TI, ChapTIT, ChapPHM, ChapHEB, ChapJAS, Chap1PE,
        Chap2PE, Chap1JN, Chap2JN, Chap3JN, ChapJUD, ChapREV, ChapTOB, ChapJDT, ChapESG, ChapWIS,
        ChapSIR, ChapBAR, ChapLJE, ChapS3Y, ChapSUS, ChapBEL, Chap1MA, Chap2MA, Chap3MA, Chap4MA,
        Chap1ES, Chap2ES, ChapMAN, ChapPS2, ChapODA, ChapPSS, ChapEZA, Chap5EZ, Chap6EZ, ChapDAG,
        ChapPS3, Chap2BA, ChapLBA, ChapJUB, ChapENO, Chap1MQ, Chap2MQ, Chap3MQ, ChapREP, Chap4BA,
        ChapLAO
    ]
);
