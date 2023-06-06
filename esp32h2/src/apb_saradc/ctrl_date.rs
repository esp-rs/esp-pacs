#[doc = "Register `CTRL_DATE` reader"]
pub struct R(crate::R<CTRL_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_DATE` writer"]
pub struct W(crate::W<CTRL_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_DATE_SPEC>;
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
impl From<crate::W<CTRL_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATE` reader - version"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - version"]
pub type DATE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL_DATE_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - version"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_DATE")
            .field("date", &format_args!("{}", self.date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - version"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<0> {
        DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "version\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_date](index.html) module"]
pub struct CTRL_DATE_SPEC;
impl crate::RegisterSpec for CTRL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_date::R](R) reader structure"]
impl crate::Readable for CTRL_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_date::W](W) writer structure"]
impl crate::Writable for CTRL_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_DATE to value 0x0220_6240"]
impl crate::Resettable for CTRL_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_6240;
}
