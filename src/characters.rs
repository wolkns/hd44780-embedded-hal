
/// Non ASCII chars for ROM Coce: A00 (japanese)
/// This ROM include ASCII chars: ' ' (space) <-> '~'
///                               0x20        <-> 0x7d
/// Except the ASCI '\' char (0x5c) represents the Yen Sign
pub enum NonASCIIA00 {
    // ASCII Realm
    // 0x00 <-> 0x0f  CG RAM
    // 0x10 <-> 0x1f  All White
    // 0x20 <-> 0x5b  Corresponde with ASCI
    YenSign             = 0x5c, // U+00A5 , ASCII: '\'
    // 0x5d <-> 0x7d  Corresponde with ASCI
    RightwardsArrow     = 0x7e, // U+2192
    LeftwardsArrow      = 0x7f, // U+2190

    // None ASCII Realm
    // 0x80 <-> 0x9f  All White
    // 0xa0           All White
    
    // TODO 0xa1
    // TODO 0xa2
    // TODO 0xa3
    KatakanaIterationMark = 0xa4, // U+30fd
    KatakanaMiddleDot   = 0xa5, // U+30fb
    KatakanaWo          = 0xa6, // U+30f2
    KatakanaASmall      = 0xa7, // U+30a1
    KatakanaISmall      = 0xa8, // U+30a3
    KatakanaUSmall      = 0xa9, // U+30a5
    KatakanaESmall      = 0xaa, // U+30a7
    KatakanaOSmall      = 0xab, // U+30a9
    KatakanaYaSmall     = 0xac, // U+30e3
    KatakanaYuSmall     = 0xad, // U+30e5
    KatakanaYoSmall     = 0xae, // U+30e7
    KatakanaTuSmall     = 0xaf, // U+30c3
    
    KatakanaHiraganaProlongedSoundMark = 0xb0, // U+30fc
    KatakanaA           = 0xb1, // U+30a2
    KatakanaI           = 0xb2, // U+30a4
    KatakanaU           = 0xb3, // U+30a6
    KatakanaE           = 0xb4, // U+30a8
    KatakanaO           = 0xb5, // U+30aa
    KatakanaKa          = 0xb6, // U+30ab
    KatakanaKi          = 0xb7, // U+30ad
    KatakanaKu          = 0xb8, // U+30af
    KatakanaKe          = 0xb9, // U+30b1
    KatakanaKo          = 0xba, // U+30b3
    KatakanaSa          = 0xbb, // U+30b5
    KatakanaSi          = 0xbc, // U+30b7
    KatakanaSu          = 0xbd, // U+30b9
    KatakanaSe          = 0xbe, // U+30bb
    KatakanaSo          = 0xbf, // U+30bd

    KatakanaTa          = 0xc0, // U+30bf
    KatakanaTi          = 0xc1, // U+30c1
    KatakanaTu          = 0xc2, // U+30c4
    KatakanaTe          = 0xc3, // U+30c6
    KatakanaTo          = 0xc4, // U+30c8
    KatakanaNa          = 0xc5, // U+30ca
    KatakanaNi          = 0xc6, // U+30cb
    KatakanaNu          = 0xc7, // U+30cc
    KatakanaNe          = 0xc8, // U+30cd
    KatakanaNo          = 0xc9, // U+30ce
    KatakanaHa          = 0xca, // U+30cf
    KatakanaHi          = 0xcb, // U+30d2
    KatakanaHu          = 0xcc, // U+30d5
    KatakanaHe          = 0xcd, // U+30d8
    KatakanaHo          = 0xce, // U+30db
    KatakanaMa          = 0xcf, // U+30de

    KatakanaMi          = 0xd0, // U+30df
    KatakanaMu          = 0xd1, // U+30e0
    KatakanaMe          = 0xd2, // U+30e1
    KatakanaMo          = 0xd3, // U+30e2
    KatakanaYa          = 0xd4, // U+30e4
    KatakanaYu          = 0xd5, // U+30e6
    KatakanaYo          = 0xd6, // U+30e8
    KatakanaRa          = 0xd7, // U+30e9
    KatakanaRi          = 0xd8, // U+30ea
    KatakanaRu          = 0xd9, // U+30eb
    KatakanaRe          = 0xda, // U+30ec
    KatakanaRo          = 0xdb, // U+30ed
    KatakanaWa          = 0xdc, // U+30ef
    KatakanaN           = 0xdd, // U+30f3
    KatakanaHiraganaVoicedSoundMark = 0xde, // U+309b
    KatakanaHiraganaSemiVoicedSoundMark = 0xdf, // U+309c

