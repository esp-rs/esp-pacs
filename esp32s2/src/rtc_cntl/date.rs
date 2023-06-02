#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTL_DATE` reader - "]
pub type CNTL_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTL_DATE` writer - "]
pub type CNTL_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, DATE_SPEC, 28, O, u32, u32>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn cntl_date(&self) -> CNTL_DATE_R {
        CNTL_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("cntl_date", &format_args!("{}", self.cntl_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    #[must_use]
    pub fn cntl_date(&mut self) -> CNTL_DATE_W<0> {
        CNTL_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0190_6191"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0190_6191;
}
