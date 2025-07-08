#[doc = "Register `TARGET2_LO` reader"]
pub type R = crate::R<TARGET2_LO_SPEC>;
#[doc = "Register `TARGET2_LO` writer"]
pub type W = crate::W<TARGET2_LO_SPEC>;
#[doc = "Field `TIMER_TARGET2_LO` reader - Configures the alarm value to be loaded to COMP2, low 32 bits."]
pub type TIMER_TARGET2_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET2_LO` writer - Configures the alarm value to be loaded to COMP2, low 32 bits."]
pub type TIMER_TARGET2_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the alarm value to be loaded to COMP2, low 32 bits."]
    #[inline(always)]
    pub fn timer_target2_lo(&self) -> TIMER_TARGET2_LO_R {
        TIMER_TARGET2_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_LO")
            .field("timer_target2_lo", &self.timer_target2_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the alarm value to be loaded to COMP2, low 32 bits."]
    #[inline(always)]
    pub fn timer_target2_lo(&mut self) -> TIMER_TARGET2_LO_W<TARGET2_LO_SPEC> {
        TIMER_TARGET2_LO_W::new(self, 0)
    }
}
#[doc = "Alarm value to be loaded to COMP2, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target2_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target2_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET2_LO_SPEC;
impl crate::RegisterSpec for TARGET2_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target2_lo::R`](R) reader structure"]
impl crate::Readable for TARGET2_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target2_lo::W`](W) writer structure"]
impl crate::Writable for TARGET2_LO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET2_LO to value 0"]
impl crate::Resettable for TARGET2_LO_SPEC {}
