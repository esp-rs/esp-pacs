#[doc = "Register `T%sLOADHI` reader"]
pub struct R(crate::R<TLOADHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLOADHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLOADHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLOADHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sLOADHI` writer"]
pub struct W(crate::W<TLOADHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLOADHI_SPEC>;
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
impl From<crate::W<TLOADHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TLOADHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD_HI` reader - High 32 bits of the value that a reload will load onto timer %s time-base counter."]
pub type LOAD_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOAD_HI` writer - High 32 bits of the value that a reload will load onto timer %s time-base counter."]
pub type LOAD_HI_W<'a, const O: u8> = crate::FieldWriter<'a, TLOADHI_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - High 32 bits of the value that a reload will load onto timer %s time-base counter."]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TLOADHI")
            .field("load_hi", &format_args!("{}", self.load_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TLOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - High 32 bits of the value that a reload will load onto timer %s time-base counter."]
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
#[doc = "Timer %s reload value, high 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tloadhi](index.html) module"]
pub struct TLOADHI_SPEC;
impl crate::RegisterSpec for TLOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tloadhi::R](R) reader structure"]
impl crate::Readable for TLOADHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tloadhi::W](W) writer structure"]
impl crate::Writable for TLOADHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sLOADHI to value 0"]
impl crate::Resettable for TLOADHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
