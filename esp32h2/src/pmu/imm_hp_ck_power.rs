#[doc = "Register `IMM_HP_CK_POWER` writer"]
pub struct W(crate::W<IMM_HP_CK_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMM_HP_CK_POWER_SPEC>;
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
impl From<crate::W<IMM_HP_CK_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMM_HP_CK_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE_LOW_GLOBAL_BBPLL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_BBPLL_ICG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_GLOBAL_XTAL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_XTAL_ICG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_I2C_RETENTION` writer - need_des"]
pub type TIE_LOW_I2C_RETENTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_XPD_BB_I2C` writer - need_des"]
pub type TIE_LOW_XPD_BB_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_XPD_BBPLL_I2C` writer - need_des"]
pub type TIE_LOW_XPD_BBPLL_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_XPD_BBPLL` writer - need_des"]
pub type TIE_LOW_XPD_BBPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_XPD_XTAL` writer - need_des"]
pub type TIE_LOW_XPD_XTAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_GLOBAL_BBPLL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_BBPLL_ICG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_GLOBAL_XTAL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_XTAL_ICG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_I2C_RETENTION` writer - need_des"]
pub type TIE_HIGH_I2C_RETENTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_XPD_BB_I2C` writer - need_des"]
pub type TIE_HIGH_XPD_BB_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_XPD_BBPLL_I2C` writer - need_des"]
pub type TIE_HIGH_XPD_BBPLL_I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_XPD_BBPLL` writer - need_des"]
pub type TIE_HIGH_XPD_BBPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
#[doc = "Field `TIE_HIGH_XPD_XTAL` writer - need_des"]
pub type TIE_HIGH_XPD_XTAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_HP_CK_POWER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_global_bbpll_icg(&mut self) -> TIE_LOW_GLOBAL_BBPLL_ICG_W<0> {
        TIE_LOW_GLOBAL_BBPLL_ICG_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_global_xtal_icg(&mut self) -> TIE_LOW_GLOBAL_XTAL_ICG_W<1> {
        TIE_LOW_GLOBAL_XTAL_ICG_W::new(self)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_i2c_retention(&mut self) -> TIE_LOW_I2C_RETENTION_W<2> {
        TIE_LOW_I2C_RETENTION_W::new(self)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_bb_i2c(&mut self) -> TIE_LOW_XPD_BB_I2C_W<3> {
        TIE_LOW_XPD_BB_I2C_W::new(self)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_bbpll_i2c(&mut self) -> TIE_LOW_XPD_BBPLL_I2C_W<4> {
        TIE_LOW_XPD_BBPLL_I2C_W::new(self)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_bbpll(&mut self) -> TIE_LOW_XPD_BBPLL_W<5> {
        TIE_LOW_XPD_BBPLL_W::new(self)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_xpd_xtal(&mut self) -> TIE_LOW_XPD_XTAL_W<6> {
        TIE_LOW_XPD_XTAL_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_global_bbpll_icg(&mut self) -> TIE_HIGH_GLOBAL_BBPLL_ICG_W<25> {
        TIE_HIGH_GLOBAL_BBPLL_ICG_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_global_xtal_icg(&mut self) -> TIE_HIGH_GLOBAL_XTAL_ICG_W<26> {
        TIE_HIGH_GLOBAL_XTAL_ICG_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_i2c_retention(&mut self) -> TIE_HIGH_I2C_RETENTION_W<27> {
        TIE_HIGH_I2C_RETENTION_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_bb_i2c(&mut self) -> TIE_HIGH_XPD_BB_I2C_W<28> {
        TIE_HIGH_XPD_BB_I2C_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_bbpll_i2c(&mut self) -> TIE_HIGH_XPD_BBPLL_I2C_W<29> {
        TIE_HIGH_XPD_BBPLL_I2C_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_bbpll(&mut self) -> TIE_HIGH_XPD_BBPLL_W<30> {
        TIE_HIGH_XPD_BBPLL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_xpd_xtal(&mut self) -> TIE_HIGH_XPD_XTAL_W<31> {
        TIE_HIGH_XPD_XTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imm_hp_ck_power](index.html) module"]
pub struct IMM_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for IMM_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [imm_hp_ck_power::W](W) writer structure"]
impl crate::Writable for IMM_HP_CK_POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_HP_CK_POWER to value 0"]
impl crate::Resettable for IMM_HP_CK_POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
