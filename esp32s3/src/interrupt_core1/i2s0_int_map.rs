#[doc = "Register `I2S0_INT_MAP` reader"]
pub struct R(crate::R<I2S0_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S0_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S0_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S0_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S0_INT_MAP` writer"]
pub struct W(crate::W<I2S0_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S0_INT_MAP_SPEC>;
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
impl From<crate::W<I2S0_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S0_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S0_INT_MAP` reader - this register used to map i2s0 interrupt to one of core1's external interrupt"]
pub type I2S0_INT_MAP_R = crate::FieldReader;
#[doc = "Field `I2S0_INT_MAP` writer - this register used to map i2s0 interrupt to one of core1's external interrupt"]
pub type I2S0_INT_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, I2S0_INT_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map i2s0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn i2s0_int_map(&self) -> I2S0_INT_MAP_R {
        I2S0_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S0_INT_MAP")
            .field(
                "i2s0_int_map",
                &format_args!("{}", self.i2s0_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2S0_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map i2s0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_int_map(&mut self) -> I2S0_INT_MAP_W<0> {
        I2S0_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s0 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s0_int_map](index.html) module"]
pub struct I2S0_INT_MAP_SPEC;
impl crate::RegisterSpec for I2S0_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s0_int_map::R](R) reader structure"]
impl crate::Readable for I2S0_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s0_int_map::W](W) writer structure"]
impl crate::Writable for I2S0_INT_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S0_INT_MAP to value 0x10"]
impl crate::Resettable for I2S0_INT_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
