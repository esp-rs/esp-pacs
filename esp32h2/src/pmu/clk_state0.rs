///Register `CLK_STATE0` reader
pub type R = crate::R<CLK_STATE0_SPEC>;
///Field `STABLE_XPD_BBPLL_STATE` reader - need_des
pub type STABLE_XPD_BBPLL_STATE_R = crate::BitReader;
///Field `STABLE_XPD_XTAL_STATE` reader - need_des
pub type STABLE_XPD_XTAL_STATE_R = crate::BitReader;
///Field `SYS_CLK_SLP_SEL_STATE` reader - need_des
pub type SYS_CLK_SLP_SEL_STATE_R = crate::BitReader;
///Field `SYS_CLK_SEL_STATE` reader - need_des
pub type SYS_CLK_SEL_STATE_R = crate::FieldReader;
///Field `SYS_CLK_NO_DIV_STATE` reader - need_des
pub type SYS_CLK_NO_DIV_STATE_R = crate::BitReader;
///Field `ICG_SYS_CLK_EN_STATE` reader - need_des
pub type ICG_SYS_CLK_EN_STATE_R = crate::BitReader;
///Field `ICG_MODEM_SWITCH_STATE` reader - need_des
pub type ICG_MODEM_SWITCH_STATE_R = crate::BitReader;
///Field `ICG_MODEM_CODE_STATE` reader - need_des
pub type ICG_MODEM_CODE_STATE_R = crate::FieldReader;
///Field `ICG_SLP_SEL_STATE` reader - need_des
pub type ICG_SLP_SEL_STATE_R = crate::BitReader;
///Field `ICG_GLOBAL_XTAL_STATE` reader - need_des
pub type ICG_GLOBAL_XTAL_STATE_R = crate::BitReader;
///Field `ICG_GLOBAL_PLL_STATE` reader - need_des
pub type ICG_GLOBAL_PLL_STATE_R = crate::BitReader;
///Field `ANA_I2C_ISO_EN_STATE` reader - need_des
pub type ANA_I2C_ISO_EN_STATE_R = crate::BitReader;
///Field `ANA_I2C_RETENTION_STATE` reader - need_des
pub type ANA_I2C_RETENTION_STATE_R = crate::BitReader;
///Field `ANA_XPD_BB_I2C_STATE` reader - need_des
pub type ANA_XPD_BB_I2C_STATE_R = crate::BitReader;
///Field `ANA_XPD_BBPLL_I2C_STATE` reader - need_des
pub type ANA_XPD_BBPLL_I2C_STATE_R = crate::BitReader;
///Field `ANA_XPD_BBPLL_STATE` reader - need_des
pub type ANA_XPD_BBPLL_STATE_R = crate::BitReader;
///Field `ANA_XPD_XTAL_STATE` reader - need_des
pub type ANA_XPD_XTAL_STATE_R = crate::BitReader;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn stable_xpd_bbpll_state(&self) -> STABLE_XPD_BBPLL_STATE_R {
        STABLE_XPD_BBPLL_STATE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn stable_xpd_xtal_state(&self) -> STABLE_XPD_XTAL_STATE_R {
        STABLE_XPD_XTAL_STATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 15 - need_des
    #[inline(always)]
    pub fn sys_clk_slp_sel_state(&self) -> SYS_CLK_SLP_SEL_STATE_R {
        SYS_CLK_SLP_SEL_STATE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - need_des
    #[inline(always)]
    pub fn sys_clk_sel_state(&self) -> SYS_CLK_SEL_STATE_R {
        SYS_CLK_SEL_STATE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - need_des
    #[inline(always)]
    pub fn sys_clk_no_div_state(&self) -> SYS_CLK_NO_DIV_STATE_R {
        SYS_CLK_NO_DIV_STATE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - need_des
    #[inline(always)]
    pub fn icg_sys_clk_en_state(&self) -> ICG_SYS_CLK_EN_STATE_R {
        ICG_SYS_CLK_EN_STATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - need_des
    #[inline(always)]
    pub fn icg_modem_switch_state(&self) -> ICG_MODEM_SWITCH_STATE_R {
        ICG_MODEM_SWITCH_STATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - need_des
    #[inline(always)]
    pub fn icg_modem_code_state(&self) -> ICG_MODEM_CODE_STATE_R {
        ICG_MODEM_CODE_STATE_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - need_des
    #[inline(always)]
    pub fn icg_slp_sel_state(&self) -> ICG_SLP_SEL_STATE_R {
        ICG_SLP_SEL_STATE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - need_des
    #[inline(always)]
    pub fn icg_global_xtal_state(&self) -> ICG_GLOBAL_XTAL_STATE_R {
        ICG_GLOBAL_XTAL_STATE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - need_des
    #[inline(always)]
    pub fn icg_global_pll_state(&self) -> ICG_GLOBAL_PLL_STATE_R {
        ICG_GLOBAL_PLL_STATE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - need_des
    #[inline(always)]
    pub fn ana_i2c_iso_en_state(&self) -> ANA_I2C_ISO_EN_STATE_R {
        ANA_I2C_ISO_EN_STATE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn ana_i2c_retention_state(&self) -> ANA_I2C_RETENTION_STATE_R {
        ANA_I2C_RETENTION_STATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn ana_xpd_bb_i2c_state(&self) -> ANA_XPD_BB_I2C_STATE_R {
        ANA_XPD_BB_I2C_STATE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn ana_xpd_bbpll_i2c_state(&self) -> ANA_XPD_BBPLL_I2C_STATE_R {
        ANA_XPD_BBPLL_I2C_STATE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn ana_xpd_bbpll_state(&self) -> ANA_XPD_BBPLL_STATE_R {
        ANA_XPD_BBPLL_STATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn ana_xpd_xtal_state(&self) -> ANA_XPD_XTAL_STATE_R {
        ANA_XPD_XTAL_STATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_STATE0")
            .field("stable_xpd_bbpll_state", &self.stable_xpd_bbpll_state())
            .field("stable_xpd_xtal_state", &self.stable_xpd_xtal_state())
            .field("sys_clk_slp_sel_state", &self.sys_clk_slp_sel_state())
            .field("sys_clk_sel_state", &self.sys_clk_sel_state())
            .field("sys_clk_no_div_state", &self.sys_clk_no_div_state())
            .field("icg_sys_clk_en_state", &self.icg_sys_clk_en_state())
            .field("icg_modem_switch_state", &self.icg_modem_switch_state())
            .field("icg_modem_code_state", &self.icg_modem_code_state())
            .field("icg_slp_sel_state", &self.icg_slp_sel_state())
            .field("icg_global_xtal_state", &self.icg_global_xtal_state())
            .field("icg_global_pll_state", &self.icg_global_pll_state())
            .field("ana_i2c_iso_en_state", &self.ana_i2c_iso_en_state())
            .field("ana_i2c_retention_state", &self.ana_i2c_retention_state())
            .field("ana_xpd_bb_i2c_state", &self.ana_xpd_bb_i2c_state())
            .field("ana_xpd_bbpll_i2c_state", &self.ana_xpd_bbpll_i2c_state())
            .field("ana_xpd_bbpll_state", &self.ana_xpd_bbpll_state())
            .field("ana_xpd_xtal_state", &self.ana_xpd_xtal_state())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`clk_state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_STATE0_SPEC;
impl crate::RegisterSpec for CLK_STATE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_state0::R`](R) reader structure
impl crate::Readable for CLK_STATE0_SPEC {}
///`reset()` method sets CLK_STATE0 to value 0x03
impl crate::Resettable for CLK_STATE0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
