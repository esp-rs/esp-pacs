#[doc = "Register `NTIMERS_DATE` reader"]
pub struct R(crate::R<NTIMERS_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTIMERS_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTIMERS_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTIMERS_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTIMERS_DATE` writer"]
pub struct W(crate::W<NTIMERS_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTIMERS_DATE_SPEC>;
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
impl From<crate::W<NTIMERS_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTIMERS_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTIMERS_DATE` reader - Timer version control register"]
pub type NTIMERS_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `NTIMERS_DATE` writer - Timer version control register"]
pub type NTIMERS_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, NTIMERS_DATE_SPEC, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - Timer version control register"]
    #[inline(always)]
    pub fn ntimers_date(&self) -> NTIMERS_DATE_R {
        NTIMERS_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NTIMERS_DATE")
            .field(
                "ntimers_date",
                &format_args!("{}", self.ntimers_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<NTIMERS_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Timer version control register"]
    #[inline(always)]
    #[must_use]
    pub fn ntimers_date(&mut self) -> NTIMERS_DATE_W<0> {
        NTIMERS_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer version control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntimers_date](index.html) module"]
pub struct NTIMERS_DATE_SPEC;
impl crate::RegisterSpec for NTIMERS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntimers_date::R](R) reader structure"]
impl crate::Readable for NTIMERS_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntimers_date::W](W) writer structure"]
impl crate::Writable for NTIMERS_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTIMERS_DATE to value 0x0200_3071"]
impl crate::Resettable for NTIMERS_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_3071;
}
