#[doc = "Register `HP_ACTIVE_HP_CK_POWER` reader"]
pub type R = crate::R<HP_ACTIVE_HP_CK_POWER_SPEC>;
#[doc = "Register `HP_ACTIVE_HP_CK_POWER` writer"]
pub type W = crate::W<HP_ACTIVE_HP_CK_POWER_SPEC>;
#[doc = "Field `HP_ACTIVE_I2C_ISO_EN` reader - need_des"]
pub type HP_ACTIVE_I2C_ISO_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_I2C_ISO_EN` writer - need_des"]
pub type HP_ACTIVE_I2C_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_I2C_RETENTION` reader - need_des"]
pub type HP_ACTIVE_I2C_RETENTION_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_I2C_RETENTION` writer - need_des"]
pub type HP_ACTIVE_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_XPD_PLL_I2C` reader - need_des"]
pub type HP_ACTIVE_XPD_PLL_I2C_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_XPD_PLL_I2C` writer - need_des"]
pub type HP_ACTIVE_XPD_PLL_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_ACTIVE_XPD_PLL` reader - need_des"]
pub type HP_ACTIVE_XPD_PLL_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_XPD_PLL` writer - need_des"]
pub type HP_ACTIVE_XPD_PLL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_iso_en(&self) -> HP_ACTIVE_I2C_ISO_EN_R {
        HP_ACTIVE_I2C_ISO_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_retention(&self) -> HP_ACTIVE_I2C_RETENTION_R {
        HP_ACTIVE_I2C_RETENTION_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll_i2c(&self) -> HP_ACTIVE_XPD_PLL_I2C_R {
        HP_ACTIVE_XPD_PLL_I2C_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll(&self) -> HP_ACTIVE_XPD_PLL_R {
        HP_ACTIVE_XPD_PLL_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_HP_CK_POWER")
            .field("hp_active_i2c_iso_en", &self.hp_active_i2c_iso_en())
            .field("hp_active_i2c_retention", &self.hp_active_i2c_retention())
            .field("hp_active_xpd_pll_i2c", &self.hp_active_xpd_pll_i2c())
            .field("hp_active_xpd_pll", &self.hp_active_xpd_pll())
            .finish()
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_iso_en(&mut self) -> HP_ACTIVE_I2C_ISO_EN_W<HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_I2C_ISO_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_retention(
        &mut self,
    ) -> HP_ACTIVE_I2C_RETENTION_W<HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_I2C_RETENTION_W::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll_i2c(&mut self) -> HP_ACTIVE_XPD_PLL_I2C_W<HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_pll(&mut self) -> HP_ACTIVE_XPD_PLL_W<HP_ACTIVE_HP_CK_POWER_SPEC> {
        HP_ACTIVE_XPD_PLL_W::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_ck_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_ck_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_hp_ck_power::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_CK_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_hp_ck_power::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_CK_POWER to value 0"]
impl crate::Resettable for HP_ACTIVE_HP_CK_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
