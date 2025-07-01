#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `GPIO_STATUS_INT` reader - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
pub type GPIO_STATUS_INT_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_STATUS_INT` writer - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
pub type GPIO_STATUS_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
    #[inline(always)]
    pub fn gpio_status_int(&self) -> GPIO_STATUS_INT_R {
        GPIO_STATUS_INT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("gpio_status_int", &self.gpio_status_int())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 interrupt status register. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. This register should be used together with RTCIO_RTC_GPIO_PINn_INT_TYPE in RTCIO_RTC_GPIO_PINn_REG. 0: no interrupt; 1: corresponding interrupt."]
    #[inline(always)]
    pub fn gpio_status_int(&mut self) -> GPIO_STATUS_INT_W<STATUS_SPEC> {
        GPIO_STATUS_INT_W::new(self, 10)
    }
}
#[doc = "RTC GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
