#[doc = "Register `TARGET0_LO` reader"]
pub type R = crate::R<TARGET0_LO_SPEC>;
#[doc = "Register `TARGET0_LO` writer"]
pub type W = crate::W<TARGET0_LO_SPEC>;
#[doc = "Field `TIMER_TARGET0_LO` reader - Configures the alarm value to be loaded to COMP0, low 32 bits."]
pub type TIMER_TARGET0_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET0_LO` writer - Configures the alarm value to be loaded to COMP0, low 32 bits."]
pub type TIMER_TARGET0_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the alarm value to be loaded to COMP0, low 32 bits."]
    #[inline(always)]
    pub fn timer_target0_lo(&self) -> TIMER_TARGET0_LO_R {
        TIMER_TARGET0_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET0_LO")
            .field("timer_target0_lo", &self.timer_target0_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the alarm value to be loaded to COMP0, low 32 bits."]
    #[inline(always)]
    pub fn timer_target0_lo(&mut self) -> TIMER_TARGET0_LO_W<TARGET0_LO_SPEC> {
        TIMER_TARGET0_LO_W::new(self, 0)
    }
}
#[doc = "Alarm value to be loaded to COMP0, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target0_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target0_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET0_LO_SPEC;
impl crate::RegisterSpec for TARGET0_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target0_lo::R`](R) reader structure"]
impl crate::Readable for TARGET0_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target0_lo::W`](W) writer structure"]
impl crate::Writable for TARGET0_LO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET0_LO to value 0"]
impl crate::Resettable for TARGET0_LO_SPEC {}
