use super::machine::ReadWrite;

const OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn ReadWrite) -> u8, u8, u8); 256] = [
/*         0                                             1                                              2                                             3                                             4                                             5                                              6                                       7                                             8                                          9                                            a                                          b                                          c                                          d                                          e                                       f                                              */
/* 00 */ ("NOP"       , Z80CPU::nop           , 0, 4), ("LD BC"     , Z80CPU::ld_bc_nn      , 2, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("INC B"     , Z80CPU::inc_b         , 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("LD B", Z80CPU::ld_b_n        , 1, 7), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("INC C"  , Z80CPU::inc_c         , 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD C", Z80CPU::ld_c_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 00 */
/* 10 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("LD DE"     , Z80CPU::ld_de_nn      , 2, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("INC D"     , Z80CPU::inc_d         , 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("LD D", Z80CPU::ld_d_n        , 1, 7), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("INC E"  , Z80CPU::inc_e         , 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD E", Z80CPU::ld_e_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 10 */
/* 20 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("LD HL"     , Z80CPU::ld_hl_nn      , 2, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("INC H"     , Z80CPU::inc_h         , 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("LD H", Z80CPU::ld_h_n        , 1, 7), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("INC L"  , Z80CPU::inc_l         , 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD L", Z80CPU::ld_l_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 20 */
/* 30 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("LD SP"     , Z80CPU::ld_sp_nn      , 2, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("INC A"  , Z80CPU::inc_a         , 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD A", Z80CPU::ld_a_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 30 */
/* 40 */ ("LD B, B"   , Z80CPU::ld_b_b        , 0, 4), ("LD B, C"   , Z80CPU::ld_b_c        , 0,  4), ("LD B, D"   , Z80CPU::ld_b_d        , 0, 4), ("LD B, E"   , Z80CPU::ld_b_e        , 0, 4), ("LD B, H"   , Z80CPU::ld_b_h        , 0, 4), ("LD B, L"   , Z80CPU::ld_b_l        , 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD B, A"   , Z80CPU::ld_b_a        , 0, 4), ("LD C, B", Z80CPU::ld_c_b        , 0, 4), ("LD C, C"  , Z80CPU::ld_c_c        , 0, 4), ("LD C, D", Z80CPU::ld_c_d        , 0, 4), ("LD C, E", Z80CPU::ld_c_e        , 0, 4), ("LD C, H", Z80CPU::ld_c_h        , 0, 4), ("LD C, L", Z80CPU::ld_c_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD C, A", Z80CPU::ld_c_a        , 0, 4), /* 40 */
/* 50 */ ("LD D, B"   , Z80CPU::ld_d_b        , 0, 4), ("LD D, C"   , Z80CPU::ld_d_c        , 0,  4), ("LD D, D"   , Z80CPU::ld_d_d        , 0, 4), ("LD D, E"   , Z80CPU::ld_d_e        , 0, 4), ("LD D, H"   , Z80CPU::ld_d_h        , 0, 4), ("LD D, L"   , Z80CPU::ld_d_l        , 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD D, A"   , Z80CPU::ld_d_a        , 0, 4), ("LD E, B", Z80CPU::ld_e_b        , 0, 4), ("LD E, C"  , Z80CPU::ld_e_c        , 0, 4), ("LD E, D", Z80CPU::ld_e_d        , 0, 4), ("LD E, E", Z80CPU::ld_e_e        , 0, 4), ("LD E, H", Z80CPU::ld_e_h        , 0, 4), ("LD E, L", Z80CPU::ld_e_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD E, A", Z80CPU::ld_e_a        , 0, 4), /* 50 */
/* 60 */ ("LD H, B"   , Z80CPU::ld_h_b        , 0, 4), ("LD H, C"   , Z80CPU::ld_h_c        , 0,  4), ("LD H, D"   , Z80CPU::ld_h_d        , 0, 4), ("LD H, E"   , Z80CPU::ld_h_e        , 0, 4), ("LD H, H"   , Z80CPU::ld_h_h        , 0, 4), ("LD H, L"   , Z80CPU::ld_h_l        , 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD H, A"   , Z80CPU::ld_h_a        , 0, 4), ("LD L, B", Z80CPU::ld_l_b        , 0, 4), ("LD L, C"  , Z80CPU::ld_l_c        , 0, 4), ("LD L, D", Z80CPU::ld_l_d        , 0, 4), ("LD L, E", Z80CPU::ld_l_e        , 0, 4), ("LD L, H", Z80CPU::ld_l_h        , 0, 4), ("LD L, L", Z80CPU::ld_l_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD L, A", Z80CPU::ld_l_a        , 0, 4), /* 60 */
/* 70 */ ("LD (HL), B", Z80CPU::ld_ptr_hl_b   , 0, 7), ("LD (HL), C", Z80CPU::ld_ptr_hl_c   , 0,  7), ("LD (HL), D", Z80CPU::ld_ptr_hl_d   , 0, 7), ("LD (HL), E", Z80CPU::ld_ptr_hl_e   , 0, 7), ("LD (HL), H", Z80CPU::ld_ptr_hl_h   , 0, 7), ("LD (HL), L", Z80CPU::ld_ptr_hl_l   , 0,  7), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD (HL), A", Z80CPU::ld_ptr_hl_a   , 0, 7), ("LD A, B", Z80CPU::ld_a_b        , 0, 4), ("LD A, C"  , Z80CPU::ld_a_c        , 0, 4), ("LD A, D", Z80CPU::ld_a_d        , 0, 4), ("LD A, E", Z80CPU::ld_a_e        , 0, 4), ("LD A, H", Z80CPU::ld_a_h        , 0, 4), ("LD A, L", Z80CPU::ld_a_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD A, A", Z80CPU::ld_a_a        , 0, 4), /* 70 */
/* 80 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 80 */
/* 90 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 90 */
/* a0 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* a0 */
/* b0 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* b0 */
/* c0 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("POP BC"    , Z80CPU::pop_bc        , 0, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("PUSH BC"   , Z80CPU::push_bc       , 0, 11), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* c0 */
/* d0 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("POP DE"    , Z80CPU::pop_de        , 0, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("PUSH DE"   , Z80CPU::push_de       , 0, 11), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("IX"     , Z80CPU::ix            , 0, 0), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* d0 */
/* e0 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("POP HL"    , Z80CPU::pop_hl        , 0, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("PUSH HL"   , Z80CPU::push_hl       , 0, 11), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* e0 */
/* f0 */ ("???"       , Z80CPU::invalid_opcode, 0, 4), ("POP AF"    , Z80CPU::pop_af        , 0, 10), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("PUSH AF"   , Z80CPU::push_af       , 0, 11), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD SP, HL", Z80CPU::ld_sp_hl      , 0, 6), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4)  /* f0 */
];

const IX_OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn ReadWrite) -> u8, u8, u8); 256] = [
/*         0                                      1                                         2                                      3                                      4                                      5                                       6                                      7                                      8                                      9                                      a                                      b                                      c                                      d                                      e                                      f                                          */
/* 00 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 00 */
/* 10 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 10 */
/* 20 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("LD IX", Z80CPU::ld_ix_nn      , 2, 14), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 20 */
/* 30 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 30 */
/* 40 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 40 */
/* 50 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 50 */
/* 60 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 60 */
/* 70 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 70 */
/* 80 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 80 */
/* 90 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 90 */
/* a0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* a0 */
/* b0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* b0 */
/* c0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* c0 */
/* d0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* d0 */
/* e0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* e0 */
/* f0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"  , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4)  /* f0 */
];

