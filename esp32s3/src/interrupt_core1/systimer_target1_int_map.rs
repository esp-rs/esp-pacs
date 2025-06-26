#[doc = "Register `SYSTIMER_TARGET1_INT_MAP` reader"]
pub type R = crate::R<SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "Register `SYSTIMER_TARGET1_INT_MAP` writer"]
pub type W = crate::W<SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "Field `SYSTIMER_TARGET1_INT_MAP` reader - this register used to map systimer_target1 interrupt to one of core1's external interrupt"]
pub type SYSTIMER_TARGET1_INT_MAP_R = crate::FieldReader;
#[doc = "Field `SYSTIMER_TARGET1_INT_MAP` writer - this register used to map systimer_target1 interrupt to one of core1's external interrupt"]
pub type SYSTIMER_TARGET1_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map systimer_target1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn systimer_target1_int_map(&self) -> SYSTIMER_TARGET1_INT_MAP_R {
        SYSTIMER_TARGET1_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_TARGET1_INT_MAP")
            .field("systimer_target1_int_map", &self.systimer_target1_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map systimer_target1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn systimer_target1_int_map(
        &mut self,
    ) -> SYSTIMER_TARGET1_INT_MAP_W<SYSTIMER_TARGET1_INT_MAP_SPEC> {
        SYSTIMER_TARGET1_INT_MAP_W::new(self, 0)
    }
}
#[doc = "systimer_target1 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target1_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target1_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_TARGET1_INT_MAP_SPEC;
impl crate::RegisterSpec for SYSTIMER_TARGET1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_target1_int_map::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_TARGET1_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_target1_int_map::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_TARGET1_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTIMER_TARGET1_INT_MAP to value 0x10"]
impl crate::Resettable for SYSTIMER_TARGET1_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
