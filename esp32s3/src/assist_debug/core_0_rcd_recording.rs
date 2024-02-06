#[doc = "Register `CORE_0_RCD_RECORDING` reader"]
pub type R = crate::R<CORE_0_RCD_RECORDING_SPEC>;
#[doc = "Register `CORE_0_RCD_RECORDING` writer"]
pub type W = crate::W<CORE_0_RCD_RECORDING_SPEC>;
#[doc = "Field `CORE_0_RCD_RECORDING` reader - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
pub type CORE_0_RCD_RECORDING_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_RECORDING` writer - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
pub type CORE_0_RCD_RECORDING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
    #[inline(always)]
    pub fn core_0_rcd_recording(&self) -> CORE_0_RCD_RECORDING_R {
        CORE_0_RCD_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_RECORDING")
            .field(
                "core_0_rcd_recording",
                &format_args!("{}", self.core_0_rcd_recording().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_RECORDING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_rcd_recording(&mut self) -> CORE_0_RCD_RECORDING_W<CORE_0_RCD_RECORDING_SPEC> {
        CORE_0_RCD_RECORDING_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_recording::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_recording::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_RECORDING_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_RECORDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_recording::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_RECORDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_rcd_recording::W`](W) writer structure"]
impl crate::Writable for CORE_0_RCD_RECORDING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_RCD_RECORDING to value 0"]
impl crate::Resettable for CORE_0_RCD_RECORDING_SPEC {
    const RESET_VALUE: u32 = 0;
}
