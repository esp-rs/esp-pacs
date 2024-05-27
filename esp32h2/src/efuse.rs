#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    pgm_data0: PGM_DATA0,
    pgm_data1: PGM_DATA1,
    pgm_data2: PGM_DATA2,
    pgm_data3: PGM_DATA3,
    pgm_data4: PGM_DATA4,
    pgm_data5: PGM_DATA5,
    pgm_data6: PGM_DATA6,
    pgm_data7: PGM_DATA7,
    pgm_check_value0: PGM_CHECK_VALUE0,
    pgm_check_value1: PGM_CHECK_VALUE1,
    pgm_check_value2: PGM_CHECK_VALUE2,
    rd_wr_dis: RD_WR_DIS,
    rd_repeat_data0: RD_REPEAT_DATA0,
    rd_repeat_data1: RD_REPEAT_DATA1,
    rd_repeat_data2: RD_REPEAT_DATA2,
    rd_repeat_data3: RD_REPEAT_DATA3,
    rd_repeat_data4: RD_REPEAT_DATA4,
    rd_mac_sys_0: RD_MAC_SYS_0,
    rd_mac_sys_1: RD_MAC_SYS_1,
    rd_mac_sys_2: RD_MAC_SYS_2,
    rd_mac_sys_3: RD_MAC_SYS_3,
    rd_mac_sys_4: RD_MAC_SYS_4,
    rd_mac_sys_5: RD_MAC_SYS_5,
    rd_sys_part1_data0: RD_SYS_PART1_DATA0,
    rd_sys_part1_data1: RD_SYS_PART1_DATA1,
    rd_sys_part1_data2: RD_SYS_PART1_DATA2,
    rd_sys_part1_data3: RD_SYS_PART1_DATA3,
    rd_sys_part1_data4: RD_SYS_PART1_DATA4,
    rd_sys_part1_data5: RD_SYS_PART1_DATA5,
    rd_sys_part1_data6: RD_SYS_PART1_DATA6,
    rd_sys_part1_data7: RD_SYS_PART1_DATA7,
    rd_usr_data0: RD_USR_DATA0,
    rd_usr_data1: RD_USR_DATA1,
    rd_usr_data2: RD_USR_DATA2,
    rd_usr_data3: RD_USR_DATA3,
    rd_usr_data4: RD_USR_DATA4,
    rd_usr_data5: RD_USR_DATA5,
    rd_usr_data6: RD_USR_DATA6,
    rd_usr_data7: RD_USR_DATA7,
    rd_key0_data0: RD_KEY0_DATA0,
    rd_key0_data1: RD_KEY0_DATA1,
    rd_key0_data2: RD_KEY0_DATA2,
    rd_key0_data3: RD_KEY0_DATA3,
    rd_key0_data4: RD_KEY0_DATA4,
    rd_key0_data5: RD_KEY0_DATA5,
    rd_key0_data6: RD_KEY0_DATA6,
    rd_key0_data7: RD_KEY0_DATA7,
    rd_key1_data0: RD_KEY1_DATA0,
    rd_key1_data1: RD_KEY1_DATA1,
    rd_key1_data2: RD_KEY1_DATA2,
    rd_key1_data3: RD_KEY1_DATA3,
    rd_key1_data4: RD_KEY1_DATA4,
    rd_key1_data5: RD_KEY1_DATA5,
    rd_key1_data6: RD_KEY1_DATA6,
    rd_key1_data7: RD_KEY1_DATA7,
    rd_key2_data0: RD_KEY2_DATA0,
    rd_key2_data1: RD_KEY2_DATA1,
    rd_key2_data2: RD_KEY2_DATA2,
    rd_key2_data3: RD_KEY2_DATA3,
    rd_key2_data4: RD_KEY2_DATA4,
    rd_key2_data5: RD_KEY2_DATA5,
    rd_key2_data6: RD_KEY2_DATA6,
    rd_key2_data7: RD_KEY2_DATA7,
    rd_key3_data0: RD_KEY3_DATA0,
    rd_key3_data1: RD_KEY3_DATA1,
    rd_key3_data2: RD_KEY3_DATA2,
    rd_key3_data3: RD_KEY3_DATA3,
    rd_key3_data4: RD_KEY3_DATA4,
    rd_key3_data5: RD_KEY3_DATA5,
    rd_key3_data6: RD_KEY3_DATA6,
    rd_key3_data7: RD_KEY3_DATA7,
    rd_key4_data0: RD_KEY4_DATA0,
    rd_key4_data1: RD_KEY4_DATA1,
    rd_key4_data2: RD_KEY4_DATA2,
    rd_key4_data3: RD_KEY4_DATA3,
    rd_key4_data4: RD_KEY4_DATA4,
    rd_key4_data5: RD_KEY4_DATA5,
    rd_key4_data6: RD_KEY4_DATA6,
    rd_key4_data7: RD_KEY4_DATA7,
    rd_key5_data0: RD_KEY5_DATA0,
    rd_key5_data1: RD_KEY5_DATA1,
    rd_key5_data2: RD_KEY5_DATA2,
    rd_key5_data3: RD_KEY5_DATA3,
    rd_key5_data4: RD_KEY5_DATA4,
    rd_key5_data5: RD_KEY5_DATA5,
    rd_key5_data6: RD_KEY5_DATA6,
    rd_key5_data7: RD_KEY5_DATA7,
    rd_sys_part2_data0: RD_SYS_PART2_DATA0,
    rd_sys_part2_data1: RD_SYS_PART2_DATA1,
    rd_sys_part2_data2: RD_SYS_PART2_DATA2,
    rd_sys_part2_data3: RD_SYS_PART2_DATA3,
    rd_sys_part2_data4: RD_SYS_PART2_DATA4,
    rd_sys_part2_data5: RD_SYS_PART2_DATA5,
    rd_sys_part2_data6: RD_SYS_PART2_DATA6,
    rd_sys_part2_data7: RD_SYS_PART2_DATA7,
    rd_repeat_err0: RD_REPEAT_ERR0,
    rd_repeat_err1: RD_REPEAT_ERR1,
    rd_repeat_err2: RD_REPEAT_ERR2,
    rd_repeat_err3: RD_REPEAT_ERR3,
    rd_repeat_err4: RD_REPEAT_ERR4,
    _reserved100: [u8; 0x30],
    rd_rs_err0: RD_RS_ERR0,
    rd_rs_err1: RD_RS_ERR1,
    clk: CLK,
    conf: CONF,
    status: STATUS,
    cmd: CMD,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dac_conf: DAC_CONF,
    rd_tim_conf: RD_TIM_CONF,
    wr_tim_conf1: WR_TIM_CONF1,
    wr_tim_conf2: WR_TIM_CONF2,
    wr_tim_conf0_rs_bypass: WR_TIM_CONF0_RS_BYPASS,
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Register 0 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data0(&self) -> &PGM_DATA0 {
        &self.pgm_data0
    }
    ///0x04 - Register 1 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data1(&self) -> &PGM_DATA1 {
        &self.pgm_data1
    }
    ///0x08 - Register 2 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data2(&self) -> &PGM_DATA2 {
        &self.pgm_data2
    }
    ///0x0c - Register 3 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data3(&self) -> &PGM_DATA3 {
        &self.pgm_data3
    }
    ///0x10 - Register 4 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data4(&self) -> &PGM_DATA4 {
        &self.pgm_data4
    }
    ///0x14 - Register 5 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data5(&self) -> &PGM_DATA5 {
        &self.pgm_data5
    }
    ///0x18 - Register 6 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data6(&self) -> &PGM_DATA6 {
        &self.pgm_data6
    }
    ///0x1c - Register 7 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data7(&self) -> &PGM_DATA7 {
        &self.pgm_data7
    }
    ///0x20 - Register 0 that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value0(&self) -> &PGM_CHECK_VALUE0 {
        &self.pgm_check_value0
    }
    ///0x24 - Register 1 that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value1(&self) -> &PGM_CHECK_VALUE1 {
        &self.pgm_check_value1
    }
    ///0x28 - Register 2 that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value2(&self) -> &PGM_CHECK_VALUE2 {
        &self.pgm_check_value2
    }
    ///0x2c - BLOCK0 data register 0.
    #[inline(always)]
    pub const fn rd_wr_dis(&self) -> &RD_WR_DIS {
        &self.rd_wr_dis
    }
    ///0x30 - BLOCK0 data register 1.
    #[inline(always)]
    pub const fn rd_repeat_data0(&self) -> &RD_REPEAT_DATA0 {
        &self.rd_repeat_data0
    }
    ///0x34 - BLOCK0 data register 2.
    #[inline(always)]
    pub const fn rd_repeat_data1(&self) -> &RD_REPEAT_DATA1 {
        &self.rd_repeat_data1
    }
    ///0x38 - BLOCK0 data register 3.
    #[inline(always)]
    pub const fn rd_repeat_data2(&self) -> &RD_REPEAT_DATA2 {
        &self.rd_repeat_data2
    }
    ///0x3c - BLOCK0 data register 4.
    #[inline(always)]
    pub const fn rd_repeat_data3(&self) -> &RD_REPEAT_DATA3 {
        &self.rd_repeat_data3
    }
    ///0x40 - BLOCK0 data register 5.
    #[inline(always)]
    pub const fn rd_repeat_data4(&self) -> &RD_REPEAT_DATA4 {
        &self.rd_repeat_data4
    }
    ///0x44 - BLOCK1 data register $n.
    #[inline(always)]
    pub const fn rd_mac_sys_0(&self) -> &RD_MAC_SYS_0 {
        &self.rd_mac_sys_0
    }
    ///0x48 - BLOCK1 data register $n.
    #[inline(always)]
    pub const fn rd_mac_sys_1(&self) -> &RD_MAC_SYS_1 {
        &self.rd_mac_sys_1
    }
    ///0x4c - BLOCK1 data register $n.
    #[inline(always)]
    pub const fn rd_mac_sys_2(&self) -> &RD_MAC_SYS_2 {
        &self.rd_mac_sys_2
    }
    ///0x50 - BLOCK1 data register $n.
    #[inline(always)]
    pub const fn rd_mac_sys_3(&self) -> &RD_MAC_SYS_3 {
        &self.rd_mac_sys_3
    }
    ///0x54 - BLOCK1 data register $n.
    #[inline(always)]
    pub const fn rd_mac_sys_4(&self) -> &RD_MAC_SYS_4 {
        &self.rd_mac_sys_4
    }
    ///0x58 - BLOCK1 data register $n.
    #[inline(always)]
    pub const fn rd_mac_sys_5(&self) -> &RD_MAC_SYS_5 {
        &self.rd_mac_sys_5
    }
    ///0x5c - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data0(&self) -> &RD_SYS_PART1_DATA0 {
        &self.rd_sys_part1_data0
    }
    ///0x60 - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data1(&self) -> &RD_SYS_PART1_DATA1 {
        &self.rd_sys_part1_data1
    }
    ///0x64 - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data2(&self) -> &RD_SYS_PART1_DATA2 {
        &self.rd_sys_part1_data2
    }
    ///0x68 - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data3(&self) -> &RD_SYS_PART1_DATA3 {
        &self.rd_sys_part1_data3
    }
    ///0x6c - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data4(&self) -> &RD_SYS_PART1_DATA4 {
        &self.rd_sys_part1_data4
    }
    ///0x70 - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data5(&self) -> &RD_SYS_PART1_DATA5 {
        &self.rd_sys_part1_data5
    }
    ///0x74 - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data6(&self) -> &RD_SYS_PART1_DATA6 {
        &self.rd_sys_part1_data6
    }
    ///0x78 - Register $n of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_part1_data7(&self) -> &RD_SYS_PART1_DATA7 {
        &self.rd_sys_part1_data7
    }
    ///0x7c - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data0(&self) -> &RD_USR_DATA0 {
        &self.rd_usr_data0
    }
    ///0x80 - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data1(&self) -> &RD_USR_DATA1 {
        &self.rd_usr_data1
    }
    ///0x84 - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data2(&self) -> &RD_USR_DATA2 {
        &self.rd_usr_data2
    }
    ///0x88 - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data3(&self) -> &RD_USR_DATA3 {
        &self.rd_usr_data3
    }
    ///0x8c - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data4(&self) -> &RD_USR_DATA4 {
        &self.rd_usr_data4
    }
    ///0x90 - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data5(&self) -> &RD_USR_DATA5 {
        &self.rd_usr_data5
    }
    ///0x94 - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data6(&self) -> &RD_USR_DATA6 {
        &self.rd_usr_data6
    }
    ///0x98 - Register $n of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data7(&self) -> &RD_USR_DATA7 {
        &self.rd_usr_data7
    }
    ///0x9c - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data0(&self) -> &RD_KEY0_DATA0 {
        &self.rd_key0_data0
    }
    ///0xa0 - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data1(&self) -> &RD_KEY0_DATA1 {
        &self.rd_key0_data1
    }
    ///0xa4 - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data2(&self) -> &RD_KEY0_DATA2 {
        &self.rd_key0_data2
    }
    ///0xa8 - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data3(&self) -> &RD_KEY0_DATA3 {
        &self.rd_key0_data3
    }
    ///0xac - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data4(&self) -> &RD_KEY0_DATA4 {
        &self.rd_key0_data4
    }
    ///0xb0 - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data5(&self) -> &RD_KEY0_DATA5 {
        &self.rd_key0_data5
    }
    ///0xb4 - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data6(&self) -> &RD_KEY0_DATA6 {
        &self.rd_key0_data6
    }
    ///0xb8 - Register $n of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data7(&self) -> &RD_KEY0_DATA7 {
        &self.rd_key0_data7
    }
    ///0xbc - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data0(&self) -> &RD_KEY1_DATA0 {
        &self.rd_key1_data0
    }
    ///0xc0 - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data1(&self) -> &RD_KEY1_DATA1 {
        &self.rd_key1_data1
    }
    ///0xc4 - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data2(&self) -> &RD_KEY1_DATA2 {
        &self.rd_key1_data2
    }
    ///0xc8 - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data3(&self) -> &RD_KEY1_DATA3 {
        &self.rd_key1_data3
    }
    ///0xcc - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data4(&self) -> &RD_KEY1_DATA4 {
        &self.rd_key1_data4
    }
    ///0xd0 - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data5(&self) -> &RD_KEY1_DATA5 {
        &self.rd_key1_data5
    }
    ///0xd4 - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data6(&self) -> &RD_KEY1_DATA6 {
        &self.rd_key1_data6
    }
    ///0xd8 - Register $n of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data7(&self) -> &RD_KEY1_DATA7 {
        &self.rd_key1_data7
    }
    ///0xdc - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data0(&self) -> &RD_KEY2_DATA0 {
        &self.rd_key2_data0
    }
    ///0xe0 - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data1(&self) -> &RD_KEY2_DATA1 {
        &self.rd_key2_data1
    }
    ///0xe4 - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data2(&self) -> &RD_KEY2_DATA2 {
        &self.rd_key2_data2
    }
    ///0xe8 - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data3(&self) -> &RD_KEY2_DATA3 {
        &self.rd_key2_data3
    }
    ///0xec - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data4(&self) -> &RD_KEY2_DATA4 {
        &self.rd_key2_data4
    }
    ///0xf0 - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data5(&self) -> &RD_KEY2_DATA5 {
        &self.rd_key2_data5
    }
    ///0xf4 - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data6(&self) -> &RD_KEY2_DATA6 {
        &self.rd_key2_data6
    }
    ///0xf8 - Register $n of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data7(&self) -> &RD_KEY2_DATA7 {
        &self.rd_key2_data7
    }
    ///0xfc - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data0(&self) -> &RD_KEY3_DATA0 {
        &self.rd_key3_data0
    }
    ///0x100 - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data1(&self) -> &RD_KEY3_DATA1 {
        &self.rd_key3_data1
    }
    ///0x104 - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data2(&self) -> &RD_KEY3_DATA2 {
        &self.rd_key3_data2
    }
    ///0x108 - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data3(&self) -> &RD_KEY3_DATA3 {
        &self.rd_key3_data3
    }
    ///0x10c - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data4(&self) -> &RD_KEY3_DATA4 {
        &self.rd_key3_data4
    }
    ///0x110 - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data5(&self) -> &RD_KEY3_DATA5 {
        &self.rd_key3_data5
    }
    ///0x114 - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data6(&self) -> &RD_KEY3_DATA6 {
        &self.rd_key3_data6
    }
    ///0x118 - Register $n of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data7(&self) -> &RD_KEY3_DATA7 {
        &self.rd_key3_data7
    }
    ///0x11c - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data0(&self) -> &RD_KEY4_DATA0 {
        &self.rd_key4_data0
    }
    ///0x120 - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data1(&self) -> &RD_KEY4_DATA1 {
        &self.rd_key4_data1
    }
    ///0x124 - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data2(&self) -> &RD_KEY4_DATA2 {
        &self.rd_key4_data2
    }
    ///0x128 - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data3(&self) -> &RD_KEY4_DATA3 {
        &self.rd_key4_data3
    }
    ///0x12c - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data4(&self) -> &RD_KEY4_DATA4 {
        &self.rd_key4_data4
    }
    ///0x130 - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data5(&self) -> &RD_KEY4_DATA5 {
        &self.rd_key4_data5
    }
    ///0x134 - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data6(&self) -> &RD_KEY4_DATA6 {
        &self.rd_key4_data6
    }
    ///0x138 - Register $n of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data7(&self) -> &RD_KEY4_DATA7 {
        &self.rd_key4_data7
    }
    ///0x13c - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data0(&self) -> &RD_KEY5_DATA0 {
        &self.rd_key5_data0
    }
    ///0x140 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data1(&self) -> &RD_KEY5_DATA1 {
        &self.rd_key5_data1
    }
    ///0x144 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data2(&self) -> &RD_KEY5_DATA2 {
        &self.rd_key5_data2
    }
    ///0x148 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data3(&self) -> &RD_KEY5_DATA3 {
        &self.rd_key5_data3
    }
    ///0x14c - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data4(&self) -> &RD_KEY5_DATA4 {
        &self.rd_key5_data4
    }
    ///0x150 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data5(&self) -> &RD_KEY5_DATA5 {
        &self.rd_key5_data5
    }
    ///0x154 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data6(&self) -> &RD_KEY5_DATA6 {
        &self.rd_key5_data6
    }
    ///0x158 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data7(&self) -> &RD_KEY5_DATA7 {
        &self.rd_key5_data7
    }
    ///0x15c - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data0(&self) -> &RD_SYS_PART2_DATA0 {
        &self.rd_sys_part2_data0
    }
    ///0x160 - Register $n of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_sys_part2_data1(&self) -> &RD_SYS_PART2_DATA1 {
        &self.rd_sys_part2_data1
    }
    ///0x164 - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data2(&self) -> &RD_SYS_PART2_DATA2 {
        &self.rd_sys_part2_data2
    }
    ///0x168 - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data3(&self) -> &RD_SYS_PART2_DATA3 {
        &self.rd_sys_part2_data3
    }
    ///0x16c - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data4(&self) -> &RD_SYS_PART2_DATA4 {
        &self.rd_sys_part2_data4
    }
    ///0x170 - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data5(&self) -> &RD_SYS_PART2_DATA5 {
        &self.rd_sys_part2_data5
    }
    ///0x174 - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data6(&self) -> &RD_SYS_PART2_DATA6 {
        &self.rd_sys_part2_data6
    }
    ///0x178 - Register $n of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_part2_data7(&self) -> &RD_SYS_PART2_DATA7 {
        &self.rd_sys_part2_data7
    }
    ///0x17c - Programming error record register 0 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err0(&self) -> &RD_REPEAT_ERR0 {
        &self.rd_repeat_err0
    }
    ///0x180 - Programming error record register 1 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err1(&self) -> &RD_REPEAT_ERR1 {
        &self.rd_repeat_err1
    }
    ///0x184 - Programming error record register 2 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err2(&self) -> &RD_REPEAT_ERR2 {
        &self.rd_repeat_err2
    }
    ///0x188 - Programming error record register 3 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err3(&self) -> &RD_REPEAT_ERR3 {
        &self.rd_repeat_err3
    }
    ///0x18c - Programming error record register 4 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err4(&self) -> &RD_REPEAT_ERR4 {
        &self.rd_repeat_err4
    }
    ///0x1c0 - Programming error record register 0 of BLOCK1-10.
    #[inline(always)]
    pub const fn rd_rs_err0(&self) -> &RD_RS_ERR0 {
        &self.rd_rs_err0
    }
    ///0x1c4 - Programming error record register 1 of BLOCK1-10.
    #[inline(always)]
    pub const fn rd_rs_err1(&self) -> &RD_RS_ERR1 {
        &self.rd_rs_err1
    }
    ///0x1c8 - eFuse clcok configuration register.
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x1cc - eFuse operation mode configuraiton register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x1d0 - eFuse status register.
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x1d4 - eFuse command register.
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x1d8 - eFuse raw interrupt register.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x1dc - eFuse interrupt status register.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x1e0 - eFuse interrupt enable register.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x1e4 - eFuse interrupt clear register.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x1e8 - Controls the eFuse programming voltage.
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DAC_CONF {
        &self.dac_conf
    }
    ///0x1ec - Configures read timing parameters.
    #[inline(always)]
    pub const fn rd_tim_conf(&self) -> &RD_TIM_CONF {
        &self.rd_tim_conf
    }
    ///0x1f0 - Configurarion register 1 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf1(&self) -> &WR_TIM_CONF1 {
        &self.wr_tim_conf1
    }
    ///0x1f4 - Configurarion register 2 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf2(&self) -> &WR_TIM_CONF2 {
        &self.wr_tim_conf2
    }
    ///0x1f8 - Configurarion register0 of eFuse programming time parameters and rs bypass operation.
    #[inline(always)]
    pub const fn wr_tim_conf0_rs_bypass(&self) -> &WR_TIM_CONF0_RS_BYPASS {
        &self.wr_tim_conf0_rs_bypass
    }
    ///0x1fc - eFuse version register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**PGM_DATA0 (rw) register accessor: Register 0 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data0`] module*/
