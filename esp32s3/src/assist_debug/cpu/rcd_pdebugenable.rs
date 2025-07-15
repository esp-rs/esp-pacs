#[doc = "Register `RCD_PDEBUGENABLE` reader"]
pub type R = crate::R<RCD_PDEBUGENABLE_SPEC>;
#[doc = "Register `RCD_PDEBUGENABLE` writer"]
pub type W = crate::W<RCD_PDEBUGENABLE_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGENABLE` reader - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
pub type CORE_0_RCD_PDEBUGENABLE_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_PDEBUGENABLE` writer - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
pub type CORE_0_RCD_PDEBUGENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugenable(&self) -> CORE_0_RCD_PDEBUGENABLE_R {
        CORE_0_RCD_PDEBUGENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_PDEBUGENABLE")
            .field("core_0_rcd_pdebugenable", &self.core_0_rcd_pdebugenable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugenable(&mut self) -> CORE_0_RCD_PDEBUGENABLE_W<RCD_PDEBUGENABLE_SPEC> {
        CORE_0_RCD_PDEBUGENABLE_W::new(self, 0)
    }
}
#[doc = "core0 pdebug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcd_pdebugenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_PDEBUGENABLE_SPEC;
impl crate::RegisterSpec for RCD_PDEBUGENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_pdebugenable::R`](R) reader structure"]
impl crate::Readable for RCD_PDEBUGENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcd_pdebugenable::W`](W) writer structure"]
impl crate::Writable for RCD_PDEBUGENABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCD_PDEBUGENABLE to value 0"]
impl crate::Resettable for RCD_PDEBUGENABLE_SPEC {}