pub struct Z80CPU {
    a: u8, f: u8, a_alt: u8, f_alt: u8,
    b: u8, c: u8, b_alt: u8, c_alt: u8,
    d: u8, e: u8, d_alt: u8, e_alt: u8,
    h: u8, l: u8, h_alt: u8, l_alt: u8,
    i: u8, r: u8,
    ix: u16,
    iy: u16,
    sp: u16,
    pc: u16,
    t_cycles: u8
}

pub struct Z80CPUState {
    pub a: u8, pub f: u8, pub a_alt: u8, pub f_alt: u8,
    pub d: u8, pub e: u8, pub d_alt: u8, pub e_alt: u8,
    pub h: u8, pub l: u8, pub h_alt: u8, pub l_alt: u8,
    pub b: u8, pub c: u8, pub b_alt: u8, pub c_alt: u8,
    pub i: u8, pub r: u8,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16
}

impl Z80CPU {
    pub fn new() -> Z80CPU {
        Z80CPU {
            a: 0, f: 0, a_alt: 0, f_alt: 0,
            b: 0, c: 0, b_alt: 0, c_alt: 0,
            d: 0, e: 0, d_alt: 0, e_alt: 0,
            h: 0, l: 0, h_alt: 0, l_alt: 0,
            i: 0, r: 0,
            ix: 0,
            iy: 0,
            sp: 0,
            pc: 0,
            t_cycles: 0
        }
    }

