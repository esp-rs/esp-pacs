#[doc = "Register `CORE_1_RCD_RECORDING` reader"]
pub struct R(crate::R<CORE_1_RCD_RECORDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_RECORDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_RECORDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_RECORDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_RCD_RECORDING` writer"]
pub struct W(crate::W<CORE_1_RCD_RECORDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_RCD_RECORDING_SPEC>;
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
impl From<crate::W<CORE_1_RCD_RECORDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_RCD_RECORDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_RCD_RECORDING` reader - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
pub type CORE_1_RCD_RECORDING_R = crate::BitReader;
#[doc = "Field `CORE_1_RCD_RECORDING` writer - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
pub type CORE_1_RCD_RECORDING_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_RCD_RECORDING_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
    #[inline(always)]
    pub fn core_1_rcd_recording(&self) -> CORE_1_RCD_RECORDING_R {
        CORE_1_RCD_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_RECORDING")
            .field(
                "core_1_rcd_recording",
                &format_args!("{}", self.core_1_rcd_recording().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_RCD_RECORDING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_rcd_recording(&mut self) -> CORE_1_RCD_RECORDING_W<0> {
        CORE_1_RCD_RECORDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_recording](index.html) module"]
pub struct CORE_1_RCD_RECORDING_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_RECORDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_recording::R](R) reader structure"]
impl crate::Readable for CORE_1_RCD_RECORDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_rcd_recording::W](W) writer structure"]
impl crate::Writable for CORE_1_RCD_RECORDING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_RCD_RECORDING to value 0"]
impl crate::Resettable for CORE_1_RCD_RECORDING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
