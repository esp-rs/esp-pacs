#[doc = "Register `SDIO_DATE` reader"]
pub struct R(crate::R<SDIO_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_DATE` writer"]
pub struct W(crate::W<SDIO_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_DATE_SPEC>;
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
impl From<crate::W<SDIO_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_DATE` reader - sdio version date."]
pub type SDIO_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SDIO_DATE` writer - sdio version date."]
pub type SDIO_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_DATE_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - sdio version date."]
    #[inline(always)]
    pub fn sdio_date(&self) -> SDIO_DATE_R {
        SDIO_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_DATE")
            .field("sdio_date", &format_args!("{}", self.sdio_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - sdio version date."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_date(&mut self) -> SDIO_DATE_W<0> {
        SDIO_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_date](index.html) module"]
pub struct SDIO_DATE_SPEC;
impl crate::RegisterSpec for SDIO_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_date::R](R) reader structure"]
impl crate::Readable for SDIO_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_date::W](W) writer structure"]
impl crate::Writable for SDIO_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_DATE to value 0x0220_3150"]
impl crate::Resettable for SDIO_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_3150;
}
