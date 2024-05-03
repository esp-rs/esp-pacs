#[doc = "Register `IMM_HP_CK_POWER` reader"]
pub type R = crate::R<IMM_HP_CK_POWER_SPEC>;
#[doc = "Register `IMM_HP_CK_POWER` writer"]
pub type W = crate::W<IMM_HP_CK_POWER_SPEC>;
#[doc = "Field `TIE_LOW_CALI_XTAL_ICG` reader - need_des"]
pub type TIE_LOW_CALI_XTAL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_LOW_CALI_XTAL_ICG` writer - need_des"]
pub type TIE_LOW_CALI_XTAL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_GLOBAL_PLL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_PLL_ICG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_LOW_GLOBAL_XTAL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_XTAL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_I2C_RETENTION` writer - need_des"]
pub type TIE_LOW_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_PLL_I2C` writer - need_des"]
pub type TIE_LOW_XPD_PLL_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_LOW_XPD_PLL` writer - need_des"]
pub type TIE_LOW_XPD_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_LOW_XPD_XTAL` writer - need_des"]
pub type TIE_LOW_XPD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_CALI_XTAL_ICG` reader - need_des"]
pub type TIE_HIGH_CALI_XTAL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_CALI_XTAL_ICG` writer - need_des"]
pub type TIE_HIGH_CALI_XTAL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_PLL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_PLL_ICG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_HIGH_GLOBAL_XTAL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_XTAL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_I2C_RETENTION` writer - need_des"]
pub type TIE_HIGH_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_PLL_I2C` writer - need_des"]
pub type TIE_HIGH_XPD_PLL_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_HIGH_XPD_PLL` writer - need_des"]
pub type TIE_HIGH_XPD_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIE_HIGH_XPD_XTAL` writer - need_des"]
pub type TIE_HIGH_XPD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn tie_low_cali_xtal_icg(&self) -> TIE_LOW_CALI_XTAL_ICG_R {
        TIE_LOW_CALI_XTAL_ICG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn tie_high_cali_xtal_icg(&self) -> TIE_HIGH_CALI_XTAL_ICG_R {
        TIE_HIGH_CALI_XTAL_ICG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMM_HP_CK_POWER")
            .field("tie_low_cali_xtal_icg", &self.tie_low_cali_xtal_icg().bit())
            .field(
                "tie_high_cali_xtal_icg",
                &self.tie_high_cali_xtal_icg().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_HP_CK_POWER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_cali_xtal_icg(&mut self) -> TIE_LOW_CALI_XTAL_ICG_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_CALI_XTAL_ICG_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_global_pll_icg(&mut self) -> TIE_LOW_GLOBAL_PLL_ICG_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_GLOBAL_PLL_ICG_W::new(self, 1)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_global_xtal_icg(&mut self) -> TIE_LOW_GLOBAL_XTAL_ICG_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_GLOBAL_XTAL_ICG_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_i2c_retention(&mut self) -> TIE_LOW_I2C_RETENTION_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_I2C_RETENTION_W::new(self, 6)
    }
    #[doc = "Bits 7:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_pll_i2c(&mut self) -> TIE_LOW_XPD_PLL_I2C_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_XPD_PLL_I2C_W::new(self, 7)
    }
    #[doc = "Bits 11:14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_pll(&mut self) -> TIE_LOW_XPD_PLL_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_XPD_PLL_W::new(self, 11)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_xtal(&mut self) -> TIE_LOW_XPD_XTAL_W<IMM_HP_CK_POWER_SPEC> {
        TIE_LOW_XPD_XTAL_W::new(self, 15)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_cali_xtal_icg(&mut self) -> TIE_HIGH_CALI_XTAL_ICG_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_CALI_XTAL_ICG_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_global_pll_icg(&mut self) -> TIE_HIGH_GLOBAL_PLL_ICG_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_GLOBAL_PLL_ICG_W::new(self, 17)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_global_xtal_icg(&mut self) -> TIE_HIGH_GLOBAL_XTAL_ICG_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_GLOBAL_XTAL_ICG_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_i2c_retention(&mut self) -> TIE_HIGH_I2C_RETENTION_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_I2C_RETENTION_W::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_pll_i2c(&mut self) -> TIE_HIGH_XPD_PLL_I2C_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_pll(&mut self) -> TIE_HIGH_XPD_PLL_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_XPD_PLL_W::new(self, 27)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_xtal(&mut self) -> TIE_HIGH_XPD_XTAL_W<IMM_HP_CK_POWER_SPEC> {
        TIE_HIGH_XPD_XTAL_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imm_hp_ck_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_hp_ck_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for IMM_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imm_hp_ck_power::R`](R) reader structure"]
impl crate::Readable for IMM_HP_CK_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imm_hp_ck_power::W`](W) writer structure"]
impl crate::Writable for IMM_HP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMM_HP_CK_POWER to value 0"]
impl crate::Resettable for IMM_HP_CK_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