    GreekSmallAlpha     = 0xe0, // U+03b1
    LatinSmallUmlautA   = 0xe1, // U+00e4
    GreekSmallBeta      = 0xe2, // U+03b2
    GreekSmallEpsilon   = 0xe3, // U+03b5
    GreekSmallMu        = 0xe4, // U+03bc
    GreekSmallSigma     = 0xe5, // U+03c3
    GreekSmallRho       = 0xe6, // U+03c1
    LatinSmallGDecender = 0xe7, // ---
    SquareRoot          = 0xe8, // U+221a
    // TODO 0xe9 (power of negativ 1)
    LatinSmallJDecender = 0xea, // ---
    SuperscriptX        = 0xeb, // ---
    CentSign            = 0xec, // U+00a2
    // TODO 0xed
    LatinSmallNLineAbove = 0xee, // ---
    LatinSmallUmlautO   = 0xef, // U+00f6

    LatinSmallPDecender = 0xf0, // ---
    LatinSmallQDecender = 0xf1, // ---
    GreekSmallTheta     = 0xf2, // U+03b8
    Infinity            = 0xf3, // U+221E
    GreekCapitalOmega   = 0xf4, // U+03a9
    LatinSmallUmlautU   = 0xf5, // U+00fc
    GreekCapitalSigma   = 0xf6, // U+03a3
    GreekSmallPi        = 0xf7, // U+03c0
    LatinSmallXBar      = 0xf8, // Average
    LatinSmallYDecender = 0xf9, // ---
    // TODO 0xfa (probably japanese)
    // TODO 0xfb (probably japanese)
    // TODO 0xfc (probably japanese)
    DivisionSign        = 0xfd, // U+00f7
    AllWhite            = 0xfe,
    AllBlack            = 0xff,
}



/// Non ASCII chars for ROM Coce: A02 (european)
/// This ROM include ASCII chars: ' ' (space) <-> '~'
///                               0x20        <-> 0x7e
pub enum NonASCIIA02 {
    // ASCII Realm
    // 0x00 <-> 0x0f  CG RAM
    TrianglePointingRight = 0x10, // U+2bc8
    TrianglePointingLeft  = 0x11, // U+2bc7
    QuotationMarkLeftDouble  = 0x12, // U+201c
    QuotationMarkRightDouble = 0x13, // U+201d
    TriangleDoubleUp      = 0x14, // ---
    TriangleDoubleDown    = 0x15, // ---
    BlackLargeCircle      = 0x16, // U+2b24
    ArrowDownLeft         = 0x17, // U+21b5 (EnterSign)
    UpwardsArrow          = 0x18, // U+2191
    DownwardsArrow        = 0x19, // U+2193
    RightwardsArrow       = 0x1a, // U+2192
    LeftwardsArrow        = 0x1b, // U+2190
    LessThanOrEqual       = 0x1c, // U+2264
    GreaterThanOrEqual    = 0x1d, // U+2265
    TrianglePointingUp    = 0x1e, // U+2bc5
    TrianglePointingDown  = 0x1f, // U+2bc6
    // 0x20 <-> 0x7e  Corresponde with ASCI
    House  = 0x7f, // U+2303

    // None ASCII Realm
    CyrillicBe            = 0x80, // U+0411
    CyrillicDe            = 0x81, // U+0414
    CyrillicZhe           = 0x82, // U+0416
    CyrillicZe            = 0x83, // U+0417
    CyrillicI             = 0x84, // U+0418
    CirillicShortI        = 0x85, // U+0419
    CirillicEl            = 0x86, // U+041b
    CirillicPe            = 0x87, // U+041f
    CyrillicU             = 0x88, // U+0423
    CyrillicTse           = 0x89, // U+0426
    CyrillicChe           = 0x8a, // U+0427
    CyrillicSha           = 0x8b, // U+0428
    CyrillicShcha         = 0x8c, // U+0429
    CyrillicHardSign      = 0x8d, // U+042a
    CyrillicYeru          = 0x8e, // U+042b
    // TODO 0x8f
    

