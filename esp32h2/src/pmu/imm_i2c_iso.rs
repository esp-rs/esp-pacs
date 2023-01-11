#[doc = "Register `IMM_I2C_ISO` writer"]
pub struct W(crate::W<IMM_I2C_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMM_I2C_ISO_SPEC>;
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
impl From<crate::W<IMM_I2C_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMM_I2C_ISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE_HIGH_I2C_ISO_EN` writer - need_des"]
pub type TIE_HIGH_I2C_ISO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_I2C_ISO_SPEC, bool, O>;
#[doc = "Field `TIE_LOW_I2C_ISO_EN` writer - need_des"]
pub type TIE_LOW_I2C_ISO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IMM_I2C_ISO_SPEC, bool, O>;
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_high_i2c_iso_en(&mut self) -> TIE_HIGH_I2C_ISO_EN_W<30> {
        TIE_HIGH_I2C_ISO_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn tie_low_i2c_iso_en(&mut self) -> TIE_LOW_I2C_ISO_EN_W<31> {
        TIE_LOW_I2C_ISO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imm_i2c_iso](index.html) module"]
pub struct IMM_I2C_ISO_SPEC;
impl crate::RegisterSpec for IMM_I2C_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [imm_i2c_iso::W](W) writer structure"]
impl crate::Writable for IMM_I2C_ISO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMM_I2C_ISO to value 0"]
impl crate::Resettable for IMM_I2C_ISO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}