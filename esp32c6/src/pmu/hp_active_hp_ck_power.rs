#[doc = "Register `HP_ACTIVE_HP_CK_POWER` reader"]
pub struct R(crate::R<HP_ACTIVE_HP_CK_POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_ACTIVE_HP_CK_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_ACTIVE_HP_CK_POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_ACTIVE_HP_CK_POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_ACTIVE_HP_CK_POWER` writer"]
pub struct W(crate::W<HP_ACTIVE_HP_CK_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_ACTIVE_HP_CK_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HP_ACTIVE_HP_CK_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_ACTIVE_HP_CK_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_ACTIVE_I2C_ISO_EN` reader - need_des"]
pub type HP_ACTIVE_I2C_ISO_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_I2C_ISO_EN` writer - need_des"]
pub type HP_ACTIVE_I2C_ISO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_HP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_ACTIVE_I2C_RETENTION` reader - need_des"]
pub type HP_ACTIVE_I2C_RETENTION_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_I2C_RETENTION` writer - need_des"]
pub type HP_ACTIVE_I2C_RETENTION_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_HP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_ACTIVE_XPD_BB_I2C` reader - need_des"]
pub type HP_ACTIVE_XPD_BB_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BB_I2C` writer - need_des"]
pub type HP_ACTIVE_XPD_BB_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_HP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL_I2C` reader - need_des"]
pub type HP_ACTIVE_XPD_BBPLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL_I2C` writer - need_des"]
pub type HP_ACTIVE_XPD_BBPLL_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_HP_CK_POWER_SPEC, O>;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL` reader - need_des"]
pub type HP_ACTIVE_XPD_BBPLL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BBPLL` writer - need_des"]
pub type HP_ACTIVE_XPD_BBPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_ACTIVE_HP_CK_POWER_SPEC, O>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_iso_en(&self) -> HP_ACTIVE_I2C_ISO_EN_R {
        HP_ACTIVE_I2C_ISO_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_active_i2c_retention(&self) -> HP_ACTIVE_I2C_RETENTION_R {
        HP_ACTIVE_I2C_RETENTION_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_bb_i2c(&self) -> HP_ACTIVE_XPD_BB_I2C_R {
        HP_ACTIVE_XPD_BB_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_bbpll_i2c(&self) -> HP_ACTIVE_XPD_BBPLL_I2C_R {
        HP_ACTIVE_XPD_BBPLL_I2C_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_bbpll(&self) -> HP_ACTIVE_XPD_BBPLL_R {
        HP_ACTIVE_XPD_BBPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_HP_CK_POWER")
            .field(
                "hp_active_i2c_iso_en",
                &format_args!("{}", self.hp_active_i2c_iso_en().bit()),
            )
            .field(
                "hp_active_i2c_retention",
                &format_args!("{}", self.hp_active_i2c_retention().bit()),
            )
            .field(
                "hp_active_xpd_bb_i2c",
                &format_args!("{}", self.hp_active_xpd_bb_i2c().bit()),
            )
            .field(
                "hp_active_xpd_bbpll_i2c",
                &format_args!("{}", self.hp_active_xpd_bbpll_i2c().bit()),
            )
            .field(
                "hp_active_xpd_bbpll",
                &format_args!("{}", self.hp_active_xpd_bbpll().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_ACTIVE_HP_CK_POWER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_i2c_iso_en(&mut self) -> HP_ACTIVE_I2C_ISO_EN_W<26> {
        HP_ACTIVE_I2C_ISO_EN_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_i2c_retention(&mut self) -> HP_ACTIVE_I2C_RETENTION_W<27> {
        HP_ACTIVE_I2C_RETENTION_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_xpd_bb_i2c(&mut self) -> HP_ACTIVE_XPD_BB_I2C_W<28> {
        HP_ACTIVE_XPD_BB_I2C_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_xpd_bbpll_i2c(&mut self) -> HP_ACTIVE_XPD_BBPLL_I2C_W<29> {
        HP_ACTIVE_XPD_BBPLL_I2C_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_xpd_bbpll(&mut self) -> HP_ACTIVE_XPD_BBPLL_W<30> {
        HP_ACTIVE_XPD_BBPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_active_hp_ck_power](index.html) module"]
pub struct HP_ACTIVE_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_active_hp_ck_power::R](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_active_hp_ck_power::W](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_CK_POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_CK_POWER to value 0"]
impl crate::Resettable for HP_ACTIVE_HP_CK_POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
