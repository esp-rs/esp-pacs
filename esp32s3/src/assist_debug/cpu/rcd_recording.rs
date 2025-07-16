#[doc = "Register `RCD_RECORDING` reader"]
pub type R = crate::R<RCD_RECORDING_SPEC>;
#[doc = "Register `RCD_RECORDING` writer"]
pub type W = crate::W<RCD_RECORDING_SPEC>;
#[doc = "Field `RCD_RECORDING` reader - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
pub type RCD_RECORDING_R = crate::BitReader;
#[doc = "Field `RCD_RECORDING` writer - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
pub type RCD_RECORDING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
    #[inline(always)]
    pub fn rcd_recording(&self) -> RCD_RECORDING_R {
        RCD_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_RECORDING")
            .field("rcd_recording", &self.rcd_recording())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pdebug record enable,set 1 to record core0 pdebug interface signal"]
    #[inline(always)]
    pub fn rcd_recording(&mut self) -> RCD_RECORDING_W<RCD_RECORDING_SPEC> {
        RCD_RECORDING_W::new(self, 0)
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_recording::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcd_recording::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_RECORDING_SPEC;
impl crate::RegisterSpec for RCD_RECORDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_recording::R`](R) reader structure"]
impl crate::Readable for RCD_RECORDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcd_recording::W`](W) writer structure"]
impl crate::Writable for RCD_RECORDING_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCD_RECORDING to value 0"]
impl crate::Resettable for RCD_RECORDING_SPEC {}
