#[doc = "Register `CPU_WAKEUP_EVENT` reader"]
pub type R = crate::R<CPU_WAKEUP_EVENT_SPEC>;
#[doc = "Register `CPU_WAKEUP_EVENT` writer"]
pub type W = crate::W<CPU_WAKEUP_EVENT_SPEC>;
#[doc = "Field `CPU0_WAKEUP_EVENT` reader - Configures the cpu0 to exit WFI mode"]
pub type CPU0_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `CPU0_WAKEUP_EVENT` writer - Configures the cpu0 to exit WFI mode"]
pub type CPU0_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1_WAKEUP_EVENT` reader - Configures the cpu1 to exit WFI mode"]
pub type CPU1_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `CPU1_WAKEUP_EVENT` writer - Configures the cpu1 to exit WFI mode"]
pub type CPU1_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the cpu0 to exit WFI mode"]
    #[inline(always)]
    pub fn cpu0_wakeup_event(&self) -> CPU0_WAKEUP_EVENT_R {
        CPU0_WAKEUP_EVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the cpu1 to exit WFI mode"]
    #[inline(always)]
    pub fn cpu1_wakeup_event(&self) -> CPU1_WAKEUP_EVENT_R {
        CPU1_WAKEUP_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAKEUP_EVENT")
            .field("cpu0_wakeup_event", &self.cpu0_wakeup_event())
            .field("cpu1_wakeup_event", &self.cpu1_wakeup_event())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the cpu0 to exit WFI mode"]
    #[inline(always)]
    pub fn cpu0_wakeup_event(&mut self) -> CPU0_WAKEUP_EVENT_W<'_, CPU_WAKEUP_EVENT_SPEC> {
        CPU0_WAKEUP_EVENT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the cpu1 to exit WFI mode"]
    #[inline(always)]
    pub fn cpu1_wakeup_event(&mut self) -> CPU1_WAKEUP_EVENT_W<'_, CPU_WAKEUP_EVENT_SPEC> {
        CPU1_WAKEUP_EVENT_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wakeup_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeup_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_WAKEUP_EVENT_SPEC;
impl crate::RegisterSpec for CPU_WAKEUP_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_wakeup_event::R`](R) reader structure"]
impl crate::Readable for CPU_WAKEUP_EVENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_wakeup_event::W`](W) writer structure"]
impl crate::Writable for CPU_WAKEUP_EVENT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WAKEUP_EVENT to value 0"]
impl crate::Resettable for CPU_WAKEUP_EVENT_SPEC {}
