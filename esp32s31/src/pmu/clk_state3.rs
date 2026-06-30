#[doc = "Register `CLK_STATE3` reader"]
pub type R = crate::R<CLK_STATE3_SPEC>;
#[doc = "Field `STABLE_XPD_PLL_STATE` reader - need_des"]
pub type STABLE_XPD_PLL_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_ICG_GLOBAL_PLL_STATE` reader - need_des"]
pub type PMU_ICG_GLOBAL_PLL_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_ANA_XPD_PLL_I2C_STATE` reader - need_des"]
pub type PMU_ANA_XPD_PLL_I2C_STATE_R = crate::FieldReader;
#[doc = "Field `PMU_ANA_XPD_PLL_STATE` reader - need_des"]
pub type PMU_ANA_XPD_PLL_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 16:19 - need_des"]
    #[inline(always)]
    pub fn stable_xpd_pll_state(&self) -> STABLE_XPD_PLL_STATE_R {
        STABLE_XPD_PLL_STATE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_global_pll_state(&self) -> PMU_ICG_GLOBAL_PLL_STATE_R {
        PMU_ICG_GLOBAL_PLL_STATE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_pll_i2c_state(&self) -> PMU_ANA_XPD_PLL_I2C_STATE_R {
        PMU_ANA_XPD_PLL_I2C_STATE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn pmu_ana_xpd_pll_state(&self) -> PMU_ANA_XPD_PLL_STATE_R {
        PMU_ANA_XPD_PLL_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_STATE3")
            .field("stable_xpd_pll_state", &self.stable_xpd_pll_state())
            .field("pmu_icg_global_pll_state", &self.pmu_icg_global_pll_state())
            .field(
                "pmu_ana_xpd_pll_i2c_state",
                &self.pmu_ana_xpd_pll_i2c_state(),
            )
            .field("pmu_ana_xpd_pll_state", &self.pmu_ana_xpd_pll_state())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_STATE3_SPEC;
impl crate::RegisterSpec for CLK_STATE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_state3::R`](R) reader structure"]
impl crate::Readable for CLK_STATE3_SPEC {}
#[doc = "`reset()` method sets CLK_STATE3 to value 0xffff_0000"]
impl crate::Resettable for CLK_STATE3_SPEC {
    const RESET_VALUE: u32 = 0xffff_0000;
}
