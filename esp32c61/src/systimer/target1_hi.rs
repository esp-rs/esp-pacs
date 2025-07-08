#[doc = "Register `TARGET1_HI` reader"]
pub type R = crate::R<TARGET1_HI_SPEC>;
#[doc = "Register `TARGET1_HI` writer"]
pub type W = crate::W<TARGET1_HI_SPEC>;
#[doc = "Field `TIMER_TARGET1_HI` reader - Configures the alarm value to be loaded to COMP1, high 20 bits."]
pub type TIMER_TARGET1_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET1_HI` writer - Configures the alarm value to be loaded to COMP1, high 20 bits."]
pub type TIMER_TARGET1_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Configures the alarm value to be loaded to COMP1, high 20 bits."]
    #[inline(always)]
    pub fn timer_target1_hi(&self) -> TIMER_TARGET1_HI_R {
        TIMER_TARGET1_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET1_HI")
            .field("timer_target1_hi", &self.timer_target1_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Configures the alarm value to be loaded to COMP1, high 20 bits."]
    #[inline(always)]
    pub fn timer_target1_hi(&mut self) -> TIMER_TARGET1_HI_W<TARGET1_HI_SPEC> {
        TIMER_TARGET1_HI_W::new(self, 0)
    }
}
#[doc = "Alarm value to be loaded to COMP1, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target1_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target1_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET1_HI_SPEC;
impl crate::RegisterSpec for TARGET1_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target1_hi::R`](R) reader structure"]
impl crate::Readable for TARGET1_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target1_hi::W`](W) writer structure"]
impl crate::Writable for TARGET1_HI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET1_HI to value 0"]
impl crate::Resettable for TARGET1_HI_SPEC {}