    pub fn clock(&mut self, bus: &mut dyn ReadWrite) {
        if self.t_cycles == 0 {
            let opcode = bus.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            self.t_cycles = OPCODES[usize::from(opcode)].3;
            self.t_cycles += OPCODES[usize::from(opcode)].1(self, bus);
        }

        self.t_cycles -= 1;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.i = 0;
        self.r = 0;
        self.t_cycles = 3;
    }

    pub fn instruction_complete(&self) -> bool {
        self.t_cycles == 0
    }

    pub fn get_next_instructions(&self, bus: &dyn ReadWrite, nb: usize) -> Vec<String> {
        let mut instructions = vec![];
        let mut pc = self.pc;
        while instructions.len() < nb {
            let opcode = bus.read(pc);
            pc = pc.wrapping_add(1);
            let nb_operands = OPCODES[usize::from(opcode)].2;
            if nb_operands == 0 {
                if opcode == 0xdd {
                    let ix_opcode = bus.read(pc);
                    pc = pc.wrapping_add(1);
                    let nb_ix_operands = IX_OPCODES[usize::from(ix_opcode)].2;
                    if nb_ix_operands == 0 {
                        instructions.push(format!("{:04X}: {}", pc.wrapping_sub(2), IX_OPCODES[usize::from(ix_opcode)].0));
                    } else if nb_ix_operands == 1 {
                        let n = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        instructions.push(format!("{:04X}: {}, ${:X}", pc.wrapping_sub(3), IX_OPCODES[usize::from(ix_opcode)].0, n));
                    } else if nb_ix_operands == 2 {
                        let n_low = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        let n_high = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        instructions.push(format!("{:04X}: {}, ${:X}", pc.wrapping_sub(4), IX_OPCODES[usize::from(ix_opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)));
                    }
                } else {
                    instructions.push(format!("{:04X}: {}", pc.wrapping_sub(1), OPCODES[usize::from(opcode)].0));
                }
            } else if nb_operands == 1 {
                let n = bus.read(pc);
                pc = pc.wrapping_add(1);
                instructions.push(format!("{:04X}: {}, ${:X}", pc.wrapping_sub(2), OPCODES[usize::from(opcode)].0, n));
            } else if nb_operands == 2 {
                let n_low = bus.read(pc);
                pc = pc.wrapping_add(1);
                let n_high = bus.read(pc);
                pc = pc.wrapping_add(1);
                instructions.push(format!("{:04X}: {}, ${:X}", pc.wrapping_sub(3), OPCODES[usize::from(opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)));
            }
        }
        instructions
    }

    pub fn get_state(&self) -> Z80CPUState {
        Z80CPUState {
            a: self.a, f: self.f, a_alt: self.a_alt, f_alt: self.f_alt,
            b: self.b, c: self.c, b_alt: self.b_alt, c_alt: self.c_alt,
            d: self.d, e: self.e, d_alt: self.d_alt, e_alt: self.e_alt,
            h: self.h, l: self.l, h_alt: self.h_alt, l_alt: self.l_alt,
            i: self.i, r: self.r,
            ix: self.ix,
            iy: self.iy,
            sp: self.sp,
            pc: self.pc
        }
    }

    fn nop(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        0
    }

    fn ld_bc_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.c = n_low;
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.b = n_high;
        0
    }

    fn inc_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.b.wrapping_add(1);
        0
    }

