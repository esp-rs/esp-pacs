#[doc = "Register `CLOCK_DIVIDER` reader"]
pub struct R(crate::R<CLOCK_DIVIDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_DIVIDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_DIVIDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_DIVIDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_DIVIDER` writer"]
pub struct W(crate::W<CLOCK_DIVIDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_DIVIDER_SPEC>;
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
impl From<crate::W<CLOCK_DIVIDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_DIVIDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub type CD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CD` writer - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub type CD_W<'a> = crate::FieldWriter<'a, u32, CLOCK_DIVIDER_SPEC, u8, u8, 8, 0>;
#[doc = "Field `CLOCK_OFF` reader - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub type CLOCK_OFF_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_OFF` writer - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub type CLOCK_OFF_W<'a> = crate::BitWriter<'a, u32, CLOCK_DIVIDER_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&self) -> CLOCK_OFF_R {
        CLOCK_OFF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W::new(self)
    }
    #[doc = "Bit 8 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&mut self) -> CLOCK_OFF_W {
        CLOCK_OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_divider](index.html) module"]
pub struct CLOCK_DIVIDER_SPEC;
impl crate::RegisterSpec for CLOCK_DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_divider::R](R) reader structure"]
impl crate::Readable for CLOCK_DIVIDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_divider::W](W) writer structure"]
impl crate::Writable for CLOCK_DIVIDER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_DIVIDER to value 0"]
impl crate::Resettable for CLOCK_DIVIDER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
