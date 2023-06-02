#[doc = "Register `DOEPDMAB2` reader"]
pub struct R(crate::R<DOEPDMAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMAB2` writer"]
pub struct W(crate::W<DOEPDMAB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMAB2_SPEC>;
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
impl From<crate::W<DOEPDMAB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMAB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABUFFERADDR2` reader - "]
pub type DMABUFFERADDR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMABUFFERADDR2` writer - "]
pub type DMABUFFERADDR2_W<'a, const O: u8> =
    crate::FieldWriter<'a, DOEPDMAB2_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr2(&self) -> DMABUFFERADDR2_R {
        DMABUFFERADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB2")
            .field(
                "dmabufferaddr2",
                &format_args!("{}", self.dmabufferaddr2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr2(&mut self) -> DMABUFFERADDR2_W<0> {
        DMABUFFERADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdmab2](index.html) module"]
pub struct DOEPDMAB2_SPEC;
impl crate::RegisterSpec for DOEPDMAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdmab2::R](R) reader structure"]
impl crate::Readable for DOEPDMAB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdmab2::W](W) writer structure"]
impl crate::Writable for DOEPDMAB2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB2 to value 0"]
impl crate::Resettable for DOEPDMAB2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
