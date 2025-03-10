/* A file to just translate chars to their correct tex coords offsets */
/* Quicker to have it precomputed */

pub const CHAR_TO_TEX: [[f32;2];127] = [
    [0.0,0.0], //SOH
    [0.0,0.0], //STX
    [0.0,0.0], //ETX
    [0.0,0.0], //EOT
    [0.0,0.0], //ENQ
    [0.0,0.0], //ACK
    [0.0,0.0], //BEL
    [0.0,0.0], //BS
    [0.0,0.0], //TAB
    [0.0,0.0], //LF
    [0.0,0.0], //VT
    [0.0,0.0], //FF
    [0.0,0.0], //VT
    [0.0,0.0], //CR
    [0.0,0.0], //SO
    [0.0,0.0], //SI
    [0.0,0.0], //DLE
    [0.0,0.0], //DC1
    [0.0,0.0], //dc2
    [0.0,0.0], //dc3
    [0.0,0.0], //dc4
    [0.0,0.0], //NAK
    [0.0,0.0], //SYN
    [0.0,0.0], //ETB
    [0.0,0.0], //CAN
    [0.0,0.0], //EM
    [0.0,0.0], //SUB
    [0.0,0.0], //ESC
    [0.0,0.0], //FS
    [0.0,0.0], //GSL
    [0.0,0.0], //RS
    [0.0,0.0], //US
    [0.0,0.875], //Space
    [0.0625,0.875], //\!
    [0.125,0.875], //"
    [0.1875,0.875], //#
    [0.25,0.875], //$
    [0.3125,0.875], //%
    [0.375,0.875], //&
    [0.4375,0.875], //'
    [0.5,0.875], //(
    [0.5625,0.875], //)
    [0.625,0.875], //*
    [0.6875,0.875], //+
    [0.75,0.875], //,
    [0.8125,0.875], //-
    [0.875,0.875], //.
    [0.9375,0.875], ///
    [0.0,0.75], //0
    [0.0625,0.75], //1
    [0.125,0.75], //2
    [0.1875,0.75], //3
    [0.25,0.75], //4
    [0.3125,0.75], //5
    [0.375,0.75], //6
    [0.4375,0.75], //7
    [0.5,0.75], //8
    [0.5625,0.75], //9
    [0.625,0.75], //:
    [0.6875,0.75], //;
    [0.75,0.75], //<
    [0.8125,0.75], //=
    [0.875,0.75], //>
    [0.9375,0.75], //?
    [0.0,0.625], //@
    [0.0625,0.625], //A
    [0.125,0.625], //B
    [0.1875,0.625], //C
    [0.25,0.625], //D
    [0.3125,0.625], //E
    [0.375,0.625], //F
    [0.4375,0.625], //G
    [0.5,0.625], //H
    [0.5625,0.625], //I
    [0.625,0.625], //J
    [0.6875,0.625], //K
    [0.75,0.625], //L
    [0.8125,0.625], //M
    [0.875,0.625], //N
    [0.9375,0.625], //O
    [0.0,0.5], //P
    [0.0625,0.5], //Q
    [0.125,0.5], //R
    [0.1875,0.5], //S
    [0.25,0.5], //T
    [0.3125,0.5], //U
    [0.375,0.5], //V
    [0.4375,0.5], //W
    [0.5,0.5], //X
    [0.5625,0.5], //Y
    [0.625,0.5], //Z
    [0.6875,0.5], //[
    [0.75,0.5], //\
    [0.8125,0.5], //]
    [0.875,0.5], //^
    [0.9375,0.5], //_
    [0.0,0.375], //`
    [0.0625,0.375], //a
    [0.125,0.375], //b
    [0.1875,0.375], //c
    [0.25,0.375], //d
    [0.3125,0.375], //e
    [0.375,0.375], //f
    [0.4375,0.375], //g
    [0.5,0.375], //h
    [0.5625,0.375], //i
    [0.625,0.375], //j
    [0.6875,0.375], //k
    [0.75,0.375], //l
    [0.8125,0.375], //m
    [0.875,0.375], //n
    [0.9375,0.375], //o
    [0.0,0.25], //p
    [0.0625,0.25], //q
    [0.125,0.25], //r
    [0.1875,0.25], //s
    [0.25,0.25], //t
    [0.3125,0.25], //u
    [0.375,0.25], //v
    [0.4375,0.25], //w
    [0.5,0.25], //x
    [0.5625,0.25], //y
    [0.625,0.25], //z
    [0.6875,0.25], //{
    [0.75,0.25], //|
    [0.8125,0.25], //}
    [0.875,0.25], //~
    ];

    const CHAR_TO_TEX_EXACT: [[[f32;2];4];127] = [
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //NUL
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //SOH
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //STX
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //ETX
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //EOT
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //ENQ
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //ACK
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //BEL
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //BS
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //TAB
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //LF
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //VT
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //FF
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //VT
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //CR
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //SO
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //SI
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //DLE
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //DC1
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //dc2
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //dc3
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //dc4
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //NAK
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //SYN
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //ETB
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //CAN
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //EM
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //SUB
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //ESC
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //FS
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //GSL
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //RS
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //US
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //Space
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //\!
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //"
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //#
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //$
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //%
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //&
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //'
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //(
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //)
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //*
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //+
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //,
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //-
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //.
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], ///
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //0
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //1
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //2
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //3
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //4
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //5
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //6
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //7
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //8
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //9
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //:
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //;
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //<
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //=
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //>
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //?
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //@
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //A
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //B
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //C
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //D
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //E
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //F
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //G
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //H
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //I
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //J
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //K
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //L
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //M
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //N
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //O
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //P
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //Q
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //R
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //S
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //T
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //U
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //V
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //W
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //X
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //Y
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //Z
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //[
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //\
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //]
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //^
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //_
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //`
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //a
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //b
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //c
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //d
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //e
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //f
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //g
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //h
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //i
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //j
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //k
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //l
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //m
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //n
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //o
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //p
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //q
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //r
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //s
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //t
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //u
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //v
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //w
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //x
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //y
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //z
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //{
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //|
        [[0.0,0.0],[0.0,0.0],[0.0,0.0],[0.0,0.0]], //}
        ];