pub type PGM_DATA0 = crate::Reg<pgm_data0::PGM_DATA0_SPEC>;
///Register 0 that stores data to be programmed.
pub mod pgm_data0;
/**PGM_DATA1 (rw) register accessor: Register 1 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data1`] module*/
pub type PGM_DATA1 = crate::Reg<pgm_data1::PGM_DATA1_SPEC>;
///Register 1 that stores data to be programmed.
pub mod pgm_data1;
/**PGM_DATA2 (rw) register accessor: Register 2 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data2`] module*/
pub type PGM_DATA2 = crate::Reg<pgm_data2::PGM_DATA2_SPEC>;
///Register 2 that stores data to be programmed.
pub mod pgm_data2;
/**PGM_DATA3 (rw) register accessor: Register 3 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data3`] module*/
pub type PGM_DATA3 = crate::Reg<pgm_data3::PGM_DATA3_SPEC>;
///Register 3 that stores data to be programmed.
pub mod pgm_data3;
/**PGM_DATA4 (rw) register accessor: Register 4 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data4`] module*/
pub type PGM_DATA4 = crate::Reg<pgm_data4::PGM_DATA4_SPEC>;
///Register 4 that stores data to be programmed.
pub mod pgm_data4;
/**PGM_DATA5 (rw) register accessor: Register 5 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data5`] module*/
pub type PGM_DATA5 = crate::Reg<pgm_data5::PGM_DATA5_SPEC>;
///Register 5 that stores data to be programmed.
pub mod pgm_data5;
/**PGM_DATA6 (rw) register accessor: Register 6 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data6`] module*/
pub type PGM_DATA6 = crate::Reg<pgm_data6::PGM_DATA6_SPEC>;
///Register 6 that stores data to be programmed.
pub mod pgm_data6;
/**PGM_DATA7 (rw) register accessor: Register 7 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data7`] module*/
pub type PGM_DATA7 = crate::Reg<pgm_data7::PGM_DATA7_SPEC>;
///Register 7 that stores data to be programmed.
pub mod pgm_data7;
/**PGM_CHECK_VALUE0 (rw) register accessor: Register 0 that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value0`] module*/
pub type PGM_CHECK_VALUE0 = crate::Reg<pgm_check_value0::PGM_CHECK_VALUE0_SPEC>;
///Register 0 that stores the RS code to be programmed.
pub mod pgm_check_value0;
/**PGM_CHECK_VALUE1 (rw) register accessor: Register 1 that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value1`] module*/
pub type PGM_CHECK_VALUE1 = crate::Reg<pgm_check_value1::PGM_CHECK_VALUE1_SPEC>;
///Register 1 that stores the RS code to be programmed.
pub mod pgm_check_value1;
/**PGM_CHECK_VALUE2 (rw) register accessor: Register 2 that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value2`] module*/
pub type PGM_CHECK_VALUE2 = crate::Reg<pgm_check_value2::PGM_CHECK_VALUE2_SPEC>;
///Register 2 that stores the RS code to be programmed.
pub mod pgm_check_value2;
/**RD_WR_DIS (r) register accessor: BLOCK0 data register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_wr_dis`] module*/
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
///BLOCK0 data register 0.
pub mod rd_wr_dis;
/**RD_REPEAT_DATA0 (r) register accessor: BLOCK0 data register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data0`] module*/
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
///BLOCK0 data register 1.
pub mod rd_repeat_data0;
/**RD_REPEAT_DATA1 (r) register accessor: BLOCK0 data register 2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data1`] module*/
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
///BLOCK0 data register 2.
pub mod rd_repeat_data1;
/**RD_REPEAT_DATA2 (r) register accessor: BLOCK0 data register 3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data2`] module*/
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
///BLOCK0 data register 3.
pub mod rd_repeat_data2;
/**RD_REPEAT_DATA3 (r) register accessor: BLOCK0 data register 4.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data3`] module*/
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
///BLOCK0 data register 4.
pub mod rd_repeat_data3;
/**RD_REPEAT_DATA4 (r) register accessor: BLOCK0 data register 5.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data4`] module*/
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
///BLOCK0 data register 5.
pub mod rd_repeat_data4;
/**RD_MAC_SYS_0 (r) register accessor: BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_sys_0`] module*/
pub type RD_MAC_SYS_0 = crate::Reg<rd_mac_sys_0::RD_MAC_SYS_0_SPEC>;
///BLOCK1 data register $n.
pub mod rd_mac_sys_0;
/**RD_MAC_SYS_1 (r) register accessor: BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_sys_1`] module*/
pub type RD_MAC_SYS_1 = crate::Reg<rd_mac_sys_1::RD_MAC_SYS_1_SPEC>;
///BLOCK1 data register $n.
pub mod rd_mac_sys_1;
/**RD_MAC_SYS_2 (r) register accessor: BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_sys_2`] module*/
pub type RD_MAC_SYS_2 = crate::Reg<rd_mac_sys_2::RD_MAC_SYS_2_SPEC>;
///BLOCK1 data register $n.
pub mod rd_mac_sys_2;
/**RD_MAC_SYS_3 (r) register accessor: BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_sys_3`] module*/
pub type RD_MAC_SYS_3 = crate::Reg<rd_mac_sys_3::RD_MAC_SYS_3_SPEC>;
///BLOCK1 data register $n.
pub mod rd_mac_sys_3;
/**RD_MAC_SYS_4 (r) register accessor: BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_sys_4`] module*/
pub type RD_MAC_SYS_4 = crate::Reg<rd_mac_sys_4::RD_MAC_SYS_4_SPEC>;
///BLOCK1 data register $n.
pub mod rd_mac_sys_4;
/**RD_MAC_SYS_5 (r) register accessor: BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_sys_5`] module*/
pub type RD_MAC_SYS_5 = crate::Reg<rd_mac_sys_5::RD_MAC_SYS_5_SPEC>;
///BLOCK1 data register $n.
pub mod rd_mac_sys_5;
/**RD_SYS_PART1_DATA0 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data0`] module*/
pub type RD_SYS_PART1_DATA0 = crate::Reg<rd_sys_part1_data0::RD_SYS_PART1_DATA0_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data0;
/**RD_SYS_PART1_DATA1 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data1`] module*/
pub type RD_SYS_PART1_DATA1 = crate::Reg<rd_sys_part1_data1::RD_SYS_PART1_DATA1_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data1;
/**RD_SYS_PART1_DATA2 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data2`] module*/
pub type RD_SYS_PART1_DATA2 = crate::Reg<rd_sys_part1_data2::RD_SYS_PART1_DATA2_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data2;
/**RD_SYS_PART1_DATA3 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data3`] module*/
pub type RD_SYS_PART1_DATA3 = crate::Reg<rd_sys_part1_data3::RD_SYS_PART1_DATA3_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data3;
/**RD_SYS_PART1_DATA4 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data4`] module*/
pub type RD_SYS_PART1_DATA4 = crate::Reg<rd_sys_part1_data4::RD_SYS_PART1_DATA4_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data4;
/**RD_SYS_PART1_DATA5 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data5`] module*/
pub type RD_SYS_PART1_DATA5 = crate::Reg<rd_sys_part1_data5::RD_SYS_PART1_DATA5_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data5;
/**RD_SYS_PART1_DATA6 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data6`] module*/
pub type RD_SYS_PART1_DATA6 = crate::Reg<rd_sys_part1_data6::RD_SYS_PART1_DATA6_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data6;
/**RD_SYS_PART1_DATA7 (r) register accessor: Register $n of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part1_data7`] module*/
pub type RD_SYS_PART1_DATA7 = crate::Reg<rd_sys_part1_data7::RD_SYS_PART1_DATA7_SPEC>;
///Register $n of BLOCK2 (system).
pub mod rd_sys_part1_data7;
/**RD_USR_DATA0 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data0`] module*/
pub type RD_USR_DATA0 = crate::Reg<rd_usr_data0::RD_USR_DATA0_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data0;
/**RD_USR_DATA1 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data1`] module*/
pub type RD_USR_DATA1 = crate::Reg<rd_usr_data1::RD_USR_DATA1_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data1;
/**RD_USR_DATA2 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data2`] module*/
pub type RD_USR_DATA2 = crate::Reg<rd_usr_data2::RD_USR_DATA2_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data2;
/**RD_USR_DATA3 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data3`] module*/
pub type RD_USR_DATA3 = crate::Reg<rd_usr_data3::RD_USR_DATA3_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data3;
/**RD_USR_DATA4 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data4`] module*/
pub type RD_USR_DATA4 = crate::Reg<rd_usr_data4::RD_USR_DATA4_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data4;
/**RD_USR_DATA5 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data5`] module*/
pub type RD_USR_DATA5 = crate::Reg<rd_usr_data5::RD_USR_DATA5_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data5;
/**RD_USR_DATA6 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data6`] module*/
pub type RD_USR_DATA6 = crate::Reg<rd_usr_data6::RD_USR_DATA6_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data6;
/**RD_USR_DATA7 (r) register accessor: Register $n of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data7`] module*/
pub type RD_USR_DATA7 = crate::Reg<rd_usr_data7::RD_USR_DATA7_SPEC>;
///Register $n of BLOCK3 (user).
pub mod rd_usr_data7;
/**RD_KEY0_DATA0 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data0`] module*/
pub type RD_KEY0_DATA0 = crate::Reg<rd_key0_data0::RD_KEY0_DATA0_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data0;
/**RD_KEY0_DATA1 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data1`] module*/
pub type RD_KEY0_DATA1 = crate::Reg<rd_key0_data1::RD_KEY0_DATA1_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data1;
/**RD_KEY0_DATA2 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data2`] module*/
pub type RD_KEY0_DATA2 = crate::Reg<rd_key0_data2::RD_KEY0_DATA2_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data2;
/**RD_KEY0_DATA3 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data3`] module*/
pub type RD_KEY0_DATA3 = crate::Reg<rd_key0_data3::RD_KEY0_DATA3_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data3;
/**RD_KEY0_DATA4 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data4`] module*/
pub type RD_KEY0_DATA4 = crate::Reg<rd_key0_data4::RD_KEY0_DATA4_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data4;
/**RD_KEY0_DATA5 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data5`] module*/
pub type RD_KEY0_DATA5 = crate::Reg<rd_key0_data5::RD_KEY0_DATA5_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data5;
/**RD_KEY0_DATA6 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data6`] module*/
pub type RD_KEY0_DATA6 = crate::Reg<rd_key0_data6::RD_KEY0_DATA6_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data6;
/**RD_KEY0_DATA7 (r) register accessor: Register $n of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data7`] module*/
pub type RD_KEY0_DATA7 = crate::Reg<rd_key0_data7::RD_KEY0_DATA7_SPEC>;
///Register $n of BLOCK4 (KEY0).
pub mod rd_key0_data7;
/**RD_KEY1_DATA0 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data0`] module*/
pub type RD_KEY1_DATA0 = crate::Reg<rd_key1_data0::RD_KEY1_DATA0_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data0;
/**RD_KEY1_DATA1 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data1`] module*/
pub type RD_KEY1_DATA1 = crate::Reg<rd_key1_data1::RD_KEY1_DATA1_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data1;
/**RD_KEY1_DATA2 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data2`] module*/
pub type RD_KEY1_DATA2 = crate::Reg<rd_key1_data2::RD_KEY1_DATA2_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data2;
/**RD_KEY1_DATA3 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data3`] module*/
pub type RD_KEY1_DATA3 = crate::Reg<rd_key1_data3::RD_KEY1_DATA3_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data3;
/**RD_KEY1_DATA4 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data4`] module*/
pub type RD_KEY1_DATA4 = crate::Reg<rd_key1_data4::RD_KEY1_DATA4_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data4;
/**RD_KEY1_DATA5 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data5`] module*/
pub type RD_KEY1_DATA5 = crate::Reg<rd_key1_data5::RD_KEY1_DATA5_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data5;
/**RD_KEY1_DATA6 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data6`] module*/
pub type RD_KEY1_DATA6 = crate::Reg<rd_key1_data6::RD_KEY1_DATA6_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data6;
/**RD_KEY1_DATA7 (r) register accessor: Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data7`] module*/
pub type RD_KEY1_DATA7 = crate::Reg<rd_key1_data7::RD_KEY1_DATA7_SPEC>;
///Register $n of BLOCK5 (KEY1).
pub mod rd_key1_data7;
/**RD_KEY2_DATA0 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data0`] module*/
pub type RD_KEY2_DATA0 = crate::Reg<rd_key2_data0::RD_KEY2_DATA0_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data0;
/**RD_KEY2_DATA1 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data1`] module*/
pub type RD_KEY2_DATA1 = crate::Reg<rd_key2_data1::RD_KEY2_DATA1_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data1;
/**RD_KEY2_DATA2 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data2`] module*/
pub type RD_KEY2_DATA2 = crate::Reg<rd_key2_data2::RD_KEY2_DATA2_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data2;
/**RD_KEY2_DATA3 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data3`] module*/
pub type RD_KEY2_DATA3 = crate::Reg<rd_key2_data3::RD_KEY2_DATA3_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data3;
/**RD_KEY2_DATA4 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data4`] module*/
pub type RD_KEY2_DATA4 = crate::Reg<rd_key2_data4::RD_KEY2_DATA4_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data4;
/**RD_KEY2_DATA5 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data5`] module*/
pub type RD_KEY2_DATA5 = crate::Reg<rd_key2_data5::RD_KEY2_DATA5_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data5;
/**RD_KEY2_DATA6 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data6`] module*/
pub type RD_KEY2_DATA6 = crate::Reg<rd_key2_data6::RD_KEY2_DATA6_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data6;
/**RD_KEY2_DATA7 (r) register accessor: Register $n of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data7`] module*/
pub type RD_KEY2_DATA7 = crate::Reg<rd_key2_data7::RD_KEY2_DATA7_SPEC>;
///Register $n of BLOCK6 (KEY2).
pub mod rd_key2_data7;
/**RD_KEY3_DATA0 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data0`] module*/
pub type RD_KEY3_DATA0 = crate::Reg<rd_key3_data0::RD_KEY3_DATA0_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data0;
/**RD_KEY3_DATA1 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data1`] module*/
pub type RD_KEY3_DATA1 = crate::Reg<rd_key3_data1::RD_KEY3_DATA1_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data1;
/**RD_KEY3_DATA2 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data2`] module*/
pub type RD_KEY3_DATA2 = crate::Reg<rd_key3_data2::RD_KEY3_DATA2_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data2;
/**RD_KEY3_DATA3 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data3`] module*/
pub type RD_KEY3_DATA3 = crate::Reg<rd_key3_data3::RD_KEY3_DATA3_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data3;
/**RD_KEY3_DATA4 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data4`] module*/
pub type RD_KEY3_DATA4 = crate::Reg<rd_key3_data4::RD_KEY3_DATA4_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data4;
/**RD_KEY3_DATA5 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data5`] module*/
pub type RD_KEY3_DATA5 = crate::Reg<rd_key3_data5::RD_KEY3_DATA5_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data5;
/**RD_KEY3_DATA6 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data6`] module*/
pub type RD_KEY3_DATA6 = crate::Reg<rd_key3_data6::RD_KEY3_DATA6_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data6;
/**RD_KEY3_DATA7 (r) register accessor: Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data7`] module*/
pub type RD_KEY3_DATA7 = crate::Reg<rd_key3_data7::RD_KEY3_DATA7_SPEC>;
///Register $n of BLOCK7 (KEY3).
pub mod rd_key3_data7;
/**RD_KEY4_DATA0 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data0`] module*/
pub type RD_KEY4_DATA0 = crate::Reg<rd_key4_data0::RD_KEY4_DATA0_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data0;
/**RD_KEY4_DATA1 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data1`] module*/
pub type RD_KEY4_DATA1 = crate::Reg<rd_key4_data1::RD_KEY4_DATA1_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data1;
/**RD_KEY4_DATA2 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data2`] module*/
pub type RD_KEY4_DATA2 = crate::Reg<rd_key4_data2::RD_KEY4_DATA2_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data2;
/**RD_KEY4_DATA3 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data3`] module*/
pub type RD_KEY4_DATA3 = crate::Reg<rd_key4_data3::RD_KEY4_DATA3_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data3;
/**RD_KEY4_DATA4 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data4`] module*/
pub type RD_KEY4_DATA4 = crate::Reg<rd_key4_data4::RD_KEY4_DATA4_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data4;
/**RD_KEY4_DATA5 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data5`] module*/
pub type RD_KEY4_DATA5 = crate::Reg<rd_key4_data5::RD_KEY4_DATA5_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data5;
/**RD_KEY4_DATA6 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data6`] module*/
pub type RD_KEY4_DATA6 = crate::Reg<rd_key4_data6::RD_KEY4_DATA6_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data6;
/**RD_KEY4_DATA7 (r) register accessor: Register $n of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data7`] module*/
pub type RD_KEY4_DATA7 = crate::Reg<rd_key4_data7::RD_KEY4_DATA7_SPEC>;
///Register $n of BLOCK8 (KEY4).
pub mod rd_key4_data7;
/**RD_KEY5_DATA0 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data0`] module*/
pub type RD_KEY5_DATA0 = crate::Reg<rd_key5_data0::RD_KEY5_DATA0_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data0;
/**RD_KEY5_DATA1 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data1`] module*/
pub type RD_KEY5_DATA1 = crate::Reg<rd_key5_data1::RD_KEY5_DATA1_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data1;
/**RD_KEY5_DATA2 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data2`] module*/
pub type RD_KEY5_DATA2 = crate::Reg<rd_key5_data2::RD_KEY5_DATA2_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data2;
/**RD_KEY5_DATA3 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data3`] module*/
pub type RD_KEY5_DATA3 = crate::Reg<rd_key5_data3::RD_KEY5_DATA3_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data3;
/**RD_KEY5_DATA4 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data4`] module*/
pub type RD_KEY5_DATA4 = crate::Reg<rd_key5_data4::RD_KEY5_DATA4_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data4;
/**RD_KEY5_DATA5 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data5`] module*/
pub type RD_KEY5_DATA5 = crate::Reg<rd_key5_data5::RD_KEY5_DATA5_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data5;
/**RD_KEY5_DATA6 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data6`] module*/
pub type RD_KEY5_DATA6 = crate::Reg<rd_key5_data6::RD_KEY5_DATA6_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data6;
/**RD_KEY5_DATA7 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data7`] module*/
pub type RD_KEY5_DATA7 = crate::Reg<rd_key5_data7::RD_KEY5_DATA7_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_key5_data7;
/**RD_SYS_PART2_DATA0 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data0`] module*/
pub type RD_SYS_PART2_DATA0 = crate::Reg<rd_sys_part2_data0::RD_SYS_PART2_DATA0_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data0;
/**RD_SYS_PART2_DATA1 (r) register accessor: Register $n of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data1`] module*/
pub type RD_SYS_PART2_DATA1 = crate::Reg<rd_sys_part2_data1::RD_SYS_PART2_DATA1_SPEC>;
///Register $n of BLOCK9 (KEY5).
pub mod rd_sys_part2_data1;
/**RD_SYS_PART2_DATA2 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data2`] module*/
pub type RD_SYS_PART2_DATA2 = crate::Reg<rd_sys_part2_data2::RD_SYS_PART2_DATA2_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data2;
/**RD_SYS_PART2_DATA3 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data3`] module*/
pub type RD_SYS_PART2_DATA3 = crate::Reg<rd_sys_part2_data3::RD_SYS_PART2_DATA3_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data3;
/**RD_SYS_PART2_DATA4 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data4`] module*/
pub type RD_SYS_PART2_DATA4 = crate::Reg<rd_sys_part2_data4::RD_SYS_PART2_DATA4_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data4;
/**RD_SYS_PART2_DATA5 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data5`] module*/
pub type RD_SYS_PART2_DATA5 = crate::Reg<rd_sys_part2_data5::RD_SYS_PART2_DATA5_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data5;
/**RD_SYS_PART2_DATA6 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data6`] module*/
pub type RD_SYS_PART2_DATA6 = crate::Reg<rd_sys_part2_data6::RD_SYS_PART2_DATA6_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data6;
/**RD_SYS_PART2_DATA7 (r) register accessor: Register $n of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_part2_data7`] module*/
pub type RD_SYS_PART2_DATA7 = crate::Reg<rd_sys_part2_data7::RD_SYS_PART2_DATA7_SPEC>;
///Register $n of BLOCK10 (system).
pub mod rd_sys_part2_data7;
/**RD_REPEAT_ERR0 (r) register accessor: Programming error record register 0 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err0`] module*/
pub type RD_REPEAT_ERR0 = crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>;
///Programming error record register 0 of BLOCK0.
pub mod rd_repeat_err0;
/**RD_REPEAT_ERR1 (r) register accessor: Programming error record register 1 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err1`] module*/
pub type RD_REPEAT_ERR1 = crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>;
///Programming error record register 1 of BLOCK0.
pub mod rd_repeat_err1;
/**RD_REPEAT_ERR2 (r) register accessor: Programming error record register 2 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err2`] module*/
pub type RD_REPEAT_ERR2 = crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>;
///Programming error record register 2 of BLOCK0.
pub mod rd_repeat_err2;
/**RD_REPEAT_ERR3 (r) register accessor: Programming error record register 3 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err3`] module*/
pub type RD_REPEAT_ERR3 = crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>;
///Programming error record register 3 of BLOCK0.
pub mod rd_repeat_err3;
/**RD_REPEAT_ERR4 (r) register accessor: Programming error record register 4 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err4`] module*/
pub type RD_REPEAT_ERR4 = crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>;
///Programming error record register 4 of BLOCK0.
pub mod rd_repeat_err4;
/**RD_RS_ERR0 (r) register accessor: Programming error record register 0 of BLOCK1-10.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_rs_err0`] module*/
pub type RD_RS_ERR0 = crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>;
///Programming error record register 0 of BLOCK1-10.
pub mod rd_rs_err0;
/**RD_RS_ERR1 (r) register accessor: Programming error record register 1 of BLOCK1-10.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_rs_err1`] module*/
pub type RD_RS_ERR1 = crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>;
///Programming error record register 1 of BLOCK1-10.
pub mod rd_rs_err1;
/**CLK (rw) register accessor: eFuse clcok configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///eFuse clcok configuration register.
pub mod clk;
/**CONF (rw) register accessor: eFuse operation mode configuraiton register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///eFuse operation mode configuraiton register
pub mod conf;
/**STATUS (r) register accessor: eFuse status register.

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///eFuse status register.
pub mod status;
/**CMD (rw) register accessor: eFuse command register.

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///eFuse command register.
pub mod cmd;
/**INT_RAW (r) register accessor: eFuse raw interrupt register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///eFuse raw interrupt register.
pub mod int_raw;
/**INT_ST (r) register accessor: eFuse interrupt status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///eFuse interrupt status register.
pub mod int_st;
/**INT_ENA (rw) register accessor: eFuse interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///eFuse interrupt enable register.
pub mod int_ena;
/**INT_CLR (w) register accessor: eFuse interrupt clear register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///eFuse interrupt clear register.
pub mod int_clr;
/**DAC_CONF (rw) register accessor: Controls the eFuse programming voltage.

You can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dac_conf`] module*/
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
///Controls the eFuse programming voltage.
pub mod dac_conf;
/**RD_TIM_CONF (rw) register accessor: Configures read timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_tim_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_tim_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_tim_conf`] module*/
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
///Configures read timing parameters.
pub mod rd_tim_conf;
/**WR_TIM_CONF1 (rw) register accessor: Configurarion register 1 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf1`] module*/
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
///Configurarion register 1 of eFuse programming timing parameters.
pub mod wr_tim_conf1;
/**WR_TIM_CONF2 (rw) register accessor: Configurarion register 2 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf2`] module*/
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
///Configurarion register 2 of eFuse programming timing parameters.
pub mod wr_tim_conf2;
/**WR_TIM_CONF0_RS_BYPASS (rw) register accessor: Configurarion register0 of eFuse programming time parameters and rs bypass operation.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0_rs_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0_rs_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf0_rs_bypass`] module*/
pub type WR_TIM_CONF0_RS_BYPASS = crate::Reg<wr_tim_conf0_rs_bypass::WR_TIM_CONF0_RS_BYPASS_SPEC>;
///Configurarion register0 of eFuse programming time parameters and rs bypass operation.
pub mod wr_tim_conf0_rs_bypass;
/**DATE (rw) register accessor: eFuse version register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///eFuse version register.
pub mod date;
