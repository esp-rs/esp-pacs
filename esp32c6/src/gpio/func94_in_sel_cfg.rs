#[doc = "Register `FUNC94_IN_SEL_CFG` reader"]
pub struct R(crate::R<FUNC94_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC94_IN_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC94_IN_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC94_IN_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC94_IN_SEL_CFG` writer"]
pub struct W(crate::W<FUNC94_IN_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC94_IN_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC94_IN_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC94_IN_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC94_IN_SEL` reader - set this value: s=0-34: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
pub type FUNC94_IN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC94_IN_SEL` writer - set this value: s=0-34: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
pub type FUNC94_IN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNC94_IN_SEL_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `FUNC94_IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC94_IN_INV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `FUNC94_IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC94_IN_INV_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FUNC94_IN_SEL_CFG_SPEC, bool, O>;
#[doc = "Field `SIG94_IN_SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG94_IN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SIG94_IN_SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG94_IN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FUNC94_IN_SEL_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - set this value: s=0-34: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    #[inline(always)]
    pub fn func94_in_sel(&self) -> FUNC94_IN_SEL_R {
        FUNC94_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func94_in_inv_sel(&self) -> FUNC94_IN_INV_SEL_R {
        FUNC94_IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig94_in_sel(&self) -> SIG94_IN_SEL_R {
        SIG94_IN_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - set this value: s=0-34: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    #[inline(always)]
    #[must_use]
    pub fn func94_in_sel(&mut self) -> FUNC94_IN_SEL_W<0> {
        FUNC94_IN_SEL_W::new(self)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    #[must_use]
    pub fn func94_in_inv_sel(&mut self) -> FUNC94_IN_INV_SEL_W<6> {
        FUNC94_IN_INV_SEL_W::new(self)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn sig94_in_sel(&mut self) -> SIG94_IN_SEL_W<7> {
        SIG94_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO input function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func94_in_sel_cfg](index.html) module"]
pub struct FUNC94_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC94_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func94_in_sel_cfg::R](R) reader structure"]
impl crate::Readable for FUNC94_IN_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func94_in_sel_cfg::W](W) writer structure"]
impl crate::Writable for FUNC94_IN_SEL_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC94_IN_SEL_CFG to value 0x3c"]
impl crate::Resettable for FUNC94_IN_SEL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c;
}
