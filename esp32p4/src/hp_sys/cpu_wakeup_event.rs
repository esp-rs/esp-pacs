#[doc = "Register `CPU_WAKEUP_EVENT` reader"]
pub type R = crate::R<CPU_WAKEUP_EVENT_SPEC>;
#[doc = "Register `CPU_WAKEUP_EVENT` writer"]
pub type W = crate::W<CPU_WAKEUP_EVENT_SPEC>;
#[doc = "Field `CORE0_WAKEUP_EVENT` reader - Set this bit to wake up hp core0"]
pub type CORE0_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `CORE0_WAKEUP_EVENT` writer - Set this bit to wake up hp core0"]
pub type CORE0_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_WAKEUP_EVENT` reader - Set this bit to wake up hp core1"]
pub type CORE1_WAKEUP_EVENT_R = crate::BitReader;
#[doc = "Field `CORE1_WAKEUP_EVENT` writer - Set this bit to wake up hp core1"]
pub type CORE1_WAKEUP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to wake up hp core0"]
    #[inline(always)]
    pub fn core0_wakeup_event(&self) -> CORE0_WAKEUP_EVENT_R {
        CORE0_WAKEUP_EVENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to wake up hp core1"]
    #[inline(always)]
    pub fn core1_wakeup_event(&self) -> CORE1_WAKEUP_EVENT_R {
        CORE1_WAKEUP_EVENT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAKEUP_EVENT")
            .field("core0_wakeup_event", &self.core0_wakeup_event())
            .field("core1_wakeup_event", &self.core1_wakeup_event())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to wake up hp core0"]
    #[inline(always)]
    pub fn core0_wakeup_event(&mut self) -> CORE0_WAKEUP_EVENT_W<'_, CPU_WAKEUP_EVENT_SPEC> {
        CORE0_WAKEUP_EVENT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to wake up hp core1"]
    #[inline(always)]
    pub fn core1_wakeup_event(&mut self) -> CORE1_WAKEUP_EVENT_W<'_, CPU_WAKEUP_EVENT_SPEC> {
        CORE1_WAKEUP_EVENT_W::new(self, 1)
    }
}
#[doc = "cpu wakeup event ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wakeup_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeup_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
