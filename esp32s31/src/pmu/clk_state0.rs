#[doc = "Register `CLK_STATE0` reader"]
pub type R = crate::R<CLK_STATE0_SPEC>;
#[doc = "Field `STABLE_XPD_XTAL_STATE` reader - need_des"]
pub type STABLE_XPD_XTAL_STATE_R = crate::BitReader;
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
#[doc = "Field `PMU_ANA_I2C_ISO_EN_STATE` reader - need_des"]
pub type PMU_ANA_I2C_ISO_EN_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ANA_I2C_RETENTION_STATE` reader - need_des"]
pub type PMU_ANA_I2C_RETENTION_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ANA_XPD_BB_I2C_STATE` reader - need_des"]
pub type PMU_ANA_XPD_BB_I2C_STATE_R = crate::BitReader;
#[doc = "Field `PMU_ANA_XPD_XTAL_STATE` reader - need_des"]
pub type PMU_ANA_XPD_XTAL_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn stable_xpd_xtal_state(&self) -> STABLE_XPD_XTAL_STATE_R {
        STABLE_XPD_XTAL_STATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_slp_sel_state(&self) -> PMU_SYS_CLK_SLP_SEL_STATE_R {
        PMU_SYS_CLK_SLP_SEL_STATE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_sel_state(&self) -> PMU_SYS_CLK_SEL_STATE_R {
        PMU_SYS_CLK_SEL_STATE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn pmu_sys_clk_no_div_state(&self) -> PMU_SYS_CLK_NO_DIV_STATE_R {
        PMU_SYS_CLK_NO_DIV_STATE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_sys_clk_en_state(&self) -> PMU_ICG_SYS_CLK_EN_STATE_R {
        PMU_ICG_SYS_CLK_EN_STATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_modem_switch_state(&self) -> PMU_ICG_MODEM_SWITCH_STATE_R {
        PMU_ICG_MODEM_SWITCH_STATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_modem_code_state(&self) -> PMU_ICG_MODEM_CODE_STATE_R {
        PMU_ICG_MODEM_CODE_STATE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_slp_sel_state(&self) -> PMU_ICG_SLP_SEL_STATE_R {
        PMU_ICG_SLP_SEL_STATE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_global_xtal_state(&self) -> PMU_ICG_GLOBAL_XTAL_STATE_R {
        PMU_ICG_GLOBAL_XTAL_STATE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_i2c_iso_en_state(&self) -> PMU_ANA_I2C_ISO_EN_STATE_R {
        PMU_ANA_I2C_ISO_EN_STATE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_i2c_retention_state(&self) -> PMU_ANA_I2C_RETENTION_STATE_R {
        PMU_ANA_I2C_RETENTION_STATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_bb_i2c_state(&self) -> PMU_ANA_XPD_BB_I2C_STATE_R {
        PMU_ANA_XPD_BB_I2C_STATE_R::new(((self.bits >> 28) & 1) != 0)
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
            .field("stable_xpd_xtal_state", &self.stable_xpd_xtal_state())
            .field(
                "pmu_sys_clk_slp_sel_state",
                &self.pmu_sys_clk_slp_sel_state(),
            )
            .field("pmu_sys_clk_sel_state", &self.pmu_sys_clk_sel_state())
            .field("pmu_sys_clk_no_div_state", &self.pmu_sys_clk_no_div_state())
            .field("pmu_icg_sys_clk_en_state", &self.pmu_icg_sys_clk_en_state())
            .field(
                "pmu_icg_modem_switch_state",
                &self.pmu_icg_modem_switch_state(),
            )
            .field("pmu_icg_modem_code_state", &self.pmu_icg_modem_code_state())
            .field("pmu_icg_slp_sel_state", &self.pmu_icg_slp_sel_state())
            .field(
                "pmu_icg_global_xtal_state",
                &self.pmu_icg_global_xtal_state(),
            )
            .field("pmu_ana_i2c_iso_en_state", &self.pmu_ana_i2c_iso_en_state())
            .field(
                "pmu_ana_i2c_retention_state",
                &self.pmu_ana_i2c_retention_state(),
            )
            .field("pmu_ana_xpd_bb_i2c_state", &self.pmu_ana_xpd_bb_i2c_state())
            .field("pmu_ana_xpd_xtal_state", &self.pmu_ana_xpd_xtal_state())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_STATE0_SPEC;
impl crate::RegisterSpec for CLK_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_state0::R`](R) reader structure"]
impl crate::Readable for CLK_STATE0_SPEC {}
#[doc = "`reset()` method sets CLK_STATE0 to value 0x02"]
impl crate::Resettable for CLK_STATE0_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
