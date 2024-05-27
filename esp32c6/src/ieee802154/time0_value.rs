#[doc = "Register `TIME0_VALUE` reader"]
pub type R = crate::R<TIME0_VALUE_SPEC>;
#[doc = "Register `TIME0_VALUE` writer"]
pub type W = crate::W<TIME0_VALUE_SPEC>;
#[doc = "Field `TIMER0_VALUE` reader - "]
pub type TIMER0_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER0_VALUE` writer - "]
pub type TIMER0_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer0_value(&self) -> TIMER0_VALUE_R {
        TIMER0_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME0_VALUE")
            .field("timer0_value", &self.timer0_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_value(&mut self) -> TIMER0_VALUE_W<TIME0_VALUE_SPEC> {
        TIMER0_VALUE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time0_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time0_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME0_VALUE_SPEC;
impl crate::RegisterSpec for TIME0_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time0_value::R`](R) reader structure"]
impl crate::Readable for TIME0_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time0_value::W`](W) writer structure"]
impl crate::Writable for TIME0_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME0_VALUE to value 0"]
impl crate::Resettable for TIME0_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
