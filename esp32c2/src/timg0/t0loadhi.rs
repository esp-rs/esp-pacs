#[doc = "Register `T0LOADHI` reader"]
pub struct R(crate::R<T0LOADHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LOADHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LOADHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LOADHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0LOADHI` writer"]
pub struct W(crate::W<T0LOADHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOADHI_SPEC>;
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
impl From<crate::W<T0LOADHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOADHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD_HI` reader - High 22 bits of the value that a reload will load onto timer 0 time-base counter."]
pub type LOAD_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOAD_HI` writer - High 22 bits of the value that a reload will load onto timer 0 time-base counter."]
pub type LOAD_HI_W<'a, const O: u8> = crate::FieldWriter<'a, T0LOADHI_SPEC, 22, O, u32, u32>;
impl R {
    #[doc = "Bits 0:21 - High 22 bits of the value that a reload will load onto timer 0 time-base counter."]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0LOADHI")
            .field("load_hi", &format_args!("{}", self.load_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0LOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - High 22 bits of the value that a reload will load onto timer 0 time-base counter."]
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<0> {
        LOAD_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer $x reload value, high 22 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadhi](index.html) module"]
pub struct T0LOADHI_SPEC;
impl crate::RegisterSpec for T0LOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0loadhi::R](R) reader structure"]
impl crate::Readable for T0LOADHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0loadhi::W](W) writer structure"]
impl crate::Writable for T0LOADHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0LOADHI to value 0"]
impl crate::Resettable for T0LOADHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
