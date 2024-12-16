#[doc = "Register `CORE_1_RCD_RECORDING` reader"]
pub type R = crate::R<CORE_1_RCD_RECORDING_SPEC>;
#[doc = "Register `CORE_1_RCD_RECORDING` writer"]
pub type W = crate::W<CORE_1_RCD_RECORDING_SPEC>;
#[doc = "Field `CORE_1_RCD_RECORDING` reader - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
pub type CORE_1_RCD_RECORDING_R = crate::BitReader;
#[doc = "Field `CORE_1_RCD_RECORDING` writer - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
pub type CORE_1_RCD_RECORDING_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("core_1_rcd_recording", &self.core_1_rcd_recording())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record Core1 pdebug interface signal"]
    #[inline(always)]
    pub fn core_1_rcd_recording(&mut self) -> CORE_1_RCD_RECORDING_W<CORE_1_RCD_RECORDING_SPEC> {
        CORE_1_RCD_RECORDING_W::new(self, 0)
    }
}
#[doc = "Core1 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_rcd_recording::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_rcd_recording::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_RCD_RECORDING_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_RECORDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_rcd_recording::R`](R) reader structure"]
impl crate::Readable for CORE_1_RCD_RECORDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_rcd_recording::W`](W) writer structure"]
impl crate::Writable for CORE_1_RCD_RECORDING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_RCD_RECORDING to value 0"]
impl crate::Resettable for CORE_1_RCD_RECORDING_SPEC {
    const RESET_VALUE: u32 = 0;
}
