#[doc = "Register `SCL_LOW` reader"]
pub struct R(crate::R<SCL_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_LOW` writer"]
pub struct W(crate::W<SCL_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_LOW_SPEC>;
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
impl From<crate::W<SCL_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - This register is used to configure how many clock cycles SCL remains low."]
pub type PERIOD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERIOD` writer - This register is used to configure how many clock cycles SCL remains low."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, SCL_LOW_SPEC, 20, O, u32, u32>;
impl R {
    #[doc = "Bits 0:19 - This register is used to configure how many clock cycles SCL remains low."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_LOW")
            .field("period", &format_args!("{}", self.period().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to configure how many clock cycles SCL remains low."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the low level width of SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_low](index.html) module"]
pub struct SCL_LOW_SPEC;
impl crate::RegisterSpec for SCL_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_low::R](R) reader structure"]
impl crate::Readable for SCL_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_low::W](W) writer structure"]
impl crate::Writable for SCL_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_LOW to value 0x0100"]
impl crate::Resettable for SCL_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
