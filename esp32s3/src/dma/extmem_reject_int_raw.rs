#[doc = "Register `EXTMEM_REJECT_INT_RAW` reader"]
pub struct R(crate::R<EXTMEM_REJECT_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTMEM_REJECT_INT_RAW` writer"]
pub struct W(crate::W<EXTMEM_REJECT_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTMEM_REJECT_INT_RAW_SPEC>;
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
impl From<crate::W<EXTMEM_REJECT_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTMEM_REJECT_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTMEM_REJECT_INT_RAW` reader - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
pub type EXTMEM_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `EXTMEM_REJECT_INT_RAW` writer - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
pub type EXTMEM_REJECT_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, EXTMEM_REJECT_INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
    #[inline(always)]
    pub fn extmem_reject_int_raw(&self) -> EXTMEM_REJECT_INT_RAW_R {
        EXTMEM_REJECT_INT_RAW_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_INT_RAW")
            .field(
                "extmem_reject_int_raw",
                &format_args!("{}", self.extmem_reject_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMEM_REJECT_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when accessing external RAM is rejected by permission control."]
    #[inline(always)]
    #[must_use]
    pub fn extmem_reject_int_raw(&mut self) -> EXTMEM_REJECT_INT_RAW_W<0> {
        EXTMEM_REJECT_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status of external RAM permission\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_int_raw](index.html) module"]
pub struct EXTMEM_REJECT_INT_RAW_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_int_raw::R](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extmem_reject_int_raw::W](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_RAW to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
