#[doc = "Register `TARGET2_HI` reader"]
pub type R = crate::R<TARGET2_HI_SPEC>;
#[doc = "Register `TARGET2_HI` writer"]
pub type W = crate::W<TARGET2_HI_SPEC>;
#[doc = "Field `TIMER_TARGET2_HI` reader - Configures the alarm value to be loaded to COMP2, high 20 bits."]
pub type TIMER_TARGET2_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET2_HI` writer - Configures the alarm value to be loaded to COMP2, high 20 bits."]
pub type TIMER_TARGET2_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Configures the alarm value to be loaded to COMP2, high 20 bits."]
    #[inline(always)]
    pub fn timer_target2_hi(&self) -> TIMER_TARGET2_HI_R {
        TIMER_TARGET2_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_HI")
            .field("timer_target2_hi", &self.timer_target2_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Configures the alarm value to be loaded to COMP2, high 20 bits."]
    #[inline(always)]
    pub fn timer_target2_hi(&mut self) -> TIMER_TARGET2_HI_W<TARGET2_HI_SPEC> {
        TIMER_TARGET2_HI_W::new(self, 0)
    }
}
#[doc = "Alarm value to be loaded to COMP2, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target2_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target2_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET2_HI_SPEC;
impl crate::RegisterSpec for TARGET2_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target2_hi::R`](R) reader structure"]
impl crate::Readable for TARGET2_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target2_hi::W`](W) writer structure"]
impl crate::Writable for TARGET2_HI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET2_HI to value 0"]
impl crate::Resettable for TARGET2_HI_SPEC {}
