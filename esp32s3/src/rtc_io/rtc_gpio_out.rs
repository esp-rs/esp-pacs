#[doc = "Register `RTC_GPIO_OUT` reader"]
pub type R = crate::R<RTC_GPIO_OUT_SPEC>;
#[doc = "Register `RTC_GPIO_OUT` writer"]
pub type W = crate::W<RTC_GPIO_OUT_SPEC>;
#[doc = "Field `DATA` reader - RTC GPIO 0 ~ 21 output data"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - RTC GPIO 0 ~ 21 output data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_OUT")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<RTC_GPIO_OUT_SPEC> {
        DATA_W::new(self, 10)
    }
}
#[doc = "RTC GPIO 0 ~ 21 output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_gpio_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_gpio_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_OUT_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_out::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_out::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_OUT to value 0"]
impl crate::Resettable for RTC_GPIO_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