    fn ld_b_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.b = n;
        0
    }

    fn inc_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.c.wrapping_add(1);
        0
    }

    fn ld_c_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.c = n;
        0
    }

    fn ld_de_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.e = n_low;
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.d = n_high;
        0
    }

    fn inc_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.d.wrapping_add(1);
        0
    }

    fn ld_d_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.d = n;
        0
    }

    fn inc_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.e.wrapping_add(1);
        0
    }

    fn ld_e_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.e = n;
        0
    }

    fn ld_hl_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.l = n_low;
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.h = n_high;
        0
    }

    fn inc_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.h.wrapping_add(1);
        0
    }

    fn ld_h_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.h = n;
        0
    }

    fn inc_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.l.wrapping_add(1);
        0
    }

    fn ld_l_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.l = n;
        0
    }

    fn ld_sp_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.sp = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }

    fn inc_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.a.wrapping_add(1);
        0
    }

    fn ld_a_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.a = n;
        0
    }

    fn ld_b_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.b;
        0
    }

    fn ld_b_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.c;
        0
    }

    fn ld_b_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.d;
        0
    }

    fn ld_b_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.e;
        0
    }

    fn ld_b_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.h;
        0
    }

    fn ld_b_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.l;
        0
    }

    fn ld_b_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.a;
        0
    }

    fn ld_c_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.b;
        0
    }

    fn ld_c_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.c;
        0
    }

    fn ld_c_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.d;
        0
    }

    fn ld_c_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.e;
        0
    }

    fn ld_c_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.h;
        0
    }

    fn ld_c_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.l;
        0
    }

    fn ld_c_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.a;
        0
    }

    fn ld_d_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.b;
        0
    }

    fn ld_d_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.c;
        0
    }

    fn ld_d_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.d;
        0
    }

    fn ld_d_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.e;
        0
    }

    fn ld_d_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.h;
        0
    }

    fn ld_d_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.l;
        0
    }

    fn ld_d_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.a;
        0
    }

    fn ld_e_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.b;
        0
    }

    fn ld_e_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.c;
        0
    }

    fn ld_e_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.d;
        0
    }

    fn ld_e_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.e;
        0
    }

    fn ld_e_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.h;
        0
    }

    fn ld_e_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.l;
        0
    }

    fn ld_e_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.a;
        0
    }

    fn ld_h_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.b;
        0
    }

    fn ld_h_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.c;
        0
    }

    fn ld_h_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.d;
        0
    }

    fn ld_h_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.e;
        0
    }

    fn ld_h_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.h;
        0
    }

    fn ld_h_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.l;
        0
    }

    fn ld_h_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.a;
        0
    }

    fn ld_l_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.b;
        0
    }

    fn ld_l_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.c;
        0
    }

    fn ld_l_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.d;
        0
    }

    fn ld_l_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.e;
        0
    }

    fn ld_l_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.h;
        0
    }

    fn ld_l_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.l;
        0
    }

    fn ld_l_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.a;
        0
    }

    fn ld_ptr_hl_b(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.b);
        0
    }

    fn ld_ptr_hl_c(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.c);
        0
    }

    fn ld_ptr_hl_d(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.d);
        0
    }

    fn ld_ptr_hl_e(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.e);
        0
    }

    fn ld_ptr_hl_h(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.h);
        0
    }

    fn ld_ptr_hl_l(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.l);
        0
    }

    fn ld_ptr_hl_a(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.a);
        0
    }

    fn ld_a_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.b;
        0
    }

    fn ld_a_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.c;
        0
    }

    fn ld_a_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.d;
        0
    }

    fn ld_a_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.e;
        0
    }

    fn ld_a_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.h;
        0
    }

    fn ld_a_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.l;
        0
    }

    fn ld_a_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.a;
        0
    }

    fn pop_bc(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.c = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.b = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }

    fn push_bc(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.b);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.c);
        0
    }

    fn pop_de(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.e = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.d = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }

    fn push_de(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.d);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.e);
        0
    }

    fn ix(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let ix_opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let mut t_cycles = IX_OPCODES[usize::from(ix_opcode)].3;
        t_cycles += IX_OPCODES[usize::from(ix_opcode)].1(self, bus);
        t_cycles
    }

    fn ld_ix_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.ix = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }

    fn pop_hl(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.l = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.h = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }

    fn push_hl(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.h);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.l);
        0
    }

    fn pop_af(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.f = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.a = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }

    fn push_af(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.a);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.f);
        0
    }

    fn ld_sp_hl(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.sp = hl;
        0
    }

    fn invalid_opcode(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    use super::*;
    use super::super::machine::MockReadWrite;

    #[test]
    fn test_nop() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x00);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: NOP");
    }

    #[test]
    fn test_ld_bc_0xbaad() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x01);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);

        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0xba);
        assert_eq!(cpu.c, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: LD BC, $BAAD");
    }

    #[test]
    fn test_inc_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x04);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC B");
    }

    #[test]
    fn test_ld_b_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x06);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD B, $D9");
    }

    #[test]
    fn test_inc_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC C");
    }

    #[test]
    fn test_ld_c_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD C, $D9");
    }

    #[test]
    fn test_ld_de_0xbaad() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x11);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0xba);
        assert_eq!(cpu.e, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: LD DE, $BAAD");
    }

    #[test]
    fn test_inc_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x14);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC D");
    }

    #[test]
    fn test_ld_d_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x16);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD D, $D9");
    }

    #[test]
    fn test_inc_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC E");
    }

    #[test]
    fn test_ld_e_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD E, $D9");
    }

    #[test]
    fn test_ld_hl_0x4001() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x21);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x01);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x40);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x40);
        assert_eq!(cpu.l, 0x01);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: LD HL, $4001");
    }

    #[test]
    fn test_inc_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x24);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC H");
    }

    #[test]
    fn test_ld_h_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x26);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD H, $D9");
    }

    #[test]
    fn test_inc_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC L");
    }

    #[test]
    fn test_ld_l_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD L, $D9");
    }

    #[test]
    fn test_ld_sp_0xbaad() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x31);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.sp, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: LD SP, $BAAD");
    }

    #[test]
    fn test_inc_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfb;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0xfc);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC A");
    }

    #[test]
    fn test_ld_a_0x2a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x2a);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD A, $2A");
    }

    #[test]
    fn test_ld_b_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x40);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, B");
    }

    #[test]
    fn test_ld_b_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x41);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, C");
    }

    #[test]
    fn test_ld_b_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x42);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, D");
    }

    #[test]
    fn test_ld_b_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x43);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, E");
    }

    #[test]
    fn test_ld_b_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x44);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, H");
    }

    #[test]
    fn test_ld_b_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x45);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, L");
    }

    #[test]
    fn test_ld_b_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x47);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD B, A");
    }

    #[test]
    fn test_ld_c_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x48);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, B");
    }

    #[test]
    fn test_ld_c_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x49);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, C");
    }

    #[test]
    fn test_ld_c_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4a);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, D");
    }

    #[test]
    fn test_ld_c_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4b);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, E");
    }

    #[test]
    fn test_ld_c_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, H");
    }

    #[test]
    fn test_ld_c_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4d);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, L");
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4f);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD C, A");
    }

    #[test]
    fn test_ld_d_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x50);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, B");
    }

    #[test]
    fn test_ld_d_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x51);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, C");
    }

    #[test]
    fn test_ld_d_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x52);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, D");
    }

    #[test]
    fn test_ld_d_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x53);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, E");
    }

    #[test]
    fn test_ld_d_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x54);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, H");
    }

    #[test]
    fn test_ld_d_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x55);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, L");
    }

    #[test]
    fn test_ld_d_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x57);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD D, A");
    }

    #[test]
    fn test_ld_e_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x58);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, B");
    }

    #[test]
    fn test_ld_e_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x59);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, C");
    }

    #[test]
    fn test_ld_e_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5a);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, D");
    }

    #[test]
    fn test_ld_e_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5b);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, E");
    }

    #[test]
    fn test_ld_e_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, H");
    }

    #[test]
    fn test_ld_e_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5d);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, L");
    }

    #[test]
    fn test_ld_e_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5f);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD E, A");
    }

    #[test]
    fn test_ld_h_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x60);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, B");
    }

    #[test]
    fn test_ld_h_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x61);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, C");
    }

    #[test]
    fn test_ld_h_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x62);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, D");
    }

    #[test]
    fn test_ld_h_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x63);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, E");
    }

    #[test]
    fn test_ld_h_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x64);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, H");
    }

    #[test]
    fn test_ld_h_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x65);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, L");
    }

    #[test]
    fn test_ld_h_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x67);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD H, A");
    }

    #[test]
    fn test_ld_l_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x68);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, B");
    }

    #[test]
    fn test_ld_l_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x69);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, C");
    }

    #[test]
    fn test_ld_l_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6a);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, D");
    }

    #[test]
    fn test_ld_l_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6b);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, E");
    }

    #[test]
    fn test_ld_l_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, H");
    }

    #[test]
    fn test_ld_l_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6d);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, L");
    }

    #[test]
    fn test_ld_l_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6f);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD L, A");
    }

    #[test]
    fn test_ld_ptr_hl_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x70);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.b = 0x2a;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), B");
    }

    #[test]
    fn test_ld_ptr_hl_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x71);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.c = 0x2a;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), C");
    }

    #[test]
    fn test_ld_ptr_hl_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x72);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.d = 0x2a;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), D");
    }

    #[test]
    fn test_ld_ptr_hl_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x73);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.e = 0x2a;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), E");
    }

    #[test]
    fn test_ld_ptr_hl_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x74);
        mock_bus.expect_write().with(eq(0x4001), eq(0x40)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), H");
    }

    #[test]
    fn test_ld_ptr_hl_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x75);
        mock_bus.expect_write().with(eq(0x4001), eq(0x01)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), L");
    }

    #[test]
    fn test_ld_ptr_hl_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x77);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.a = 0x2a;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
		assert_eq!(disasm, "0000: LD (HL), A");
    }

    #[test]
    fn test_ld_a_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x78);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, B");
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x79);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, C");
    }

    #[test]
    fn test_ld_a_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7a);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, D");
    }

    #[test]
    fn test_ld_a_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7b);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, E");
    }

    #[test]
    fn test_ld_a_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7c);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, H");
    }

    #[test]
    fn test_ld_a_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7d);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, L");
    }

    #[test]
    fn test_ld_a_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7f);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
		assert_eq!(disasm, "0000: LD A, A");
    }

    #[test]
    fn test_pop_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xc1);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0xba);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0xba);
        assert_eq!(cpu.c, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: POP BC");
    }

    #[test]
    fn test_push_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xc5);
        mock_bus.expect_write().with(eq(0x4fff), eq(0xba)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0xad)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0xba;
        cpu.c = 0xad;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 11);
		assert_eq!(disasm, "0000: PUSH BC");
    }

    #[test]
    fn test_pop_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd1);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0xba);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0xba);
        assert_eq!(cpu.e, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: POP DE");
    }


    #[test]
    fn test_push_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd5);
        mock_bus.expect_write().with(eq(0x4fff), eq(0xba)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0xad)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0xba;
        cpu.e = 0xad;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 11);
		assert_eq!(disasm, "0000: PUSH DE");
    }

    #[test]
    fn test_ld_ix_0xbaad() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xdd);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x21);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(3)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.ix, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 14);
        assert_eq!(disasm, "0000: LD IX, $BAAD");
    }

    #[test]
    fn test_pop_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xe1);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0xba);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0xba);
        assert_eq!(cpu.l, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: POP HL");
    }

    #[test]
    fn test_push_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xe5);
        mock_bus.expect_write().with(eq(0x4fff), eq(0xba)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0xad)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xba;
        cpu.l = 0xad;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 11);
		assert_eq!(disasm, "0000: PUSH HL");
    }

    #[test]
    fn test_pop_af() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf1);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0xba);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0xba);
        assert_eq!(cpu.f, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
		assert_eq!(disasm, "0000: POP AF");
    }


    #[test]
    fn test_push_af() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf5);
        mock_bus.expect_write().with(eq(0x4fff), eq(0xba)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0xad)).return_const(());

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xba;
        cpu.f = 0xad;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 11);
		assert_eq!(disasm, "0000: PUSH AF");
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf9);

        cpu.reset();
		let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0xff;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.sp, 0x40ff);
        assert_eq!(1 + cpu.t_cycles, 6);
		assert_eq!(disasm, "0000: LD SP, HL");
    }
}