#[doc = "Register `CLK_STATE0` reader"]
pub type R = crate::R<CLK_STATE0_SPEC>;
#[doc = "Field `STABLE_XPD_PLL_STATE` reader - need_des"]
pub type STABLE_XPD_PLL_STATE_R = crate::FieldReader;
#[doc = "Field `STABLE_XPD_XTAL_STATE` reader - need_des"]
pub type STABLE_XPD_XTAL_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ANA_XPD_PLL_I2C_STATE` reader - need_des"]
pub type PMU_ANA_XPD_PLL_I2C_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_SYS_CLK_SLP_SEL_STATE` reader - need_des"]
pub type PMU_SYS_CLK_SLP_SEL_STATE_R = crate::BitReader;
#[doc = "Field `PMU_SYS_CLK_SEL_STATE` reader - need_des"]
pub type PMU_SYS_CLK_SEL_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_SYS_CLK_NO_DIV_STATE` reader - need_des"]
pub type PMU_SYS_CLK_NO_DIV_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ICG_SYS_CLK_EN_STATE` reader - need_des"]
pub type PMU_ICG_SYS_CLK_EN_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ICG_MODEM_SWITCH_STATE` reader - need_des"]
pub type PMU_ICG_MODEM_SWITCH_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ICG_MODEM_CODE_STATE` reader - need_des"]
pub type PMU_ICG_MODEM_CODE_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_ICG_SLP_SEL_STATE` reader - need_des"]
pub type PMU_ICG_SLP_SEL_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ICG_GLOBAL_XTAL_STATE` reader - need_des"]
pub type PMU_ICG_GLOBAL_XTAL_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ICG_GLOBAL_PLL_STATE` reader - need_des"]
pub type PMU_ICG_GLOBAL_PLL_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_ANA_I2C_ISO_EN_STATE` reader - need_des"]
pub type PMU_ANA_I2C_ISO_EN_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ANA_I2C_RETENTION_STATE` reader - need_des"]
pub type PMU_ANA_I2C_RETENTION_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ANA_XPD_PLL_STATE` reader - need_des"]
pub type PMU_ANA_XPD_PLL_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_ANA_XPD_XTAL_STATE` reader - need_des"]
pub type PMU_ANA_XPD_XTAL_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn stable_xpd_pll_state(&self) -> STABLE_XPD_PLL_STATE_R {
        STABLE_XPD_PLL_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn stable_xpd_xtal_state(&self) -> STABLE_XPD_XTAL_STATE_R {
        STABLE_XPD_XTAL_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_pll_i2c_state(&self) -> PMU_ANA_XPD_PLL_I2C_STATE_R {
        PMU_ANA_XPD_PLL_I2C_STATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_slp_sel_state(&self) -> PMU_SYS_CLK_SLP_SEL_STATE_R {
        PMU_SYS_CLK_SLP_SEL_STATE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_sel_state(&self) -> PMU_SYS_CLK_SEL_STATE_R {
        PMU_SYS_CLK_SEL_STATE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_no_div_state(&self) -> PMU_SYS_CLK_NO_DIV_STATE_R {
        PMU_SYS_CLK_NO_DIV_STATE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_sys_clk_en_state(&self) -> PMU_ICG_SYS_CLK_EN_STATE_R {
        PMU_ICG_SYS_CLK_EN_STATE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_modem_switch_state(&self) -> PMU_ICG_MODEM_SWITCH_STATE_R {
        PMU_ICG_MODEM_SWITCH_STATE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_modem_code_state(&self) -> PMU_ICG_MODEM_CODE_STATE_R {
        PMU_ICG_MODEM_CODE_STATE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_slp_sel_state(&self) -> PMU_ICG_SLP_SEL_STATE_R {
        PMU_ICG_SLP_SEL_STATE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_global_xtal_state(&self) -> PMU_ICG_GLOBAL_XTAL_STATE_R {
        PMU_ICG_GLOBAL_XTAL_STATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_global_pll_state(&self) -> PMU_ICG_GLOBAL_PLL_STATE_R {
        PMU_ICG_GLOBAL_PLL_STATE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_i2c_iso_en_state(&self) -> PMU_ANA_I2C_ISO_EN_STATE_R {
        PMU_ANA_I2C_ISO_EN_STATE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_i2c_retention_state(&self) -> PMU_ANA_I2C_RETENTION_STATE_R {
        PMU_ANA_I2C_RETENTION_STATE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_pll_state(&self) -> PMU_ANA_XPD_PLL_STATE_R {
        PMU_ANA_XPD_PLL_STATE_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_xtal_state(&self) -> PMU_ANA_XPD_XTAL_STATE_R {
        PMU_ANA_XPD_XTAL_STATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_STATE0")
            .field("stable_xpd_pll_state", &self.stable_xpd_pll_state().bits())
            .field("stable_xpd_xtal_state", &self.stable_xpd_xtal_state().bit())
            .field(
                "pmu_ana_xpd_pll_i2c_state",
                &self.pmu_ana_xpd_pll_i2c_state().bits(),
            )
            .field(
                "pmu_sys_clk_slp_sel_state",
                &self.pmu_sys_clk_slp_sel_state().bit(),
            )
            .field(
                "pmu_sys_clk_sel_state",
                &self.pmu_sys_clk_sel_state().bits(),
            )
            .field(
                "pmu_sys_clk_no_div_state",
                &self.pmu_sys_clk_no_div_state().bit(),
            )
            .field(
                "pmu_icg_sys_clk_en_state",
                &self.pmu_icg_sys_clk_en_state().bit(),
            )
            .field(
                "pmu_icg_modem_switch_state",
                &self.pmu_icg_modem_switch_state().bit(),
            )
            .field(
                "pmu_icg_modem_code_state",
                &self.pmu_icg_modem_code_state().bits(),
            )
            .field("pmu_icg_slp_sel_state", &self.pmu_icg_slp_sel_state().bit())
            .field(
                "pmu_icg_global_xtal_state",
                &self.pmu_icg_global_xtal_state().bit(),
            )
            .field(
                "pmu_icg_global_pll_state",
                &self.pmu_icg_global_pll_state().bits(),
            )
            .field(
                "pmu_ana_i2c_iso_en_state",
                &self.pmu_ana_i2c_iso_en_state().bit(),
            )
            .field(
                "pmu_ana_i2c_retention_state",
                &self.pmu_ana_i2c_retention_state().bit(),
            )
            .field(
                "pmu_ana_xpd_pll_state",
                &self.pmu_ana_xpd_pll_state().bits(),
            )
            .field(
                "pmu_ana_xpd_xtal_state",
                &self.pmu_ana_xpd_xtal_state().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_STATE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_STATE0_SPEC;
impl crate::RegisterSpec for CLK_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_state0::R`](R) reader structure"]
impl crate::Readable for CLK_STATE0_SPEC {}
#[doc = "`reset()` method sets CLK_STATE0 to value 0x0f"]
impl crate::Resettable for CLK_STATE0_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
