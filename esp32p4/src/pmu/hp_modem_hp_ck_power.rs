#[doc = "Register `HP_MODEM_HP_CK_POWER` writer"]
pub type W = crate::W<HP_MODEM_HP_CK_POWER_SPEC>;
#[doc = "Field `HP_MODEM_I2C_ISO_EN` writer - need_des"]
pub type HP_MODEM_I2C_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_I2C_RETENTION` writer - need_des"]
pub type HP_MODEM_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_PLL_I2C` writer - need_des"]
pub type HP_MODEM_XPD_PLL_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_XPD_PLL` writer - need_des"]
pub type HP_MODEM_XPD_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_HP_CK_POWER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_i2c_iso_en(&mut self) -> HP_MODEM_I2C_ISO_EN_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_I2C_ISO_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_i2c_retention(
        &mut self,
    ) -> HP_MODEM_I2C_RETENTION_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_I2C_RETENTION_W::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_xpd_pll_i2c(&mut self) -> HP_MODEM_XPD_PLL_I2C_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_xpd_pll(&mut self) -> HP_MODEM_XPD_PLL_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_PLL_W::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_ck_power::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_hp_ck_power::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_HP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_MODEM_HP_CK_POWER to value 0"]
impl crate::Resettable for HP_MODEM_HP_CK_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
