use super::machine::Bus;

const OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn Bus) -> u8, u8, u8); 256] = [
/*         0                                          1                                           2                                             3                                               4                                              5                                              6                                               7                                              8                                              9                                              a                                               b                                            c                                            d                                            e                                             f                                                */
/* 00 */ ("NOP"       , Z80CPU::nop        , 0, 4), ("LD BC,"    , Z80CPU::ld_bc_nn   , 2, 10), ("LD (BC), A" , Z80CPU::ld_ptr_bc_a , 0,  7), ("INC BC"     , Z80CPU::inc_bc        , 0,  6), ("INC B"     , Z80CPU::inc_b         , 0,  4), ("DEC B"     , Z80CPU::dec_b         , 0,  4), ("LD B,"      , Z80CPU::ld_b_n        , 1,  7), ("RLCA"      , Z80CPU::rlca          , 0,  4), ("EX AF, AF'", Z80CPU::ex_af_af_alt  , 0,  4), ("ADD HL, BC", Z80CPU::add_hl_bc     , 0, 11), ("LD A, (BC)" , Z80CPU::ld_a_ptr_bc   , 0,  7), ("DEC BC"   , Z80CPU::dec_bc        , 0, 6), ("INC C"   , Z80CPU::inc_c         , 0,  4), ("DEC C"   , Z80CPU::dec_c         , 0,  4), ("LD C,"     , Z80CPU::ld_c_n        , 1, 7), ("RRCA"    , Z80CPU::rrca          , 0,  4), /* 00 */
/* 10 */ ("DJNZ $"    , Z80CPU::djnz_e     , 1, 8), ("LD DE,"    , Z80CPU::ld_de_nn   , 2, 10), ("LD (DE), A" , Z80CPU::ld_ptr_de_a , 0,  7), ("INC DE"     , Z80CPU::inc_de        , 0,  6), ("INC D"     , Z80CPU::inc_d         , 0,  4), ("DEC D"     , Z80CPU::dec_d         , 0,  4), ("LD D,"      , Z80CPU::ld_d_n        , 1,  7), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("JR $"      , Z80CPU::jr_e          , 1, 12), ("ADD HL, DE", Z80CPU::add_hl_de     , 0, 11), ("LD A, (DE)" , Z80CPU::ld_a_ptr_de   , 0,  7), ("DEC DE"   , Z80CPU::dec_de        , 0, 6), ("INC E"   , Z80CPU::inc_e         , 0,  4), ("DEC E"   , Z80CPU::dec_e         , 0,  4), ("LD E,"     , Z80CPU::ld_e_n        , 1, 7), ("???"     , Z80CPU::invalid_opcode, 0,  4), /* 10 */
/* 20 */ ("JR NZ, $"  , Z80CPU::jr_nz_e    , 1, 7), ("LD HL,"    , Z80CPU::ld_hl_nn   , 2, 10), ("LD (nn), HL", Z80CPU::ld_ptr_nn_hl, 2, 16), ("INC HL"     , Z80CPU::inc_hl        , 0,  6), ("INC H"     , Z80CPU::inc_h         , 0,  4), ("DEC H"     , Z80CPU::dec_h         , 0,  4), ("LD H,"      , Z80CPU::ld_h_n        , 1,  7), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("JR Z, $"   , Z80CPU::jr_z_e        , 1,  7), ("ADD HL, HL", Z80CPU::add_hl_hl     , 0, 11), ("LD HL, (nn)", Z80CPU::ld_hl_ptr_nn  , 2, 16), ("DEC HL"   , Z80CPU::dec_hl        , 0, 6), ("INC L"   , Z80CPU::inc_l         , 0,  4), ("DEC L"   , Z80CPU::dec_l         , 0,  4), ("LD L,"     , Z80CPU::ld_l_n        , 1, 7), ("???"     , Z80CPU::invalid_opcode, 0,  4), /* 20 */
/* 30 */ ("JR NC, $"  , Z80CPU::jr_nc_e    , 1, 7), ("LD SP,"    , Z80CPU::ld_sp_nn   , 2, 10), ("LD (nn), A" , Z80CPU::ld_ptr_nn_a , 2, 13), ("INC SP"     , Z80CPU::inc_sp        , 0,  6), ("INC (HL)"  , Z80CPU::inc_ptr_hl    , 0, 11), ("DEC (HL)"  , Z80CPU::dec_ptr_hl    , 0, 11), ("LD (HL),"   , Z80CPU::ld_ptr_hl_n   , 1, 10), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("JR C, $"   , Z80CPU::jr_c_e        , 1,  7), ("ADD HL, SP", Z80CPU::add_hl_sp     , 0, 11), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("DEC SP"   , Z80CPU::dec_sp        , 0, 6), ("INC A"   , Z80CPU::inc_a         , 0,  4), ("DEC A"   , Z80CPU::dec_a         , 0,  4), ("LD A,"     , Z80CPU::ld_a_n        , 1, 7), ("???"     , Z80CPU::invalid_opcode, 0,  4), /* 30 */
/* 40 */ ("LD B, B"   , Z80CPU::ld_b_b     , 0, 4), ("LD B, C"   , Z80CPU::ld_b_c     , 0,  4), ("LD B, D"    , Z80CPU::ld_b_d      , 0,  4), ("LD B, E"    , Z80CPU::ld_b_e        , 0,  4), ("LD B, H"   , Z80CPU::ld_b_h        , 0,  4), ("LD B, L"   , Z80CPU::ld_b_l        , 0,  4), ("LD B, (HL)" , Z80CPU::ld_b_ptr_hl   , 0,  7), ("LD B, A"   , Z80CPU::ld_b_a        , 0,  4), ("LD C, B"   , Z80CPU::ld_c_b        , 0,  4), ("LD C, C"   , Z80CPU::ld_c_c        , 0,  4), ("LD C, D"    , Z80CPU::ld_c_d        , 0,  4), ("LD C, E"  , Z80CPU::ld_c_e        , 0, 4), ("LD C, H" , Z80CPU::ld_c_h        , 0,  4), ("LD C, L" , Z80CPU::ld_c_l        , 0,  4), ("LD C, (HL)", Z80CPU::ld_c_ptr_hl   , 0, 7), ("LD C, A" , Z80CPU::ld_c_a        , 0,  4), /* 40 */
/* 50 */ ("LD D, B"   , Z80CPU::ld_d_b     , 0, 4), ("LD D, C"   , Z80CPU::ld_d_c     , 0,  4), ("LD D, D"    , Z80CPU::ld_d_d      , 0,  4), ("LD D, E"    , Z80CPU::ld_d_e        , 0,  4), ("LD D, H"   , Z80CPU::ld_d_h        , 0,  4), ("LD D, L"   , Z80CPU::ld_d_l        , 0,  4), ("LD D, (HL)" , Z80CPU::ld_d_ptr_hl   , 0,  7), ("LD D, A"   , Z80CPU::ld_d_a        , 0,  4), ("LD E, B"   , Z80CPU::ld_e_b        , 0,  4), ("LD E, C"   , Z80CPU::ld_e_c        , 0,  4), ("LD E, D"    , Z80CPU::ld_e_d        , 0,  4), ("LD E, E"  , Z80CPU::ld_e_e        , 0, 4), ("LD E, H" , Z80CPU::ld_e_h        , 0,  4), ("LD E, L" , Z80CPU::ld_e_l        , 0,  4), ("LD E, (HL)", Z80CPU::ld_e_ptr_hl   , 0, 7), ("LD E, A" , Z80CPU::ld_e_a        , 0,  4), /* 50 */
/* 60 */ ("LD H, B"   , Z80CPU::ld_h_b     , 0, 4), ("LD H, C"   , Z80CPU::ld_h_c     , 0,  4), ("LD H, D"    , Z80CPU::ld_h_d      , 0,  4), ("LD H, E"    , Z80CPU::ld_h_e        , 0,  4), ("LD H, H"   , Z80CPU::ld_h_h        , 0,  4), ("LD H, L"   , Z80CPU::ld_h_l        , 0,  4), ("LD H, (HL)" , Z80CPU::ld_h_ptr_hl   , 0,  7), ("LD H, A"   , Z80CPU::ld_h_a        , 0,  4), ("LD L, B"   , Z80CPU::ld_l_b        , 0,  4), ("LD L, C"   , Z80CPU::ld_l_c        , 0,  4), ("LD L, D"    , Z80CPU::ld_l_d        , 0,  4), ("LD L, E"  , Z80CPU::ld_l_e        , 0, 4), ("LD L, H" , Z80CPU::ld_l_h        , 0,  4), ("LD L, L" , Z80CPU::ld_l_l        , 0,  4), ("LD L, (HL)", Z80CPU::ld_l_ptr_hl   , 0, 7), ("LD L, A" , Z80CPU::ld_l_a        , 0,  4), /* 60 */
/* 70 */ ("LD (HL), B", Z80CPU::ld_ptr_hl_b, 0, 7), ("LD (HL), C", Z80CPU::ld_ptr_hl_c, 0,  7), ("LD (HL), D" , Z80CPU::ld_ptr_hl_d , 0,  7), ("LD (HL), E" , Z80CPU::ld_ptr_hl_e   , 0,  7), ("LD (HL), H", Z80CPU::ld_ptr_hl_h   , 0,  7), ("LD (HL), L", Z80CPU::ld_ptr_hl_l   , 0,  7), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("LD (HL), A", Z80CPU::ld_ptr_hl_a   , 0,  7), ("LD A, B"   , Z80CPU::ld_a_b        , 0,  4), ("LD A, C"   , Z80CPU::ld_a_c        , 0,  4), ("LD A, D"    , Z80CPU::ld_a_d        , 0,  4), ("LD A, E"  , Z80CPU::ld_a_e        , 0, 4), ("LD A, H" , Z80CPU::ld_a_h        , 0,  4), ("LD A, L" , Z80CPU::ld_a_l        , 0,  4), ("LD A, (HL)", Z80CPU::ld_a_ptr_hl   , 0, 7), ("LD A, A" , Z80CPU::ld_a_a        , 0,  4), /* 70 */
/* 80 */ ("ADD A, B"  , Z80CPU::add_a_b    , 0, 4), ("ADD A, C"  , Z80CPU::add_a_c    , 0,  4), ("ADD A, D"   , Z80CPU::add_a_d     , 0,  4), ("ADD A, E"   , Z80CPU::add_a_e       , 0,  4), ("ADD A, H"  , Z80CPU::add_a_h       , 0,  4), ("ADD A, L"  , Z80CPU::add_a_l       , 0,  4), ("ADD A, (HL)", Z80CPU::add_a_ptr_hl  , 0,  7), ("ADD A, A"  , Z80CPU::add_a_a       , 0,  4), ("ADC A, B"  , Z80CPU::adc_a_b       , 0,  4), ("ADC A, C"  , Z80CPU::adc_a_c       , 0,  4), ("ADC A, D"   , Z80CPU::adc_a_d       , 0,  4), ("ADC A, E" , Z80CPU::adc_a_e       , 0, 4), ("ADC A, H", Z80CPU::adc_a_h       , 0,  4), ("ADC A, L", Z80CPU::adc_a_l       , 0,  4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("ADC A, A", Z80CPU::adc_a_a       , 0,  4), /* 80 */
/* 90 */ ("SUB B"     , Z80CPU::sub_b      , 0, 4), ("SUB C"     , Z80CPU::sub_c      , 0,  4), ("SUB D"      , Z80CPU::sub_d       , 0,  4), ("SUB E"      , Z80CPU::sub_e         , 0,  4), ("SUB H"     , Z80CPU::sub_h         , 0,  4), ("SUB L"     , Z80CPU::sub_l         , 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("SUB A"     , Z80CPU::sub_a         , 0,  4), ("SBC A, B"  , Z80CPU::sbc_a_b       , 0,  4), ("SBC A, C"  , Z80CPU::sbc_a_c       , 0,  4), ("SBC A, D"   , Z80CPU::sbc_a_d       , 0,  4), ("SBC A, E" , Z80CPU::sbc_a_e       , 0, 4), ("SBC A, H", Z80CPU::sbc_a_h       , 0,  4), ("SBC A, L", Z80CPU::sbc_a_l       , 0,  4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("SBC A, A", Z80CPU::sbc_a_a       , 0,  4), /* 90 */
/* a0 */ ("AND B"     , Z80CPU::and_b      , 0, 4), ("AND C"     , Z80CPU::and_c      , 0,  4), ("AND D"      , Z80CPU::and_d       , 0,  4), ("AND E"      , Z80CPU::and_e         , 0,  4), ("AND H"     , Z80CPU::and_h         , 0,  4), ("AND L"     , Z80CPU::and_l         , 0,  4), ("AND (HL)"   , Z80CPU::and_ptr_hl    , 0,  7), ("AND A"     , Z80CPU::and_a         , 0,  4), ("XOR B"     , Z80CPU::xor_b         , 0,  4), ("XOR C"     , Z80CPU::xor_c         , 0,  4), ("XOR D"      , Z80CPU::xor_d         , 0,  4), ("XOR E"    , Z80CPU::xor_e         , 0, 4), ("XOR H"   , Z80CPU::xor_h         , 0,  4), ("XOR L"   , Z80CPU::xor_l         , 0,  4), ("XOR (HL)"  , Z80CPU::xor_ptr_hl    , 0, 7), ("XOR A"   , Z80CPU::xor_a         , 0,  4), /* a0 */
/* b0 */ ("OR B"      , Z80CPU::or_b       , 0, 4), ("OR C"      , Z80CPU::or_c       , 0,  4), ("OR D"       , Z80CPU::or_d        , 0,  4), ("OR E"       , Z80CPU::or_e          , 0,  4), ("OR H"      , Z80CPU::or_h          , 0,  4), ("OR L"      , Z80CPU::or_l          , 0,  4), ("OR (HL)"    , Z80CPU::or_ptr_hl     , 0,  7), ("OR A"      , Z80CPU::or_a          , 0,  4), ("CP B"      , Z80CPU::cp_b          , 0,  4), ("CP C"      , Z80CPU::cp_c          , 0,  4), ("CP D"       , Z80CPU::cp_d          , 0,  4), ("CP E"     , Z80CPU::cp_e          , 0, 4), ("CP H"    , Z80CPU::cp_h          , 0,  4), ("CP L"    , Z80CPU::cp_l          , 0,  4), ("CP (HL)"   , Z80CPU::cp_ptr_hl     , 0, 7), ("CP A"    , Z80CPU::cp_a          , 0,  4), /* b0 */
/* c0 */ ("RET NZ"    , Z80CPU::ret_nz     , 0, 5), ("POP BC"    , Z80CPU::pop_bc     , 0, 10), ("JP NZ,"     , Z80CPU::jp_nz_nn    , 2, 10), ("JP"         , Z80CPU::jp_nn         , 2, 10), ("CALL NZ,"  , Z80CPU::call_nz_nn    , 2, 10), ("PUSH BC"   , Z80CPU::push_bc       , 0, 11), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("RST 00h"   , Z80CPU::rst_00h       , 0, 11), ("RET Z"     , Z80CPU::ret_z         , 0,  5), ("RET"       , Z80CPU::ret           , 0, 10), ("JP Z,"      , Z80CPU::jp_z_nn       , 2, 10), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("CALL Z," , Z80CPU::call_z_nn     , 2, 10), ("CALL"    , Z80CPU::call_nn       , 2, 17), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("RST 08h" , Z80CPU::rst_08h       , 0, 11), /* c0 */
/* d0 */ ("RET NC"    , Z80CPU::ret_nc     , 0, 5), ("POP DE"    , Z80CPU::pop_de     , 0, 10), ("JP NC,"     , Z80CPU::jp_nc_nn    , 2, 10), ("OUT (n), A" , Z80CPU::out_ptr_n_a   , 1, 11), ("CALL NC,"  , Z80CPU::call_nc_nn    , 2, 10), ("PUSH DE"   , Z80CPU::push_de       , 0, 11), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("RST 10h"   , Z80CPU::rst_10h       , 0, 11), ("RET C"     , Z80CPU::ret_c         , 0,  5), ("EXX"       , Z80CPU::exx           , 0,  4), ("JP C,"      , Z80CPU::jp_c_nn       , 2, 10), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("CALL C," , Z80CPU::call_c_nn     , 2, 10), ("IX"      , Z80CPU::ix            , 0,  0), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("RST 18h" , Z80CPU::rst_18h       , 0, 11), /* d0 */
/* e0 */ ("RET PO"    , Z80CPU::ret_po     , 0, 5), ("POP HL"    , Z80CPU::pop_hl     , 0, 10), ("JP PO,"     , Z80CPU::jp_po_nn    , 2, 10), ("EX (SP), HL", Z80CPU::ex_ptr_sp_hl  , 0, 19), ("CALL PO,"  , Z80CPU::call_po_nn    , 2, 10), ("PUSH HL"   , Z80CPU::push_hl       , 0, 11), ("AND"        , Z80CPU::and_n         , 1,  7), ("RST 20h"   , Z80CPU::rst_20h       , 0, 11), ("RET PE"    , Z80CPU::ret_pe        , 0,  5), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("JP PE,"     , Z80CPU::jp_pe_nn      , 2, 10), ("EX DE, HL", Z80CPU::ex_de_hl      , 0, 4), ("CALL PE,", Z80CPU::call_pe_nn    , 2, 10), ("EXTENDED", Z80CPU::extended      , 0,  0), ("XOR"       , Z80CPU::xor_n         , 1, 7), ("RST 28h" , Z80CPU::rst_28h       , 0, 11), /* e0 */
/* f0 */ ("RET P"     , Z80CPU::ret_p      , 0, 5), ("POP AF"    , Z80CPU::pop_af     , 0, 10), ("JP P,"      , Z80CPU::jp_p_nn     , 2, 10), ("DI"         , Z80CPU::di            , 0,  4), ("CALL P,"   , Z80CPU::call_p_nn     , 2, 10), ("PUSH AF"   , Z80CPU::push_af       , 0, 11), ("OR"         , Z80CPU::or_n          , 1,  7), ("RST 30h"   , Z80CPU::rst_30h       , 0, 11), ("RET M"     , Z80CPU::ret_m         , 0,  5), ("LD SP, HL" , Z80CPU::ld_sp_hl      , 0,  6), ("JP M,"      , Z80CPU::jp_m_nn       , 2, 10), ("EI"       , Z80CPU::ei            , 0, 4), ("CALL M," , Z80CPU::call_m_nn     , 2, 10), ("IY"      , Z80CPU::iy            , 0,  0), ("CP"        , Z80CPU::cp_n          , 1, 7), ("RST 38h" , Z80CPU::rst_38h       , 0, 11)  /* f0 */
];

const IX_OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn Bus) -> u8, u8, u8); 256] = [
/*         0                                      1                                          2                                      3                                      4                                      5                                       6                                      7                                      8                                      9                                      a                                      b                                      c                                      d                                      e                                      f                                          */
/* 00 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 00 */
/* 10 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 10 */
/* 20 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("LD IX,", Z80CPU::ld_ix_nn      , 2, 14), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 20 */
/* 30 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 30 */
/* 40 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 40 */
/* 50 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 50 */
/* 60 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 60 */
/* 70 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 70 */
/* 80 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 80 */
/* 90 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 90 */
/* a0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* a0 */
/* b0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* b0 */
/* c0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* c0 */
/* d0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* d0 */
/* e0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* e0 */
/* f0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4)  /* f0 */
];