    GreekSmallAlpha       = 0x90, // U+03b1
    EighthNote            = 0x91, // U+266a
    GreekCapitalGamma     = 0x92, // U+0393
    GreekSmallPi          = 0x93, // U+03c0
    GreekCapitalSigma     = 0x94, // U+03a3
    GreekSmallSigma       = 0x95, // U+03c3
    BeamedDecendingNote   = 0x96, // U+1f39d
    GreekSmallTau         = 0x97, // U+03c4
    BellSymbol            = 0x98, // ---
    GreekCapitalTheta     = 0x99, // U+0398
    GreekCapitalOmega     = 0x9a, // U+03a9
    GreekSmallDelta       = 0x9b, // U+03b4
    Infinity              = 0x9c, // U+221E
    BlackHeartSuit        = 0x9d, // U+2665
    GreekSmallEpsilon     = 0x9e, // U+03b5
    // TODO 0x9f

    HeavyVerticalBars     = 0xa0, // ---
    InvertedExclamationMark = 0xa1, // U+00a1
    CentSign              = 0xa2, // U+00a2
    PoundSign             = 0xa3, // U+00a3
    CurrencySign          = 0xa4, // U+00a4
    YenSign               = 0xa5, // U+00a5
    BrokenBar             = 0xa6, // U+00a6
    SectionSign           = 0xa7, // U+00a7
    // TODO 0xa8
    CopyrightSign         = 0xa9, // U+00a9
    LatinSmallAUnderbar   = 0xaa, // ---
    DoubleAngleQuotationMarkLeft = 0xab, // U+00ab
    CyrillicYu            = 0xac, // U+042e
    CyrillicYa            = 0xad, // U+042f
    RegisteredSign        = 0xae, // U+00ae
    // TODO 0xaf

    DegreeSign            = 0xb0, // U+00b0
    PlusMinusSign         = 0xb1, // U+00b1
    SuperscriptTwo        = 0xb2, // U+00b2
    SuperscriptThree      = 0xb3, // U+00b3
    // TODO 0xb4
    MicroSign             = 0xb5, // U+00b5
    PilcrowSign           = 0xb6, // U+00b6
    MiddleDot             = 0xb7, // U+00b7
    GreekSmallOmega       = 0xb8, // U+03c9
    SuperscriptOne        = 0xb9, // ---
    LatinSmallOUnderbar   = 0xba, // ---
    DoubleAngleQuotationMarkRight = 0xbb, // U+00bb
    FractionOneQuarter    = 0xbc, // U+00bc
    FractionOneHalf       = 0xbd, // U+00bd
    FractionThreeQuarter  = 0xbe, // U+00be
    InvertedQuestionMark  = 0xbf, // U+00bf

    LatinCapitalAGrave    = 0xc0, // U+00c0
    LatinCapitalAAcute    = 0xc1, // U+00c1
    LatinCapitalACircumflex = 0xc2, // U+00c2
    LatinCapitalATilde    = 0xc3, // U+00c3
    LatinCapitalADiaeresis = 0xc4, // U+00c4
    LatinCapitalARingAbove = 0xc5, // U+00c5
    LatinCapitalAE         = 0xc6, // U+00c6
    LatinCapitalCCedilla   = 0xc7, // U+00c7
    
    LatinCapitalEGrave    = 0xc8, // U+00c8
    LatinCapitalEAcute    = 0xc9, // U+00c9
    LatinCapitalECircumflex = 0xca, // U+00ca
    LatinCapitalEDiaeresis = 0xcb, // U+00cb

    LatinCapitalIGrave    = 0xcc, // U+00cc
    LatinCapitalIAcute    = 0xcd, // U+00cd
    LatinCapitalICircumflex = 0xce, // U+00ce
    LatinCapitalIDiaeresis = 0xcf, // U+00cf
    
