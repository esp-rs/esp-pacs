#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TRIGGER_SPEC>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TRIGGER_SPEC>;
#[doc = "Field `ON` writer - Configure whether to enable the encoder.\\\\0: Invalid \\\\1: Enable\\\\"]
pub type ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF` writer - Configure whether to disable the encoder.\\\\0: Invalid \\\\1: Disable\\\\"]
pub type OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_LOOP` reader - Configure the memory writing mode. \\\\0: Non-loop mode. \\\\1: Loop mode\\\\"]
pub type MEM_LOOP_R = crate::BitReader;
#[doc = "Field `MEM_LOOP` writer - Configure the memory writing mode. \\\\0: Non-loop mode. \\\\1: Loop mode\\\\"]
pub type MEM_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART_ENA` reader - Configure whether or not enable automatic restart function for the encoder.\\\\0: Disable\\\\1: Enable\\\\"]
pub type RESTART_ENA_R = crate::BitReader;
#[doc = "Field `RESTART_ENA` writer - Configure whether or not enable automatic restart function for the encoder.\\\\0: Disable\\\\1: Enable\\\\"]
pub type RESTART_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Configure the memory writing mode. \\\\0: Non-loop mode. \\\\1: Loop mode\\\\"]
    #[inline(always)]
    pub fn mem_loop(&self) -> MEM_LOOP_R {
        MEM_LOOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure whether or not enable automatic restart function for the encoder.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn restart_ena(&self) -> RESTART_ENA_R {
        RESTART_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGGER")
            .field("mem_loop", &self.mem_loop())
            .field("restart_ena", &self.restart_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether to enable the encoder.\\\\0: Invalid \\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W<'_, TRIGGER_SPEC> {
        ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure whether to disable the encoder.\\\\0: Invalid \\\\1: Disable\\\\"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W<'_, TRIGGER_SPEC> {
        OFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configure the memory writing mode. \\\\0: Non-loop mode. \\\\1: Loop mode\\\\"]
    #[inline(always)]
    pub fn mem_loop(&mut self) -> MEM_LOOP_W<'_, TRIGGER_SPEC> {
        MEM_LOOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configure whether or not enable automatic restart function for the encoder.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn restart_ena(&mut self) -> RESTART_ENA_W<'_, TRIGGER_SPEC> {
        RESTART_ENA_W::new(self, 3)
    }
}
#[doc = "Trace enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0x0c"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