const EXTENDED_OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn Bus) -> u8, u8, u8); 256] = [
/*         0                                        1                                      2                                              3                                               4                                      5                                       6                                       7                                          8                                        9                                      a                                      b                                      c                                      d                                      e                                       f                                          */
/* 00 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 00 */
/* 10 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 10 */
/* 20 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 20 */
/* 30 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 30 */
/* 40 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("LD (nn), BC", Z80CPU::ld_ptr_nn_bc  , 2, 20), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("IM 0", Z80CPU::im_0          , 0, 8), ("LD I, A", Z80CPU::ld_i_a        , 0, 9), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 40 */
/* 50 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("SBC HL, DE", Z80CPU::sbc_hl_de     , 0, 15), ("LD (nn), DE", Z80CPU::ld_ptr_nn_de  , 2, 20), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("IM 1", Z80CPU::im_1          , 0, 8), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("IM 2", Z80CPU::im_2,           0, 8), ("???", Z80CPU::invalid_opcode, 0, 4), /* 50 */
/* 60 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 60 */
/* 70 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 70 */
/* 80 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 80 */
/* 90 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 90 */
/* a0 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* a0 */
/* b0 */ ("LDIR", Z80CPU::ldir,           0, 16), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LDDR", Z80CPU::lddr          , 0, 16), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* b0 */
/* c0 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* c0 */
/* d0 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* d0 */
/* e0 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* e0 */
/* f0 */ ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???"        , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4)  /* f0 */
]; 

