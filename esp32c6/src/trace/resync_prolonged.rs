#[doc = "Register `RESYNC_PROLONGED` reader"]
pub struct R(crate::R<RESYNC_PROLONGED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESYNC_PROLONGED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESYNC_PROLONGED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESYNC_PROLONGED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESYNC_PROLONGED` writer"]
pub struct W(crate::W<RESYNC_PROLONGED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESYNC_PROLONGED_SPEC>;
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
impl From<crate::W<RESYNC_PROLONGED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESYNC_PROLONGED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESYNC_PROLONGED` reader - count number, when count to this value, send a sync package"]
pub type RESYNC_PROLONGED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESYNC_PROLONGED` writer - count number, when count to this value, send a sync package"]
pub type RESYNC_PROLONGED_W<'a, const O: u8> =
    crate::FieldWriter<'a, RESYNC_PROLONGED_SPEC, 24, O, u32, u32>;
#[doc = "Field `RESYNC_MODE` reader - resyc mode sel: 0: default, cycle count 1: package num count"]
pub type RESYNC_MODE_R = crate::BitReader;
#[doc = "Field `RESYNC_MODE` writer - resyc mode sel: 0: default, cycle count 1: package num count"]
pub type RESYNC_MODE_W<'a, const O: u8> = crate::BitWriter<'a, RESYNC_PROLONGED_SPEC, O>;
impl R {
    #[doc = "Bits 0:23 - count number, when count to this value, send a sync package"]
    #[inline(always)]
    pub fn resync_prolonged(&self) -> RESYNC_PROLONGED_R {
        RESYNC_PROLONGED_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - resyc mode sel: 0: default, cycle count 1: package num count"]
    #[inline(always)]
    pub fn resync_mode(&self) -> RESYNC_MODE_R {
        RESYNC_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESYNC_PROLONGED")
            .field(
                "resync_prolonged",
                &format_args!("{}", self.resync_prolonged().bits()),
            )
            .field("resync_mode", &format_args!("{}", self.resync_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESYNC_PROLONGED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - count number, when count to this value, send a sync package"]
    #[inline(always)]
    #[must_use]
    pub fn resync_prolonged(&mut self) -> RESYNC_PROLONGED_W<0> {
        RESYNC_PROLONGED_W::new(self)
    }
    #[doc = "Bit 24 - resyc mode sel: 0: default, cycle count 1: package num count"]
    #[inline(always)]
    #[must_use]
    pub fn resync_mode(&mut self) -> RESYNC_MODE_W<24> {
        RESYNC_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "resync configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resync_prolonged](index.html) module"]
pub struct RESYNC_PROLONGED_SPEC;
impl crate::RegisterSpec for RESYNC_PROLONGED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resync_prolonged::R](R) reader structure"]
impl crate::Readable for RESYNC_PROLONGED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resync_prolonged::W](W) writer structure"]
impl crate::Writable for RESYNC_PROLONGED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESYNC_PROLONGED to value 0x80"]
impl crate::Resettable for RESYNC_PROLONGED_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
