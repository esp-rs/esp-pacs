#[doc = "Register `CORE_0_RCD_RECORDING` reader"]
pub struct R(crate::R<CORE_0_RCD_RECORDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_RECORDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_RECORDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_RECORDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_RCD_RECORDING` writer"]
pub struct W(crate::W<CORE_0_RCD_RECORDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_RCD_RECORDING_SPEC>;
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
impl From<crate::W<CORE_0_RCD_RECORDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_RCD_RECORDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_RCD_RECORDING` reader - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
pub type CORE_0_RCD_RECORDING_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_RCD_RECORDING` writer - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
pub type CORE_0_RCD_RECORDING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_0_RCD_RECORDING_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
    #[inline(always)]
    pub fn core_0_rcd_recording(&self) -> CORE_0_RCD_RECORDING_R {
        CORE_0_RCD_RECORDING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
    #[inline(always)]
    pub fn core_0_rcd_recording(&mut self) -> CORE_0_RCD_RECORDING_W<0> {
        CORE_0_RCD_RECORDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_recording](index.html) module"]
pub struct CORE_0_RCD_RECORDING_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_RECORDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_recording::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_RECORDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_rcd_recording::W](W) writer structure"]
impl crate::Writable for CORE_0_RCD_RECORDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_RCD_RECORDING to value 0"]
impl crate::Resettable for CORE_0_RCD_RECORDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
