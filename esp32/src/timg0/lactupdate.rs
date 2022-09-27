#[doc = "Register `LACTUPDATE` writer"]
pub struct W(crate::W<LACTUPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTUPDATE_SPEC>;
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
impl From<crate::W<LACTUPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTUPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_UPDATE` writer - "]
pub type LACT_UPDATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LACTUPDATE_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_update(&mut self) -> LACT_UPDATE_W<0> {
        LACT_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactupdate](index.html) module"]
pub struct LACTUPDATE_SPEC;
impl crate::RegisterSpec for LACTUPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lactupdate::W](W) writer structure"]
impl crate::Writable for LACTUPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LACTUPDATE to value 0"]
impl crate::Resettable for LACTUPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