const IY_OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn Bus) -> u8, u8, u8); 256] = [
/*         0                                      1                                          2                                      3                                      4                                      5                                              6                                      7                                      8                                      9                                      a                                      b                                                 c                                      d                                      e                                      f                                          */
/* 00 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 00 */
/* 10 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 10 */
/* 20 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("LD IY,", Z80CPU::ld_iy_nn      , 2, 14), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 20 */
/* 30 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("DEC (IY+d)", Z80CPU::dec_ptr_iy_d  , 1, 23), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 30 */
/* 40 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 40 */
/* 50 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 50 */
/* 60 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 60 */
/* 70 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 70 */
/* 80 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 80 */
/* 90 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* 90 */
/* a0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* a0 */
/* b0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* b0 */
/* c0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("SET b, (IY+d)", Z80CPU::set_b_ptr_iy_d, 2, 23), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* c0 */
/* d0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* d0 */
/* e0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), /* e0 */
/* f0 */ ("???", Z80CPU::invalid_opcode, 0, 4), ("???"   , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???"          , Z80CPU::invalid_opcode, 0,  4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4)  /* f0 */
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
    t_cycles: u8,
    interrupts_enabled: bool,
    interrupt_mode: u8
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
    pub pc: u16,
    pub interrupts_enabled: bool,
    pub interrupt_mode: u8
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
            t_cycles: 0,
            interrupts_enabled: true,
            interrupt_mode: 0
        }
    }
    
    pub fn clock(&mut self, bus: &mut dyn Bus) {
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
    
    pub fn get_next_instructions(&self, bus: &dyn Bus, nb: usize) -> Vec<String> {
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
                        if IX_OPCODES[usize::from(ix_opcode)].0 == "???" {
                            instructions.push(format!("{:04X}: {} (DD {:02X})", pc.wrapping_sub(2), IX_OPCODES[usize::from(ix_opcode)].0, ix_opcode));
                        } else {
                            instructions.push(format!("{:04X}: {}", pc.wrapping_sub(2), IX_OPCODES[usize::from(ix_opcode)].0));
                        }
                    } else if nb_ix_operands == 1 {
                        let n = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        instructions.push(format!("{:04X}: {} {:02X}h", pc.wrapping_sub(3), IX_OPCODES[usize::from(ix_opcode)].0, n));
                    } else if nb_ix_operands == 2 {
                        let n_low = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        let n_high = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        instructions.push(format!("{:04X}: {} {:04X}h", pc.wrapping_sub(4), IX_OPCODES[usize::from(ix_opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)));
                    }
                } else if opcode == 0xed {
                    let extended_opcode = bus.read(pc);
                    pc = pc.wrapping_add(1);
                    let nb_extended_operands = EXTENDED_OPCODES[usize::from(extended_opcode)].2;
                    if nb_extended_operands == 0 {
                        if EXTENDED_OPCODES[usize::from(extended_opcode)].0 == "???" {
                            instructions.push(format!("{:04X}: {} (ED {:02X})", pc.wrapping_sub(2), EXTENDED_OPCODES[usize::from(extended_opcode)].0, extended_opcode));
                        } else {
                            instructions.push(format!("{:04X}: {}", pc.wrapping_sub(2), EXTENDED_OPCODES[usize::from(extended_opcode)].0));
                        }
                    } else if nb_extended_operands == 1 {
                        let n = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        instructions.push(format!("{:04X}: {} {:02X}h", pc.wrapping_sub(3), EXTENDED_OPCODES[usize::from(extended_opcode)].0, n));
                    } else if nb_extended_operands == 2 {
                        let n_low = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        let n_high = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        match extended_opcode {
                            0x43 | 0x53 => {
                                let pass1 = format!("{:04X}: {}", pc.wrapping_sub(4), EXTENDED_OPCODES[usize::from(extended_opcode)].0);
                                let nn = format!("{:04X}h", (u16::from(n_high) << 8) + u16::from(n_low));
                                instructions.push(pass1.replace("nn", &nn))
                            },
                            _ => instructions.push(format!("{:04X}: {} {:04X}h", pc.wrapping_sub(4), EXTENDED_OPCODES[usize::from(extended_opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)))
                        }
                    }
                } else if opcode == 0xfd {
                    let iy_opcode = bus.read(pc);
                    pc = pc.wrapping_add(1);
                    let nb_iy_operands = IY_OPCODES[usize::from(iy_opcode)].2;
                    if nb_iy_operands == 0 {
                        if IY_OPCODES[usize::from(iy_opcode)].0 == "???" {
                            instructions.push(format!("{:04X}: {} (FD {:02X})", pc.wrapping_sub(2), IY_OPCODES[usize::from(iy_opcode)].0, iy_opcode));
                        } else {
                            instructions.push(format!("{:04X}: {}", pc.wrapping_sub(2), IY_OPCODES[usize::from(iy_opcode)].0));
                        }
                    } else if nb_iy_operands == 1 {
                        let n = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        match iy_opcode {
                            0x35 => {
                                let pass1 = format!("{:04X}: {}", pc.wrapping_sub(3), IY_OPCODES[usize::from(iy_opcode)].0);
                                let d = format!("{:+}", n as i8);
                                instructions.push(pass1.replace("+d", &d))
                            },
                            _ => instructions.push(format!("{:04X}: {} {:02X}h", pc.wrapping_sub(3), IY_OPCODES[usize::from(iy_opcode)].0, n))
                        }
                    } else if nb_iy_operands == 2 {
                        let n_low = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        let n_high = bus.read(pc);
                        pc = pc.wrapping_add(1);
                        match iy_opcode {
                            0xcb => {
                                let pass1 = format!("{:04X}: {}", pc.wrapping_sub(4), IY_OPCODES[usize::from(iy_opcode)].0);
                                let d = format!("{:+}", n_low as i8);
                                let b = format!("{}", (n_high & 0b00111000) >> 3);
                                instructions.push(pass1.replace("b", &b).replace("+d", &d))
                            },
                            _ => instructions.push(format!("{:04X}: {} {:04X}h", pc.wrapping_sub(4), IY_OPCODES[usize::from(iy_opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)))
                        }
                    }
                } else {
                    if OPCODES[usize::from(opcode)].0 == "???" {
                        instructions.push(format!("{:04X}: {} ({:02X})", pc.wrapping_sub(1), OPCODES[usize::from(opcode)].0, opcode));
                    } else {
                        instructions.push(format!("{:04X}: {}", pc.wrapping_sub(1), OPCODES[usize::from(opcode)].0));
                    }
                }
            } else if nb_operands == 1 {
                let n = bus.read(pc);
                pc = pc.wrapping_add(1);
                match opcode {
                    0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => instructions.push(format!("{:04X}: {}{:+}", pc.wrapping_sub(2), OPCODES[usize::from(opcode)].0, (n + 2) as i8)),
                    0xd3 => {
                        let pass1 = format!("{:04X}: {}", pc.wrapping_sub(2), OPCODES[usize::from(opcode)].0);
                        let n = format!("{:02X}h", n);
                        instructions.push(pass1.replace("n", &n))
                    },
                    _ => instructions.push(format!("{:04X}: {} {:02X}h", pc.wrapping_sub(2), OPCODES[usize::from(opcode)].0, n))
                }
            } else if nb_operands == 2 {
                let n_low = bus.read(pc);
                pc = pc.wrapping_add(1);
                let n_high = bus.read(pc);
                pc = pc.wrapping_add(1);
                match opcode {
                    0x22 | 0x2a | 0x32 => {
                        let pass1 = format!("{:04X}: {}", pc.wrapping_sub(3), OPCODES[usize::from(opcode)].0);
                        let nn = format!("{:04X}h", (u16::from(n_high) << 8) + u16::from(n_low));
                        instructions.push(pass1.replace("nn", &nn))
                    },
                    _ => instructions.push(format!("{:04X}: {} {:04X}h", pc.wrapping_sub(3), OPCODES[usize::from(opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)))
                }
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
            pc: self.pc,
            interrupts_enabled: self.interrupts_enabled,
            interrupt_mode: self.interrupt_mode
        }
    }

    fn nop(&mut self, _bus: &mut dyn Bus) -> u8 {
        0
    }
    
    fn ld_bc_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.c = n_low;
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.b = n_high;
        0
    }
    
    fn ld_ptr_bc_a(&mut self, bus: &mut dyn Bus) -> u8 {
        let bc = (u16::from(self.b) << 8) + u16::from(self.c);
        bus.write(bc, self.a);
        0
    }
    
    fn inc_bc(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.c.wrapping_add(1);
        if self.c == 0 {
            self.b = self.b.wrapping_add(1);
        }
        0
    }
    
    fn inc_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.b.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.b & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.b > 0x7f) != (res > 0x7f);
        self.b = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.b.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.b & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.b > 0x7f) != (res > 0x7f);
        self.b = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_b_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.b = n;
        0
    }
    
    fn rlca(&mut self, _bus: &mut dyn Bus) -> u8 {
        if self.a & 0b10000000 == 0b10000000 {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.a = self.a.rotate_left(1);
        self.f &= 0b11101101;
        0
    }
    
    fn ex_af_af_alt(&mut self, _bus: &mut dyn Bus) -> u8 {
        let temp_a = self.a;
        self.a = self.a_alt;
        self.a_alt = temp_a;
        let temp_f = self.f;
        self.f = self.f_alt;
        self.f_alt = temp_f;
        0
    }
    
    fn add_hl_bc(&mut self, _bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let bc = (u16::from(self.b) << 8) + u16::from(self.c);
        let (sum, carry) = hl.overflowing_add(bc);
        let half_carry = ((hl & 0xfff) + (bc & 0xfff)) & 0x1000 == 0x1000;
        self.h = sum.to_be_bytes()[0];
        self.l = sum.to_be_bytes()[1];
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn ld_a_ptr_bc(&mut self, bus: &mut dyn Bus) -> u8 {
        let bc = (u16::from(self.b) << 8) + u16::from(self.c);
        self.a = bus.read(bc);
        0
    }
    
    fn dec_bc(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.c.wrapping_sub(1);
        if self.c == 0xff {
            self.b = self.b.wrapping_sub(1);
        }
        0
    }
    
    fn inc_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.c.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.c & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.c > 0x7f) != (res > 0x7f);
        self.c = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.c.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.c & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.c > 0x7f) != (res > 0x7f);
        self.c = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_c_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.c = n;
        0
    }
    
    fn rrca(&mut self, _bus: &mut dyn Bus) -> u8 {
        if self.a & 0b00000001 == 0b00000001 {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.a = self.a.rotate_right(1);
        self.f &= 0b11101101;
        0
    }
    
    fn djnz_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let e_minus_2 = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        self.b = self.b.wrapping_sub(1);
        if self.b != 0 {
            self.pc = self.pc.wrapping_add(e_minus_2 as u16);
            5
        } else {
            0
        }
    }

    fn ld_de_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.e = n_low;
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.d = n_high;
        0
    }
    
    fn ld_ptr_de_a(&mut self, bus: &mut dyn Bus) -> u8 {
        let de = (u16::from(self.d) << 8) + u16::from(self.e);
        bus.write(de, self.a);
        0
    }

    fn inc_de(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.e.wrapping_add(1);
        if self.e == 0 {
            self.d = self.d.wrapping_add(1);
        }
        0
    }
    
    fn inc_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.d.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.d & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.d > 0x7f) != (res > 0x7f);
        self.d = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.d.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.d & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.d > 0x7f) != (res > 0x7f);
        self.d = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_d_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.d = n;
        0
    }
    
    fn jr_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let e_minus_2 = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        self.pc = self.pc.wrapping_add(e_minus_2 as u16);
        0
    }

    fn add_hl_de(&mut self, _bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let de = (u16::from(self.d) << 8) + u16::from(self.e);
        let (sum, carry) = hl.overflowing_add(de);
        let half_carry = ((hl & 0xfff) + (de & 0xfff)) & 0x1000 == 0x1000;
        self.h = sum.to_be_bytes()[0];
        self.l = sum.to_be_bytes()[1];
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn ld_a_ptr_de(&mut self, bus: &mut dyn Bus) -> u8 {
        let de = (u16::from(self.d) << 8) + u16::from(self.e);
        self.a = bus.read(de);
        0
    }
    
    fn dec_de(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.e.wrapping_sub(1);
        if self.e == 0xff {
            self.d = self.d.wrapping_sub(1);
        }
        0
    }
    
    fn inc_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.e.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.e & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.e > 0x7f) != (res > 0x7f);
        self.e = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.e.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.e & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.e > 0x7f) != (res > 0x7f);
        self.e = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_e_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.e = n;
        0
    }
    
    fn jr_nz_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let e_minus_2 = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b01000000 != 0b01000000 {
            self.pc = self.pc.wrapping_add(e_minus_2 as u16);
            5
        } else {
            0
        }
    }

    fn ld_hl_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.l = n_low;
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.h = n_high;
        0
    }
    
    fn ld_ptr_nn_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let nn = (u16::from(n_high) << 8) + u16::from(n_low);
        bus.write(nn, self.l);
        bus.write(nn + 1, self.h);
        0
    }

    fn inc_hl(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.l.wrapping_add(1);
        if self.l == 0 {
            self.h = self.h.wrapping_add(1);
        }
        0
    }
    
    fn inc_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.h.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.h & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.h > 0x7f) != (res > 0x7f);
        self.h = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.h.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.h & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.h > 0x7f) != (res > 0x7f);
        self.h = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_h_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.h = n;
        0
    }
    
    fn jr_z_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let e_minus_2 = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b01000000 == 0b01000000 {
            self.pc = self.pc.wrapping_add(e_minus_2 as u16);
            5
        } else {
            0
        }
    }

    fn add_hl_hl(&mut self, _bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let (sum, carry) = hl.overflowing_add(hl);
        let half_carry = ((hl & 0xfff) + (hl & 0xfff)) & 0x1000 == 0x1000;
        self.h = sum.to_be_bytes()[0];
        self.l = sum.to_be_bytes()[1];
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn ld_hl_ptr_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let nn = (u16::from(n_high) << 8) + u16::from(n_low);
        self.l = bus.read(nn);
        self.h = bus.read(nn.wrapping_add(1));
        0
    }

    fn dec_hl(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.l.wrapping_sub(1);
        if self.l == 0xff {
            self.h = self.h.wrapping_sub(1);
        }
        0
    }
    
    fn inc_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.l.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.l & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.l > 0x7f) != (res > 0x7f);
        self.l = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.l.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.l & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.l > 0x7f) != (res > 0x7f);
        self.l = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_l_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.l = n;
        0
    }
    
    fn jr_nc_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let e_minus_2 = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000001 != 0b00000001 {
            self.pc = self.pc.wrapping_add(e_minus_2 as u16);
            5
        } else {
            0
        }
    }

    fn ld_sp_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.sp = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }
    
    fn ld_ptr_nn_a(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let nn = (u16::from(n_high) << 8) + u16::from(n_low);
        bus.write(nn, self.a);
        0
    }

    fn inc_sp(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        0
    }
    
    fn inc_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let temp_ptr_hl = bus.read(hl);
        let (res, _) = temp_ptr_hl.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((temp_ptr_hl & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (temp_ptr_hl > 0x7f) != (res > 0x7f);
        bus.write(hl, res);
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }

    fn dec_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let temp_ptr_hl = bus.read(hl);
        let (res, _) = temp_ptr_hl.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (temp_ptr_hl & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (temp_ptr_hl > 0x7f) != (res > 0x7f);
        bus.write(hl, res);
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }

    fn ld_ptr_hl_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, n);
        0
    }

    fn jr_c_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let e_minus_2 = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000001 == 0b00000001 {
            self.pc = self.pc.wrapping_add(e_minus_2 as u16);
            5
        } else {
            0
        }
    }

    fn add_hl_sp(&mut self, _bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let (sum, carry) = hl.overflowing_add(self.sp);
        let half_carry = ((hl & 0xfff) + (self.sp & 0xfff)) & 0x1000 == 0x1000;
        self.h = sum.to_be_bytes()[0];
        self.l = sum.to_be_bytes()[1];
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_sp(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        0
    }
    
    fn inc_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.a.overflowing_add(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + 1) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11111101;
        0
    }
    
    fn dec_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, _) = self.a.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_a_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.a = n;
        0
    }
    
    fn ld_b_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.b;
        0
    }
    
    fn ld_b_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.c;
        0
    }
    
    fn ld_b_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.d;
        0
    }
    
    fn ld_b_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.e;
        0
    }
    
    fn ld_b_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.h;
        0
    }
    
    fn ld_b_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.l;
        0
    }
    
    fn ld_b_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.b = bus.read(hl);
        0
    }
    
    fn ld_b_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.b = self.a;
        0
    }
    
    fn ld_c_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.b;
        0
    }
    
    fn ld_c_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.c;
        0
    }
    
    fn ld_c_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.d;
        0
    }
    
    fn ld_c_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.e;
        0
    }
    
    fn ld_c_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.h;
        0
    }
    
    fn ld_c_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.l;
        0
    }
    
    fn ld_c_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.c = bus.read(hl);
        0
    }
    
    fn ld_c_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.c = self.a;
        0
    }
    
    fn ld_d_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.b;
        0
    }
    
    fn ld_d_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.c;
        0
    }
    
    fn ld_d_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.d;
        0
    }
    
    fn ld_d_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.e;
        0
    }
    
    fn ld_d_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.h;
        0
    }
    
    fn ld_d_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.l;
        0
    }
    
    fn ld_d_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.d = bus.read(hl);
        0
    }
    
    fn ld_d_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.d = self.a;
        0
    }
    
    fn ld_e_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.b;
        0
    }
    
    fn ld_e_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.c;
        0
    }
    
    fn ld_e_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.d;
        0
    }
    
    fn ld_e_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.e;
        0
    }
    
    fn ld_e_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.h;
        0
    }
    
    fn ld_e_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.l;
        0
    }
    
    fn ld_e_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.e = bus.read(hl);
        0
    }
    
    fn ld_e_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.e = self.a;
        0
    }
    
    fn ld_h_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.b;
        0
    }
    
    fn ld_h_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.c;
        0
    }
    
    fn ld_h_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.d;
        0
    }
    
    fn ld_h_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.e;
        0
    }
    
    fn ld_h_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.h;
        0
    }
    
    fn ld_h_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.l;
        0
    }
    
    fn ld_h_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.h = bus.read(hl);
        0
    }
    
    fn ld_h_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.h = self.a;
        0
    }
    
    fn ld_l_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.b;
        0
    }
    
    fn ld_l_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.c;
        0
    }
    
    fn ld_l_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.d;
        0
    }
    
    fn ld_l_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.e;
        0
    }
    
    fn ld_l_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.h;
        0
    }
    
    fn ld_l_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.l;
        0
    }
    
    fn ld_l_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.l = bus.read(hl);
        0
    }
    
    fn ld_l_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.l = self.a;
        0
    }
    
    fn ld_ptr_hl_b(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.b);
        0
    }
    
    fn ld_ptr_hl_c(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.c);
        0
    }
    
    fn ld_ptr_hl_d(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.d);
        0
    }
    
    fn ld_ptr_hl_e(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.e);
        0
    }
    
    fn ld_ptr_hl_h(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.h);
        0
    }
    
    fn ld_ptr_hl_l(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.l);
        0
    }
    
    fn ld_ptr_hl_a(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.a);
        0
    }
    
    fn ld_a_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.b;
        0
    }
    
    fn ld_a_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.c;
        0
    }
    
    fn ld_a_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.d;
        0
    }
    
    fn ld_a_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.e;
        0
    }
    
    fn ld_a_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.h;
        0
    }
    
    fn ld_a_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.l;
        0
    }
    
    fn ld_a_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.a = bus.read(hl);
        0
    }
    
    fn ld_a_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a;
        0
    }
    
    fn add_a_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.b);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.b & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.c);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.c & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.d);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.d & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.e);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.e & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.h);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.h & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.l);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.l & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let (res, carry) = self.a.overflowing_add(bus.read(hl));
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.l & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn add_a_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_add(self.a);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.a & 0xf)) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.b);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.b & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.c);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.c & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.d);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.d & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.e);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.e & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.h);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.h & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.l);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.l & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn adc_a_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_add(self.a);
        let (res, carry2) = res1.overflowing_add(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = ((self.a & 0xf) + (self.a & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f &= 0b11111101;
        0
    }

    fn sub_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.b);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sub_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.c);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.c & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sub_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.d);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.d & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sub_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.e);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.e & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sub_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.h);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.h & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sub_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.l);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.l & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sub_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.a);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.a & 0xf) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.b);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.b & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.c);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.c & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.d);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.d & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.e);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.e & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.h);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.h & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.l);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.l & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn sbc_a_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = self.f & 0b00000001;
        let (res1, carry1) = self.a.overflowing_sub(self.a);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub((self.a & 0xf) + carry_in) & 0x10 == 0x10;
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        self.a = res;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }

    fn and_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.b;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.c;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.d;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.e;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.h;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.l;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.a = self.a & bus.read(hl);
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn and_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a & self.a;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn xor_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.b;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.c;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.d;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.e;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.h;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.l;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.a = self.a ^ bus.read(hl);
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn xor_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a ^ self.a;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.b;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.c;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.d;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.e;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.h;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.l;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.a = self.a | bus.read(hl);
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn or_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.a = self.a | self.a;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn cp_b(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.b);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_c(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.c);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_d(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.d);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_e(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.e);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_h(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.h);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_l(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.l);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_ptr_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let (res, carry) = self.a.overflowing_sub(bus.read(hl));
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn cp_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        let (res, carry) = self.a.overflowing_sub(self.a);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(self.b & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ret_nz(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b01000000 != 0b01000000 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn pop_bc(&mut self, bus: &mut dyn Bus) -> u8 {
        self.c = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.b = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }

    fn jp_nz_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b01000000 != 0b01000000 {
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn jp_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }

    fn call_nz_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b01000000 != 0b01000000 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }
    
    fn push_bc(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.b);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.c);
        0
    }
    
    fn rst_00h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0000;
        0
    }

    fn ret_z(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b01000000 == 0b01000000 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn ret(&mut self, bus: &mut dyn Bus) -> u8 {
        let ret_low = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let ret_high = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
        0
    }
    
    fn jp_z_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b01000000 == 0b01000000 {
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn call_z_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b01000000 == 0b01000000 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn call_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }
    
    fn rst_08h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0008;
        0
    }

    fn ret_nc(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b00000001 != 0b00000001 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn pop_de(&mut self, bus: &mut dyn Bus) -> u8 {
        self.e = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.d = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }
    
    fn jp_nc_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000001 != 0b00000001 {
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn out_ptr_n_a(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let addr = (u16::from(self.a) << 8) + u16::from(n);
        bus.write(addr, self.a);
        0
    }

    fn call_nc_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000001 != 0b00000001 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn push_de(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.d);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.e);
        0
    }
    
    fn rst_10h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0010;
        0
    }

    fn ret_c(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b00000001 == 0b00000001 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn exx(&mut self, _bus: &mut dyn Bus) -> u8 {
        let temp_b = self.b;
        self.b = self.b_alt;
        self.b_alt = temp_b;
        let temp_c = self.c;
        self.c = self.c_alt;
        self.c_alt = temp_c;
        let temp_d = self.d;
        self.d = self.d_alt;
        self.d_alt = temp_d;
        let temp_e = self.e;
        self.e = self.e_alt;
        self.e_alt = temp_e;
        let temp_h = self.h;
        self.h = self.h_alt;
        self.h_alt = temp_h;
        let temp_l = self.l;
        self.l = self.l_alt;
        self.l_alt = temp_l;
        0
    }

    fn jp_c_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000001 == 0b00000001 {
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn call_c_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000001 == 0b00000001 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn ix(&mut self, bus: &mut dyn Bus) -> u8 {
        let ix_opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let mut t_cycles = IX_OPCODES[usize::from(ix_opcode)].3;
        t_cycles += IX_OPCODES[usize::from(ix_opcode)].1(self, bus);
        t_cycles
    }
    
    fn ld_ix_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.ix = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }
    
    fn rst_18h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0018;
        0
    }

    fn ret_po(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b00000100 != 0b00000100 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn pop_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        self.l = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.h = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }

    fn jp_po_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000100 != 0b00000100 {
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }
    
    fn ex_ptr_sp_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        let temp_ptr_sp_low = bus.read(self.sp);
        let temp_ptr_sp_high = bus.read(self.sp.wrapping_add(1));
        bus.write(self.sp, self.l);
        bus.write(self.sp.wrapping_add(1), self.h);
        self.l = temp_ptr_sp_low;
        self.h = temp_ptr_sp_high;
        0
    }

    fn call_po_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000100 != 0b00000100 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn push_hl(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.h);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.l);
        0
    }
    
    fn and_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.a = self.a & n;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        self.f |= 0b00010000;
        0
    }
    
    fn rst_20h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0020;
        0
    }

    fn ret_pe(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b00000100 == 0b00000100 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn jp_pe_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000100 == 0b00000100 {
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn ex_de_hl(&mut self, _bus: &mut dyn Bus) -> u8 {
        let temp_d = self.d;
        self.d = self.h;
        self.h = temp_d;
        let temp_e = self.e;
        self.e = self.l;
        self.l = temp_e;
        0
    }

    fn call_pe_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b00000100 == 0b00000100 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn extended(&mut self, bus: &mut dyn Bus) -> u8 {
        let extended_opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let mut t_cycles = EXTENDED_OPCODES[usize::from(extended_opcode)].3;
        t_cycles += EXTENDED_OPCODES[usize::from(extended_opcode)].1(self, bus);
        t_cycles
    }

    fn ld_ptr_nn_bc(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let nn = (u16::from(n_high) << 8) + u16::from(n_low);
        bus.write(nn, self.c);
        bus.write(nn + 1, self.b);
        0
    }

    fn im_0(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.interrupt_mode = 0;
        0
    }

    fn ld_i_a(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.i = self.a;
        0
    }

    fn im_1(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.interrupt_mode = 1;
        0
    }

    fn im_2(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.interrupt_mode = 2;
        0
    }

    fn sbc_hl_de(&mut self, _bus: &mut dyn Bus) -> u8 {
        let carry_in = u16::from(self.f & 0b00000001);
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let de = (u16::from(self.d) << 8) + u16::from(self.e);
        let (res1, carry1) = hl.overflowing_sub(de);
        let (res, carry2) = res1.overflowing_sub(carry_in);
        let carry = carry1 || carry2;
        let sign = res > 0x7fff;
        let zero = res == 0;
        let half_carry = (hl & 0xfff).wrapping_sub((de & 0xfff) + carry_in) & 0x1000 == 0x1000;
        let overflow = (hl > 0x7fff) != (res > 0x7fff);
        self.h = res.to_be_bytes()[0];
        self.l = res.to_be_bytes()[1];
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn ld_ptr_nn_de(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let nn = (u16::from(n_high) << 8) + u16::from(n_low);
        bus.write(nn, self.e);
        bus.write(nn + 1, self.d);
        0
    }

    fn ldir(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let de = (u16::from(self.d) << 8) + u16::from(self.e);
        let bc = (u16::from(self.b) << 8) + u16::from(self.c);
        bus.write(de, bus.read(hl));
        self.h = hl.wrapping_add(1).to_be_bytes()[0];
        self.l = hl.wrapping_add(1).to_be_bytes()[1];
        self.d = de.wrapping_add(1).to_be_bytes()[0];
        self.e = de.wrapping_add(1).to_be_bytes()[1];
        self.b = bc.wrapping_sub(1).to_be_bytes()[0];
        self.c = bc.wrapping_sub(1).to_be_bytes()[1];
        self.f &= 0b11101101;
        if bc.wrapping_sub(1) == 0 {
            self.f &= 0b11111011;
            0
        } else {
            self.f |= 0b00000100;
            self.pc = self.pc.wrapping_sub(2);
            5
        }
    }

    fn lddr(&mut self, bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        let de = (u16::from(self.d) << 8) + u16::from(self.e);
        let bc = (u16::from(self.b) << 8) + u16::from(self.c);
        bus.write(de, bus.read(hl));
        self.h = hl.wrapping_sub(1).to_be_bytes()[0];
        self.l = hl.wrapping_sub(1).to_be_bytes()[1];
        self.d = de.wrapping_sub(1).to_be_bytes()[0];
        self.e = de.wrapping_sub(1).to_be_bytes()[1];
        self.b = bc.wrapping_sub(1).to_be_bytes()[0];
        self.c = bc.wrapping_sub(1).to_be_bytes()[1];
        self.f &= 0b11101001;
        if bc.wrapping_sub(1) == 0 {
            0
        } else {
            self.pc = self.pc.wrapping_sub(2);
            5
        }
    }

    fn xor_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.a = self.a ^ n;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn rst_28h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0028;
        0
    }

    fn ret_p(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b10000000 != 0b10000000 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn pop_af(&mut self, bus: &mut dyn Bus) -> u8 {
        self.f = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        self.a = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        0
    }
    
    fn jp_p_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b10000000 != 0b10000000 {
            
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn di(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.interrupts_enabled = false;
        0
    }

    fn call_p_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b10000000 != 0b10000000 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn push_af(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.a);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.f);
        0
    }
    
    fn or_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        self.a = self.a | n;
        let sign = self.a > 0x7f;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        let zero = self.a == 0;
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        let parity_even = self.a.count_ones() % 2 == 0;
        if parity_even {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f &= 0b11101100;
        0
    }
    
    fn rst_30h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0030;
        0
    }

    fn ret_m(&mut self, bus: &mut dyn Bus) -> u8 {
        //            SZ H VNC
        if self.f & 0b10000000 == 0b10000000 {
            let ret_low = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            let ret_high = bus.read(self.sp);
            self.sp = self.sp.wrapping_add(1);
            self.pc = (u16::from(ret_high) << 8) + u16::from(ret_low);
            6
        } else {
            0
        }
    }

    fn ld_sp_hl(&mut self, _bus: &mut dyn Bus) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.sp = hl;
        0
    }
    
    fn jp_m_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b10000000 == 0b10000000 {
            
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
        }
        0
    }

    fn ei(&mut self, _bus: &mut dyn Bus) -> u8 {
        self.interrupts_enabled = true;
        0
    }

    fn call_m_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        //            SZ H VNC
        if self.f & 0b10000000 == 0b10000000 {
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[0]);
            self.sp = self.sp.wrapping_sub(1);
            bus.write(self.sp, self.pc.to_be_bytes()[1]);
            self.pc = (u16::from(n_high) << 8) + u16::from(n_low);
            7
        } else {
            0
        }
    }

    fn iy(&mut self, bus: &mut dyn Bus) -> u8 {
        let iy_opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let mut t_cycles = IY_OPCODES[usize::from(iy_opcode)].3;
        t_cycles += IY_OPCODES[usize::from(iy_opcode)].1(self, bus);
        t_cycles
    }

    fn ld_iy_nn(&mut self, bus: &mut dyn Bus) -> u8 {
        let n_low = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let n_high = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.iy = (u16::from(n_high) << 8) + u16::from(n_low);
        0
    }

    fn dec_ptr_iy_d(&mut self, bus: &mut dyn Bus) -> u8 {
        let d = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        let iy_d = self.iy.wrapping_add(d as u16);
        let temp_ptr_iy_d = bus.read(iy_d);
        let (res, _) = temp_ptr_iy_d.overflowing_sub(1);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (temp_ptr_iy_d & 0xf).wrapping_sub(1) & 0x10 == 0x10;
        let overflow = (temp_ptr_iy_d > 0x7f) != (res > 0x7f);
        bus.write(iy_d, res);
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        self.f |= 0b00000010;
        0
    }

    fn set_b_ptr_iy_d(&mut self, bus: &mut dyn Bus) -> u8 {
        let d = bus.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        let b = (bus.read(self.pc) & 0b00111000) >> 3;
        self.pc = self.pc.wrapping_add(1);
        let iy_d = self.iy.wrapping_add(d as u16);
        let temp_ptr_iy_d = bus.read(iy_d);
        let res = temp_ptr_iy_d | (1 << b);
        bus.write(iy_d, res);
        0
    }

    fn cp_n(&mut self, bus: &mut dyn Bus) -> u8 {
        let n = bus.read(self.pc);
        let (res, carry) = self.a.overflowing_sub(n);
        let sign = res > 0x7f;
        let zero = res == 0;
        let half_carry = (self.a & 0xf).wrapping_sub(n & 0xf) & 0x10 == 0x10;
        if sign {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
        if zero {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
        if half_carry {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
        let overflow = (self.a > 0x7f) != (res > 0x7f);
        if overflow {
            self.f |= 0b00000100;
        } else {
            self.f &= 0b11111011;
        }
        if carry {
            self.f |= 0b00000001;
        } else {
            self.f &= 0b11111110;
        }
        self.f |= 0b00000010;
        0
    }
    
    fn rst_38h(&mut self, bus: &mut dyn Bus) -> u8 {
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[0]);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, self.pc.to_be_bytes()[1]);
        self.pc = 0x0038;
        0
    }

    fn invalid_opcode(&mut self, _bus: &mut dyn Bus) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    
    use super::*;
    use super::super::machine::MockBus;
    
    #[test]
    fn test_nop() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x00);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: NOP");
    }
    
    #[test]
    fn test_ld_bc_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        assert_eq!(disasm, "0000: LD BC, BAADh");
    }
    
    #[test]
    fn test_ld_ptr_bc_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x02);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x40;
        cpu.c = 0x01;
        cpu.a = 0x2a;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD (BC), A");
    }
    
    #[test]
    fn test_inc_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x03);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x01;
        cpu.c = 0xff;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x02);
        assert_eq!(cpu.c, 0x00);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: INC BC");
    }
    
    #[test]
    fn test_inc_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x04);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC B");
    }
    
    #[test]
    fn test_dec_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x05);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC B");
    }
    
    #[test]
    fn test_ld_b_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x06);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD B, D9h");
    }
    
    #[test]
    fn test_rlca() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x07);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0b10001000;
        //        SZ H VNC
        cpu.f = 0b00010010;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0b00010001);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00000001);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: RLCA");
    }
    
    #[test]
    fn test_ex_af_af_alt() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x08);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x99;
        cpu.f = 0x48;
        cpu.a_alt = 0x59;
        cpu.f_alt = 0x44;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0x59);
        assert_eq!(cpu.f, 0x44);
        assert_eq!(cpu.a_alt, 0x99);
        assert_eq!(cpu.f_alt, 0x48);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: EX AF, AF'");
    }
    
    #[test]
    fn test_add_hl_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x09);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xff;
        cpu.l = 0xfe;
        cpu.b = 0x12;
        cpu.c = 0x34;
        //        SZ H VNC
        cpu.f = 0b00000010;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x32);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00010001);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: ADD HL, BC");
    }
    
    #[test]
    fn test_ld_a_ptr_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0a);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x40;
        cpu.c = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD A, (BC)");
    }
    
    #[test]
    fn test_dec_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0b);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x02;
        cpu.c = 0x00;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x01);
        assert_eq!(cpu.c, 0xff);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: DEC BC");
    }
    
    #[test]
    fn test_inc_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0c);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.c, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC C");
    }
    
    #[test]
    fn test_dec_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0d);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.c = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.c, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC C");
    }
    
    #[test]
    fn test_ld_c_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.c, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD C, D9h");
    }
    
    #[test]
    fn test_rrca() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0f);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0b00010001;
        //        SZ H VNC
        cpu.f = 0b00010010;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0b10001000);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00000001);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: RRCA");
    }
    
    #[test]
    fn test_djnz_e_when_e_is_positive_and_b_is_not_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x10);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x56);
        assert_eq!(cpu.pc, 0x1239);
        assert_eq!(1 + cpu.t_cycles, 13);
        assert_eq!(disasm, "1234: DJNZ $+5");
    }

    #[test]
    fn test_djnz_e_when_e_is_positive_and_b_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x10);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.pc, 0x1236);
        assert_eq!(1 + cpu.t_cycles, 8);
        assert_eq!(disasm, "1234: DJNZ $+5");
    }

    #[test]
    fn test_djnz_e_when_e_is_negative_and_b_is_not_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x10);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x56);
        assert_eq!(cpu.pc, 0x122f);
        assert_eq!(1 + cpu.t_cycles, 13);
        assert_eq!(disasm, "1234: DJNZ $-5");
    }

    #[test]
    fn test_djnz_e_when_e_is_negative_and_b_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x10);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.pc, 0x1236);
        assert_eq!(1 + cpu.t_cycles, 8);
        assert_eq!(disasm, "1234: DJNZ $-5");
    }

    #[test]
    fn test_ld_de_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        assert_eq!(disasm, "0000: LD DE, BAADh");
    }
    
    #[test]
    fn test_ld_ptr_de_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x12);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x40;
        cpu.e = 0x01;
        cpu.a = 0x2a;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD (DE), A");
    }

    #[test]
    fn test_inc_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x13);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x01;
        cpu.e = 0xff;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0x02);
        assert_eq!(cpu.e, 0x00);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: INC DE");
    }
    
    #[test]
    fn test_inc_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x14);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC D");
    }
    
    #[test]
    fn test_dec_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x15);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC D");
    }
    
    #[test]
    fn test_ld_d_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x16);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD D, D9h");
    }
    
    #[test]
    fn test_jr_e_when_e_is_positive() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x18);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1239);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR $+5");
    }

    #[test]
    fn test_jr_e_when_e_is_negative() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x18);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x122f);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR $-5");
    }

    #[test]
    fn test_add_hl_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x19);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xff;
        cpu.l = 0xfe;
        cpu.d = 0x12;
        cpu.e = 0x34;
        //        SZ H VNC
        cpu.f = 0b00000010;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x32);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00010001);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: ADD HL, DE");
    }
    
    #[test]
    fn test_ld_a_ptr_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1a);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x40;
        cpu.e = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD A, (DE)");
    }
    
    #[test]
    fn test_dec_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1b);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x02;
        cpu.e = 0x00;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0x01);
        assert_eq!(cpu.e, 0xff);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: DEC DE");
    }
    
    #[test]
    fn test_inc_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1c);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.e, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC E");
    }
    
    #[test]
    fn test_dec_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1d);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.e = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.e, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC E");
    }
    
    #[test]
    fn test_ld_e_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.e, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD E, D9h");
    }
    
    #[test]
    fn test_jr_nz_e_when_z_is_0_and_e_is_positive() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x20);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1239);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR NZ, $+5");
    }

    #[test]
    fn test_jr_nz_e_when_z_is_0_and_e_is_negative() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x20);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x122f);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR NZ, $-5");
    }

    #[test]
    fn test_jr_nz_e_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x20);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1236);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "1234: JR NZ, $+5");
    }

    #[test]
    fn test_ld_hl_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        assert_eq!(disasm, "0000: LD HL, 4001h");
    }
    
    #[test]
    fn test_ld_ptr_nn_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x22);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x29);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xb2);
        mock_bus.expect_write().with(eq(0xb229), eq(0x3a)).return_const(());
        mock_bus.expect_write().with(eq(0xb22a), eq(0x48)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x48;
        cpu.l = 0x3a;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 16);
        assert_eq!(disasm, "0000: LD (B229h), HL");
    }

    #[test]
    fn test_inc_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x23);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x01;
        cpu.l = 0xff;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x02);
        assert_eq!(cpu.l, 0x00);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: INC HL");
    }
    
    #[test]
    fn test_inc_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x24);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC H");
    }
    
    #[test]
    fn test_dec_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x25);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC H");
    }
    
    #[test]
    fn test_ld_h_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x26);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD H, D9h");
    }
    
    #[test]
    fn test_jr_z_e_when_z_is_1_and_e_is_positive() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x28);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1239);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR Z, $+5");
    }

    #[test]
    fn test_jr_z_e_when_z_is_1_and_e_is_negative() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x28);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x122f);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR Z, $-5");
    }

    #[test]
    fn test_jr_z_e_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x28);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1236);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "1234: JR Z, $+5");
    }

    #[test]
    fn test_add_hl_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x29);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xff;
        cpu.l = 0xfe;
        //        SZ H VNC
        cpu.f = 0b00000010;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0xff);
        assert_eq!(cpu.l, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00010001);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: ADD HL, HL");
    }
    
    #[test]
    fn test_ld_hl_ptr_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2a);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x01);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x40);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0xd7);
        mock_bus.expect_read().with(eq(0x4002)).returning(|_| 0x2e);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x2e);
        assert_eq!(cpu.l, 0xd7);
        assert_eq!(cpu.pc, 0x3);
        assert_eq!(1 + cpu.t_cycles, 16);
        assert_eq!(disasm, "0000: LD HL, (4001h)");
    }

    #[test]
    fn test_dec_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2b);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x02;
        cpu.l = 0x00;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x01);
        assert_eq!(cpu.l, 0xff);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: DEC HL");
    }
    
    #[test]
    fn test_inc_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2c);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.l, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC L");
    }
    
    #[test]
    fn test_dec_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2d);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.l = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.l, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC L");
    }
    
    #[test]
    fn test_ld_l_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.l, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD L, D9h");
    }
    
    #[test]
    fn test_jr_nc_e_when_c_is_0_and_e_is_positive() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x30);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1239);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR NC, $+5");
    }

    #[test]
    fn test_jr_nc_e_when_c_is_0_and_e_is_negative() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x30);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x122f);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR NC, $-5");
    }

    #[test]
    fn test_jr_nc_e_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x30);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1236);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "1234: JR NC, $+5");
    }

    #[test]
    fn test_ld_sp_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x31);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.sp, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: LD SP, BAADh");
    }
    
    #[test]
    fn test_ld_ptr_nn_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x32);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x41);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x31);
        mock_bus.expect_write().with(eq(0x3141), eq(0xd7)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xd7;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 13);
        assert_eq!(disasm, "0000: LD (3141h), A");
    }

    #[test]
    fn test_inc_sp() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x33);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x01ff;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.sp, 0x0200);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: INC SP");
    }
    
    #[test]
    fn test_inc_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x34);
        mock_bus.expect_read().with(eq(0x3434)).returning(|_| 0x82);
        mock_bus.expect_write().with(eq(0x3434), eq(0x83)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x34;
        cpu.l = 0x34;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: INC (HL)");
    }

    #[test]
    fn test_dec_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x35);
        mock_bus.expect_read().with(eq(0x3434)).returning(|_| 0x83);
        mock_bus.expect_write().with(eq(0x3434), eq(0x82)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x34;
        cpu.l = 0x34;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: DEC (HL)");
    }

    #[test]
    fn test_ld_ptr_hl_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x36);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x28);
        mock_bus.expect_write().with(eq(0x4444), eq(0x28)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x44;
        cpu.l = 0x44;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: LD (HL), 28h");
    }

    #[test]
    fn test_jr_c_e_when_c_is_1_and_e_is_positive() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x38);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1239);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR C, $+5");
    }

    #[test]
    fn test_jr_c_e_when_c_is_1_and_e_is_negative() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x38);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xf9);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x122f);
        assert_eq!(1 + cpu.t_cycles, 12);
        assert_eq!(disasm, "1234: JR C, $-5");
    }

    #[test]
    fn test_jr_c_e_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0x38);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0x03);

        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1236);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "1234: JR C, $+5");
    }

    #[test]
    fn test_add_hl_sp() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x39);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0xff;
        cpu.l = 0xfe;
        cpu.sp = 0x1234;
        //        SZ H VNC
        cpu.f = 0b00000010;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x32);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00010001);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: ADD HL, SP");
    }
    
    #[test]
    fn test_dec_sp() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3b);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x0200;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.sp, 0x01ff);
        assert_eq!(1 + cpu.t_cycles, 6);
        assert_eq!(disasm, "0000: DEC SP");
    }
    
    #[test]
    fn test_inc_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3c);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfb;
        //        SZ H VNC
        cpu.f = 0b01010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfc);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: INC A");
    }
    
    #[test]
    fn test_dec_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3d);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfc;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DEC A");
    }
    
    #[test]
    fn test_ld_a_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD A, 2Ah");
    }
    
    #[test]
    fn test_ld_b_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_b_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x46);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD B, (HL)");
    }
    
    #[test]
    fn test_ld_b_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_c_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4e);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.c, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD C, (HL)");
    }
    
    #[test]
    fn test_ld_c_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_d_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x56);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD D, (HL)");
    }
    
    #[test]
    fn test_ld_d_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_e_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5e);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.e, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD E, (HL)");
    }
    
    #[test]
    fn test_ld_e_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_h_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x66);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD H, (HL)");
    }
    
    #[test]
    fn test_ld_h_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_l_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6e);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.l, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD L, (HL)");
    }
    
    #[test]
    fn test_ld_l_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
        let mut mock_bus = MockBus::new();
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
    fn test_ld_a_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7e);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x2a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: LD A, (HL)");
    }
    
    #[test]
    fn test_ld_a_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_add_a_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x80);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.b = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, B");
    }

    #[test]
    fn test_add_a_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x81);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.c = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, C");
    }

    #[test]
    fn test_add_a_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x82);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.d = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, D");
    }

    #[test]
    fn test_add_a_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x83);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.e = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, E");
    }

    #[test]
    fn test_add_a_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x84);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.h = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, H");
    }

    #[test]
    fn test_add_a_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x85);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.l = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, L");
    }

    #[test]
    fn test_add_a_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x86);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x7e);

        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0xfd);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: ADD A, (HL)");
    }

    #[test]
    fn test_add_a_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x87);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADD A, A");
    }
    
    #[test]
    fn test_adc_a_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x88);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.b = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, B");
    }

    #[test]
    fn test_adc_a_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x89);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.c = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, C");
    }

    #[test]
    fn test_adc_a_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x8a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.d = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, D");
    }

    #[test]
    fn test_adc_a_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x8b);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.e = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, E");
    }

    #[test]
    fn test_adc_a_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x8c);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.h = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, H");
    }

    #[test]
    fn test_adc_a_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x8d);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        cpu.l = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfe);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, L");
    }

    #[test]
    fn test_adc_a_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x8f);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: ADC A, A");
    }

    #[test]
    fn test_sub_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x90);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7e;
        cpu.b = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB B");
    }

    #[test]
    fn test_sub_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x91);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7e;
        cpu.c = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB C");
    }

    #[test]
    fn test_sub_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x92);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7e;
        cpu.d = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB D");
    }

    #[test]
    fn test_sub_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x93);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7e;
        cpu.e = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB E");
    }

    #[test]
    fn test_sub_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x94);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7e;
        cpu.h = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB H");
    }

    #[test]
    fn test_sub_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x95);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7e;
        cpu.l = 0x7f;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB L");
    }

    #[test]
    fn test_sub_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x97);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x7f;
        //        SZ H VNC
        cpu.f = 0b10010101;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b01000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SUB A");
    }

    #[test]
    fn test_sbc_a_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x98);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        cpu.b = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01010001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xe0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, B");
    }

    #[test]
    fn test_sbc_a_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x99);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        cpu.c = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01010001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xe0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, C");
    }

    #[test]
    fn test_sbc_a_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x9a);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        cpu.d = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01010001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xe0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, D");
    }

    #[test]
    fn test_sbc_a_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x9b);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        cpu.e = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01010001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xe0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, E");
    }

    #[test]
    fn test_sbc_a_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x9c);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        cpu.h = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01010001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xe0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, H");
    }

    #[test]
    fn test_sbc_a_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x9d);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        cpu.l = 0x7e;
        //        SZ H VNC
        cpu.f = 0b01010001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xe0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, L");
    }

    #[test]
    fn test_sbc_a_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x9f);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x5f;
        //        SZ H VNC
        cpu.f = 0b01000001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010111);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: SBC A, A");
    }

    #[test]
    fn test_and_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa0);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.b = 0xf5;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND B");
    }
    
    #[test]
    fn test_and_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa1);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.c = 0xf5;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND C");
    }
    
    #[test]
    fn test_and_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa2);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.d = 0xf5;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND D");
    }
    
    #[test]
    fn test_and_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa3);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.e = 0xf5;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND E");
    }
    
    #[test]
    fn test_and_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa4);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.h = 0xf5;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND H");
    }
    
    #[test]
    fn test_and_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa5);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.l = 0xf5;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND L");
    }
    
    #[test]
    fn test_and_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa6);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0xf5);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.h = 0x40;
        cpu.l = 0x01;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: AND (HL)");
    }
    
    #[test]
    fn test_and_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa7);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        //        SZ H VNC
        cpu.f = 0b01000011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc3);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: AND A");
    }
    
    #[test]
    fn test_xor_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa8);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.b = 0x15;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR B");
    }
    
    #[test]
    fn test_xor_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xa9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.c = 0x15;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR C");
    }
    
    #[test]
    fn test_xor_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xaa);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.d = 0x15;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR D");
    }
    
    #[test]
    fn test_xor_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xab);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.e = 0x15;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR E");
    }
    
    #[test]
    fn test_xor_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xac);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.h = 0x15;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR H");
    }
    
    #[test]
    fn test_xor_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xad);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.l = 0x15;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR L");
    }
    
    #[test]
    fn test_xor_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xae);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x15);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        cpu.h = 0x40;
        cpu.l = 0x01;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: XOR (HL)");
    }
    
    #[test]
    fn test_xor_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xaf);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b01000100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: XOR A");
    }
    
    #[test]
    fn test_or_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb0);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.b = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR B");
    }
    
    #[test]
    fn test_or_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb1);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.c = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR C");
    }
    
    #[test]
    fn test_or_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb2);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.d = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR D");
    }
    
    #[test]
    fn test_or_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb3);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.e = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR E");
    }
    
    #[test]
    fn test_or_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb4);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.h = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR H");
    }
    
    #[test]
    fn test_or_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb5);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.l = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR L");
    }
    
    #[test]
    fn test_or_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb6);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0xfa);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.h = 0x40;
        cpu.l = 0x01;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: OR (HL)");
    }
    
    #[test]
    fn test_or_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb7);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfa);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000100);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: OR A");
    }
    
    #[test]
    fn test_cp_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb8);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.b = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP B");
    }
    
    #[test]
    fn test_cp_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xb9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.c = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP C");
    }
    
    #[test]
    fn test_cp_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.d = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP D");
    }
    
    #[test]
    fn test_cp_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xbb);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.e = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP E");
    }
    
    #[test]
    fn test_cp_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xbc);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.h = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP H");
    }
    
    #[test]
    fn test_cp_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xbd);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.l = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP L");
    }
    
    #[test]
    fn test_cp_ptr_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xbe);
        mock_bus.expect_read().with(eq(0x4001)).returning(|_| 0x23);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        cpu.h = 0x40;
        cpu.l = 0x01;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: CP (HL)");
    }
    
    #[test]
    fn test_cp_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xbf);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        //        SZ H VNC
        cpu.f = 0b10010001;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b01000110);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: CP A");
    }
    
    #[test]
    fn test_ret_nz_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xc0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET NZ");
    }

    #[test]
    fn test_ret_nz_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xc0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET NZ");
    }

    #[test]
    fn test_pop_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_jp_nz_nn_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xc2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP NZ, BAADh");
    }

    #[test]
    fn test_jp_nz_nn_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xc2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP NZ, BAADh");
    }

    #[test]
    fn test_jp_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xc3);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP BAADh");
    }
    
    #[test]
    fn test_call_nz_nn_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xc4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL NZ, BAADh");
    }

    #[test]
    fn test_call_nz_nn_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xc4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL NZ, BAADh");
    }

    #[test]
    fn test_push_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_rst_00h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xc7);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0000);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 00h");
    }

    #[test]
    fn test_ret_z_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xc8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET Z");
    }

    #[test]
    fn test_ret_z_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xc8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET Z");
    }

    #[test]
    fn test_ret() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xc9);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "BAAD: RET");
    }
    
    #[test]
    fn test_jp_z_nn_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xca);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP Z, BAADh");
    }

    #[test]
    fn test_jp_z_nn_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xca);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP Z, BAADh");
    }
    
    #[test]
    fn test_call_z_nn_when_z_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xcc);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01000000;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL Z, BAADh");
    }

    #[test]
    fn test_call_z_nn_when_z_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xcc);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL Z, BAADh");
    }

    #[test]
    fn test_call_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xcd);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL BAADh");
    }
    
    #[test]
    fn test_rst_08h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xcf);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0008);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 08h");
    }

    #[test]
    fn test_ret_nc_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xd0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET NC");
    }

    #[test]
    fn test_ret_nc_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xd0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET NC");
    }

    #[test]
    fn test_pop_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_jp_nc_nn_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP NC, BAADh");
    }

    #[test]
    fn test_jp_nc_nn_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP NC, BAADh");
    }
    
    #[test]
    fn test_out_ptr_n_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd3);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x01);
        mock_bus.expect_write().with(eq(0x2301), eq(0x23)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "0000: OUT (01h), A");
    }

    #[test]
    fn test_call_nc_nn_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xd4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL NC, BAADh");
    }

    #[test]
    fn test_call_nc_nn_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xd4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL NC, BAADh");
    }

    #[test]
    fn test_push_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_rst_10h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xd7);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0010);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 10h");
    }

    #[test]
    fn test_ret_c_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xd8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET C");
    }

    #[test]
    fn test_ret_c_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xd8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010110;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET C");
    }

    #[test]
    fn test_exx() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd9);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x99;
        cpu.c = 0x48;
        cpu.b_alt = 0x59;
        cpu.c_alt = 0x44;
        cpu.d = 0x12;
        cpu.e = 0x34;
        cpu.d_alt = 0x56;
        cpu.e_alt = 0x78;
        cpu.h = 0x81;
        cpu.l = 0xea;
        cpu.h_alt = 0xd2;
        cpu.l_alt = 0xc5;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.b, 0x59);
        assert_eq!(cpu.c, 0x44);
        assert_eq!(cpu.b_alt, 0x99);
        assert_eq!(cpu.c_alt, 0x48);
        assert_eq!(cpu.d, 0x56);
        assert_eq!(cpu.e, 0x78);
        assert_eq!(cpu.d_alt, 0x12);
        assert_eq!(cpu.e_alt, 0x34);
        assert_eq!(cpu.h, 0xd2);
        assert_eq!(cpu.l, 0xc5);
        assert_eq!(cpu.h_alt, 0x81);
        assert_eq!(cpu.l_alt, 0xea);
        assert_eq!(cpu.pc, 1);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: EXX");
    }

    #[test]
    fn test_jp_c_nn_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xda);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000001;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP C, BAADh");
    }

    #[test]
    fn test_jp_c_nn_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xda);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP C, BAADh");
    }

    #[test]
    fn test_call_c_nn_when_c_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xdc);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000001;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL C, BAADh");
    }

    #[test]
    fn test_call_c_nn_when_c_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xdc);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL C, BAADh");
    }

    #[test]
    fn test_ld_ix_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
        assert_eq!(disasm, "0000: LD IX, BAADh");
    }
    
    #[test]
    fn test_rst_18h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xdf);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0018);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 18h");
    }

    #[test]
    fn test_ret_po_when_v_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xe0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010011;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET PO");
    }

    #[test]
    fn test_ret_po_when_v_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xe0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET PO");
    }

    #[test]
    fn test_pop_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_jp_po_nn_when_v_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xe2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP PO, BAADh");
    }

    #[test]
    fn test_ex_ptr_sp_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xe3);
        mock_bus.expect_read().with(eq(0x8856)).returning(|_| 0x11);
        mock_bus.expect_read().with(eq(0x8857)).returning(|_| 0x22);
        mock_bus.expect_write().with(eq(0x8856), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x8857), eq(0x70)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x70;
        cpu.l = 0x12;
        cpu.sp = 0x8856;
        
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x22);
        assert_eq!(cpu.l, 0x11);
        assert_eq!(cpu.sp, 0x8856);
        assert_eq!(1 + cpu.t_cycles, 19);
        assert_eq!(disasm, "0000: EX (SP), HL");
    }

    #[test]
    fn test_jp_po_nn_when_v_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xe2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP PO, BAADh");
    }

    #[test]
    fn test_call_po_nn_when_v_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xe4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010011;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL PO, BAADh");
    }

    #[test]
    fn test_call_po_nn_when_v_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xe4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL PO, BAADh");
    }

    #[test]
    fn test_push_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_and_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xe6);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xf5);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        //        SZ H VNC
        cpu.f = 0b01000111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xc1);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10010000);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: AND F5h");
    }
    
    #[test]
    fn test_rst_20h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xe7);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0020);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 20h");
    }

    #[test]
    fn test_ret_pe_when_v_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xe8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET PE");
    }

    #[test]
    fn test_ret_pe_when_v_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xe8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010011;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET PE");
    }

    #[test]
    fn test_jp_pe_nn_when_v_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xea);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP PE, BAADh");
    }

    #[test]
    fn test_jp_pe_nn_when_v_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xea);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP PE, BAADh");
    }

    #[test]
    fn test_ex_de_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xeb);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x99;
        cpu.e = 0x48;
        cpu.h = 0x59;
        cpu.l = 0x44;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.d, 0x59);
        assert_eq!(cpu.e, 0x44);
        assert_eq!(cpu.h, 0x99);
        assert_eq!(cpu.l, 0x48);
        assert_eq!(cpu.pc, 1);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: EX DE, HL");
    }

    #[test]
    fn test_call_pe_nn_when_v_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xec);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000100;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL PE, BAADh");
    }

    #[test]
    fn test_call_pe_nn_when_v_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xec);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL PE, BAADh");
    }

    #[test]
    fn test_ld_ptr_nn_bc() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x43);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x29);
        mock_bus.expect_read().with(eq(3)).returning(|_| 0xb2);
        mock_bus.expect_write().with(eq(0xb229), eq(0x3a)).return_const(());
        mock_bus.expect_write().with(eq(0xb22a), eq(0x48)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.b = 0x48;
        cpu.c = 0x3a;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 4);
        assert_eq!(1 + cpu.t_cycles, 20);
        assert_eq!(disasm, "0000: LD (B229h), BC");
    }

    #[test]
    fn test_im_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x46);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.interrupt_mode = 1;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.interrupt_mode, 0);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 8);
        assert_eq!(disasm, "0000: IM 0");
    }

    #[test]
    fn test_ld_i_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x47);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x24;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.i, 0x24);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 9);
        assert_eq!(disasm, "0000: LD I, A");
    }

    #[test]
    fn test_sbc_hl_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x52);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x3f;
        cpu.l = 0xff;
        cpu.d = 0xff;
        cpu.e = 0xff;
        //        SZ H VNC
        cpu.f = 0b11010100;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x40);
        assert_eq!(cpu.l, 0x00);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b00000011);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 15);
        assert_eq!(disasm, "0000: SBC HL, DE");
    }

    #[test]
    fn test_im_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x56);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.interrupt_mode = 2;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.interrupt_mode, 1);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 8);
        assert_eq!(disasm, "0000: IM 1");
    }

    #[test]
    fn test_im_2() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x5e);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.interrupt_mode = 1;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.interrupt_mode, 2);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 8);
        assert_eq!(disasm, "0000: IM 2");
    }

    #[test]
    fn test_ldir_bc_is_3() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xb0);
        mock_bus.expect_read().with(eq(0x1111)).returning(|_| 0x88);
        mock_bus.expect_read().with(eq(0x2222)).returning(|_| 0x66);
        mock_bus.expect_write().with(eq(0x2222), eq(0x88)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x11;
        cpu.l = 0x11;
        cpu.d = 0x22;
        cpu.e = 0x22;
        cpu.b = 0x00;
        cpu.c = 0x03;
        //        SZ H VNC
        cpu.f = 0b11010011;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x11);
        assert_eq!(cpu.l, 0x12);
        assert_eq!(cpu.d, 0x22);
        assert_eq!(cpu.e, 0x23);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x02);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b11000101);
        assert_eq!(cpu.pc, 0);
        assert_eq!(1 + cpu.t_cycles, 21);
        assert_eq!(disasm, "0000: LDIR");
    }

    #[test]
    fn test_ldir_bc_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xb0);
        mock_bus.expect_read().with(eq(0x1113)).returning(|_| 0xa5);
        mock_bus.expect_read().with(eq(0x2224)).returning(|_| 0xc5);
        mock_bus.expect_write().with(eq(0x2224), eq(0xa5)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x11;
        cpu.l = 0x13;
        cpu.d = 0x22;
        cpu.e = 0x24;
        cpu.b = 0x00;
        cpu.c = 0x01;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x11);
        assert_eq!(cpu.l, 0x14);
        assert_eq!(cpu.d, 0x22);
        assert_eq!(cpu.e, 0x25);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x00);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b11000001);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 16);
        assert_eq!(disasm, "0000: LDIR");
    }

    #[test]
    fn test_ldir_bc_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xb0);
        mock_bus.expect_read().with(eq(0x1114)).returning(|_| 0xe5);
        mock_bus.expect_read().with(eq(0x2225)).returning(|_| 0x9b);
        mock_bus.expect_write().with(eq(0x2225), eq(0xe5)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x11;
        cpu.l = 0x14;
        cpu.d = 0x22;
        cpu.e = 0x25;
        cpu.b = 0x00;
        cpu.c = 0x00;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x11);
        assert_eq!(cpu.l, 0x15);
        assert_eq!(cpu.d, 0x22);
        assert_eq!(cpu.e, 0x26);
        assert_eq!(cpu.b, 0xff);
        assert_eq!(cpu.c, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b11000101);
        assert_eq!(cpu.pc, 0);
        assert_eq!(1 + cpu.t_cycles, 21);
        assert_eq!(disasm, "0000: LDIR");
    }

    #[test]
    fn test_lddr_bc_is_3() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xb8);
        mock_bus.expect_read().with(eq(0x1114)).returning(|_| 0xa5);
        mock_bus.expect_read().with(eq(0x2225)).returning(|_| 0xc5);
        mock_bus.expect_write().with(eq(0x2225), eq(0xa5)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x11;
        cpu.l = 0x14;
        cpu.d = 0x22;
        cpu.e = 0x25;
        cpu.b = 0x00;
        cpu.c = 0x03;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x11);
        assert_eq!(cpu.l, 0x13);
        assert_eq!(cpu.d, 0x22);
        assert_eq!(cpu.e, 0x24);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x02);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b11000001);
        assert_eq!(cpu.pc, 0);
        assert_eq!(1 + cpu.t_cycles, 21);
        assert_eq!(disasm, "0000: LDDR");
    }

    #[test]
    fn test_lddr_bc_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xb8);
        mock_bus.expect_read().with(eq(0x1112)).returning(|_| 0x88);
        mock_bus.expect_read().with(eq(0x2223)).returning(|_| 0x66);
        mock_bus.expect_write().with(eq(0x2223), eq(0x88)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x11;
        cpu.l = 0x12;
        cpu.d = 0x22;
        cpu.e = 0x23;
        cpu.b = 0x00;
        cpu.c = 0x01;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x11);
        assert_eq!(cpu.l, 0x11);
        assert_eq!(cpu.d, 0x22);
        assert_eq!(cpu.e, 0x22);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x00);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b11000001);
        assert_eq!(cpu.pc, 2);
        assert_eq!(1 + cpu.t_cycles, 16);
        assert_eq!(disasm, "0000: LDDR");
    }

    #[test]
    fn test_lddr_bc_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xb8);
        mock_bus.expect_read().with(eq(0x1111)).returning(|_| 0xe5);
        mock_bus.expect_read().with(eq(0x2222)).returning(|_| 0x9b);
        mock_bus.expect_write().with(eq(0x2222), eq(0xe5)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.h = 0x11;
        cpu.l = 0x11;
        cpu.d = 0x22;
        cpu.e = 0x22;
        cpu.b = 0x00;
        cpu.c = 0x00;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.h, 0x11);
        assert_eq!(cpu.l, 0x10);
        assert_eq!(cpu.d, 0x22);
        assert_eq!(cpu.e, 0x21);
        assert_eq!(cpu.b, 0xff);
        assert_eq!(cpu.c, 0xff);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b11000001);
        assert_eq!(cpu.pc, 0);
        assert_eq!(1 + cpu.t_cycles, 21);
        assert_eq!(disasm, "0000: LDDR");
    }

    #[test]
    fn test_ld_ptr_nn_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xed);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x53);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x29);
        mock_bus.expect_read().with(eq(3)).returning(|_| 0xb2);
        mock_bus.expect_write().with(eq(0xb229), eq(0x3a)).return_const(());
        mock_bus.expect_write().with(eq(0xb22a), eq(0x48)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.d = 0x48;
        cpu.e = 0x3a;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 4);
        assert_eq!(1 + cpu.t_cycles, 20);
        assert_eq!(disasm, "0000: LD (B229h), DE");
    }

    #[test]
    fn test_xor_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xee);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x15);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xc3;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xd6);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: XOR 15h");
    }
    
    #[test]
    fn test_rst_28h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xef);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0028);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 28h");
    }

    #[test]
    fn test_ret_p_when_s_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xf0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET P");
    }

    #[test]
    fn test_ret_p_when_s_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xf0);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET P");
    }

    #[test]
    fn test_pop_af() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_jp_p_nn_when_s_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP P, BAADh");
    }

    #[test]
    fn test_jp_p_nn_when_s_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf2);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP P, BAADh");
    }
    
    #[test]
    fn test_di() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf3);

        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.interrupts_enabled = true;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.interrupts_enabled, false);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: DI");
    }

    #[test]
    fn test_call_p_nn_when_s_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xf4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL P, BAADh");
    }

    #[test]
    fn test_call_p_nn_when_s_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xf4);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL P, BAADh");
    }

    #[test]
    fn test_push_af() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    fn test_or_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf6);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xfa);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0x23;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.a, 0xfb);
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000000);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: OR FAh");
    }
    
    #[test]
    fn test_rst_30h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xf7);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0030);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 30h");
    }

    #[test]
    fn test_ret_m_when_s_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xf8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b11010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "BAAD: RET M");
    }

    #[test]
    fn test_ret_m_when_s_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0xbaad)).returning(|_| 0xf8);
        mock_bus.expect_read().with(eq(0x4ffe)).returning(|_| 0x37);
        mock_bus.expect_read().with(eq(0x4fff)).returning(|_| 0x12);
        
        cpu.reset();
        cpu.pc = 0xbaad;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b01010111;
        cpu.sp = 0x4ffe;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaae);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 5);
        assert_eq!(disasm, "BAAD: RET M");
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
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
    
    #[test]
    fn test_jp_m_nn_when_s_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfa);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP M, BAADh");
    }

    #[test]
    fn test_jp_m_nn_when_s_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfa);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "0000: JP M, BAADh");
    }

    #[test]
    fn test_ei() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfb);

        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.interrupts_enabled = false;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cpu.interrupts_enabled, true);
        assert_eq!(1 + cpu.t_cycles, 4);
        assert_eq!(disasm, "0000: EI");
    }

    #[test]
    fn test_call_m_nn_when_s_is_1() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xfc);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b10000000;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0xbaad);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 17);
        assert_eq!(disasm, "1234: CALL M, BAADh");
    }

    #[test]
    fn test_call_m_nn_when_s_is_0() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xfc);
        mock_bus.expect_read().with(eq(0x1235)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(0x1236)).returning(|_| 0xba);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x37)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        //        SZ H VNC
        cpu.f = 0b00000000;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x1237);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(1 + cpu.t_cycles, 10);
        assert_eq!(disasm, "1234: CALL M, BAADh");
    }

    #[test]
    fn test_ld_iy_nn() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfd);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x21);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(3)).returning(|_| 0xba);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.iy, 0xbaad);
        assert_eq!(1 + cpu.t_cycles, 14);
        assert_eq!(disasm, "0000: LD IY, BAADh");
    }

    #[test]
    fn test_dec_ptr_iy_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfd);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x35);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xc6);
        mock_bus.expect_read().with(eq(0x5c00)).returning(|_| 0x83);
        mock_bus.expect_write().with(eq(0x5c00), eq(0x82)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.iy = 0x5c3a;
        //        SZ H VNC
        cpu.f = 0b01010100;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 23);
        assert_eq!(disasm, "0000: DEC (IY-58)");
    }

    #[test]
    fn test_set_b_ptr_iy_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfd);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xcb);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x01);
        mock_bus.expect_read().with(eq(3)).returning(|_| 0xce);
        mock_bus.expect_read().with(eq(0x5c3b)).returning(|_| 0x00);
        mock_bus.expect_write().with(eq(0x5c3b), eq(0x02)).return_const(());
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.iy = 0x5c3a;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(1 + cpu.t_cycles, 23);
        assert_eq!(disasm, "0000: SET 1, (IY+1)");
    }

    #[test]
    fn test_cp_n() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xfe);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x23);
        
        cpu.reset();
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.a = 0xfa;
        //        SZ H VNC
        cpu.f = 0b01010101;
        cpu.clock(&mut mock_bus);
        
        //                  SZ H VNC
        assert_eq!(cpu.f, 0b10000010);
        assert_eq!(1 + cpu.t_cycles, 7);
        assert_eq!(disasm, "0000: CP 23h");
    }

    #[test]
    fn test_rst_38h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().with(eq(0x1234)).returning(|_| 0xff);
        mock_bus.expect_write().with(eq(0x4fff), eq(0x12)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0x35)).return_const(());
        
        cpu.reset();
        cpu.pc = 0x1234;
        let disasm = &cpu.get_next_instructions(&mock_bus, 1)[0];
        cpu.t_cycles = 0;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);
        
        assert_eq!(cpu.pc, 0x0038);
        assert_eq!(cpu.sp, 0x4ffe);
        assert_eq!(1 + cpu.t_cycles, 11);
        assert_eq!(disasm, "1234: RST 38h");
    }
}