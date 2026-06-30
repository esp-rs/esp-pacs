#[doc = "Register `IMM_HP_CK_POWER_1` writer"]
pub type W = crate::W<IMM_HP_CK_POWER_1_SPEC>;
#[doc = "Field `TIE_LOW_XPD_PLL_I2C` writer - need_des"]
pub type TIE_LOW_XPD_PLL_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_LOW_XPD_PLL` writer - need_des"]
pub type TIE_LOW_XPD_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_LOW_GLOBAL_PLL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_PLL_ICG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_HIGH_GLOBAL_PLL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_PLL_ICG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_HIGH_XPD_PLL_I2C` writer - need_des"]
pub type TIE_HIGH_XPD_PLL_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_HIGH_XPD_PLL` writer - need_des"]
pub type TIE_HIGH_XPD_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_HP_CK_POWER_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_pll_i2c(&mut self) -> TIE_LOW_XPD_PLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_PLL_I2C_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_pll(&mut self) -> TIE_LOW_XPD_PLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_PLL_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - need_des"]
    #[inline(always)]
    pub fn tie_low_global_pll_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_PLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_GLOBAL_PLL_ICG_W::new(self, 8)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn tie_high_global_pll_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_PLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_GLOBAL_PLL_ICG_W::new(self, 19)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_pll_i2c(&mut self) -> TIE_HIGH_XPD_PLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_pll(&mut self) -> TIE_HIGH_XPD_PLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_PLL_W::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_ck_power_1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_HP_CK_POWER_1_SPEC;
impl crate::RegisterSpec for IMM_HP_CK_POWER_1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_hp_ck_power_1::W`](W) writer structure"]
impl crate::Writable for IMM_HP_CK_POWER_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_HP_CK_POWER_1 to value 0"]
impl crate::Resettable for IMM_HP_CK_POWER_1_SPEC {}