    LatinCapitalEth       = 0xd0, // U+00d0
    LatinCapitalNTilde    = 0xd1, // U+00d1

    LatinCapitalOGrave    = 0xd2, // U+00d2
    LatinCapitalOAcute    = 0xd3, // U+00d3
    LatinCapitalOCircumflex = 0xd4, // U+00d4
    LatinCapitalOTilde    = 0xd5, // U+00d5
    LatinCapitalODiaeresis = 0xd6, // U+00d6

    MultiplicationSign    = 0xd7, // U+00d7

    LatinCapitalOStroke   = 0xd8, // U+03a6 (GreekCapitalPhi)

    LatinCapitalUGrave    = 0xd9, // U+00d9
    LatinCapitalUAcute    = 0xda, // U+00da
    LatinCapitalUCircumflex = 0xdb, // U+00db
    LatinCapitalUDiaeresis = 0xdc, // U+00dc

    LatinCapitalYAcute    = 0xdd, // U+00dd
    LatinCapitalThorn     = 0xde, // U+00de
    LatinSmallSharpS      = 0xdf, // U+00df

    LatinSmallAGrave      = 0xe0, // U+00e0
    LatinSmallAAcute      = 0xe1, // U+00e1
    LatinSmallACircumflex = 0xe2, // U+00e2
    LatinSmallATilde      = 0xe3, // U+00e3
    LatinSmallADiaeresis  = 0xe4, // U+00e4
    LatinSmallARingAbove  = 0xe5, // U+00e5
    LatinSmallAE          = 0xe6, // U+00e6
    LatinSmallCCedilla    = 0xe7, // U+00e7

    LatinSmallEGrave      = 0xe8, // U+00e8
    LatinSmallEAcute      = 0xe9, // U+00e9
    LatinSmallECircumflex = 0xea, // U+00ea
    LatinSmallEDiaeresis  = 0xeb, // U+00eb

    LatinSmallIGrave      = 0xec, // U+00ec
    LatinSmallIAcute      = 0xed, // U+00ed
    LatinSmallICircumflex = 0xee, // U+00ee
    LatinSmallIDiaeresis  = 0xef, // U+00ef

    LatinSmallEth         = 0xf0, // U+00f0
    LatinSmallNTilde      = 0xf1, // U+00f1

    LatinSmallOGrave      = 0xf2, // U+00f2
    LatinSmallOAcute      = 0xf3, // U+00f3
    LatinSmallOCircumflex = 0xf4, // U+00f4
    LatinSmallOTilde      = 0xf5, // U+00f5
    LatinSmallODiaeresis  = 0xf6, // U+00f6

    DivisionSign          = 0xf7, // U+00f7

    LatinSmallOStroke     = 0xf8, // U+00f8

    LatinSmallUGrave      = 0xf9, // U+00f9
    LatinSmallUAcute      = 0xfa, // U+00fa
    LatinSmallUCircumflex = 0xfb, // U+00fb
    LatinSmallUDiaeresis  = 0xfc, // U+00fc

    LatinSmallYAcute      = 0xfd, // U+00fd
    LatinSmallThorn       = 0xfe, // U+00fe
    LatinSmallYDiaeresis  = 0xff, // U+00ff
}


/// There are 8 possible Custom Chars for Font 5x8.
/// Character codes are: 0b0000*xxx
/// * has no effect. x bits can be 0 or 1
/// 
pub enum CustomFont5x8 {
    Char0=0b0000_1000,
    Char1=0b0000_1001,
    Char2=0b0000_1010,
    Char3=0b0000_1011,
    Char4=0b0000_1100,
    Char5=0b0000_1101,
    Char6=0b0000_1110,
    Char7=0b0000_1111,
}


/// There are 4 possible Custom Chars for Font 5x10.
/// Character codes are: 0b0000*xx*
/// * has no effect. x bits can be 0 or 1
/// 
pub enum CustomFont5x10 {
    Char0=0b0000_1001,
    Char1=0b0000_1011,
    Char2=0b0000_1101,
    Char3=0b0000_1111,
}