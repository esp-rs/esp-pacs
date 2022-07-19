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
#[doc = "Field `NTIMGS_DATE` reader - Timer version control register"]
pub type NTIMGS_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NTIMGS_DATE` writer - Timer version control register"]
pub type NTIMGS_DATE_W<'a> = crate::FieldWriter<'a, u32, NTIMERS_DATE_SPEC, u32, u32, 28, 0>;
impl R {
    #[doc = "Bits 0:27 - Timer version control register"]
    #[inline(always)]
    pub fn ntimgs_date(&self) -> NTIMGS_DATE_R {
        NTIMGS_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Timer version control register"]
    #[inline(always)]
    pub fn ntimgs_date(&mut self) -> NTIMGS_DATE_W {
        NTIMGS_DATE_W::new(self)
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
}
#[doc = "`reset()` method sets NTIMERS_DATE to value 0x0200_6191"]
impl crate::Resettable for NTIMERS_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_6191
    }
}
