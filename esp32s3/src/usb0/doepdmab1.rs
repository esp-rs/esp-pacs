#[doc = "Register `DOEPDMAB1` reader"]
pub struct R(crate::R<DOEPDMAB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMAB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMAB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMAB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMAB1` writer"]
pub struct W(crate::W<DOEPDMAB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMAB1_SPEC>;
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
impl From<crate::W<DOEPDMAB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMAB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABUFFERADDR1` reader - "]
pub type DMABUFFERADDR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMABUFFERADDR1` writer - "]
pub type DMABUFFERADDR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, DOEPDMAB1_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr1(&self) -> DMABUFFERADDR1_R {
        DMABUFFERADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB1")
            .field(
                "dmabufferaddr1",
                &format_args!("{}", self.dmabufferaddr1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr1(&mut self) -> DMABUFFERADDR1_W<0> {
        DMABUFFERADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdmab1](index.html) module"]
pub struct DOEPDMAB1_SPEC;
impl crate::RegisterSpec for DOEPDMAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdmab1::R](R) reader structure"]
impl crate::Readable for DOEPDMAB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdmab1::W](W) writer structure"]
impl crate::Writable for DOEPDMAB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB1 to value 0"]
impl crate::Resettable for DOEPDMAB1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
