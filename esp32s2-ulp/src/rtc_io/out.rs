#[doc = "Register `OUT` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Field `GPIO_OUT_DATA` reader - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
pub type GPIO_OUT_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_OUT_DATA` writer - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
pub type GPIO_OUT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
    #[inline(always)]
    pub fn gpio_out_data(&self) -> GPIO_OUT_DATA_R {
        GPIO_OUT_DATA_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT")
            .field("gpio_out_data", &self.gpio_out_data().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_data(&mut self) -> GPIO_OUT_DATA_W<OUT_SPEC> {
        GPIO_OUT_DATA_W::new(self, 10)
    }
}
#[doc = "RTC GPIO output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
