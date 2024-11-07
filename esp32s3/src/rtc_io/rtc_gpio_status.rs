#[doc = "Register `RTC_GPIO_STATUS` reader"]
pub type R = crate::R<RTC_GPIO_STATUS_SPEC>;
#[doc = "Register `RTC_GPIO_STATUS` writer"]
pub type W = crate::W<RTC_GPIO_STATUS_SPEC>;
#[doc = "Field `INT` reader - RTC GPIO 0 ~ 21 interrupt status"]
pub type INT_R = crate::FieldReader<u32>;
#[doc = "Field `INT` writer - RTC GPIO 0 ~ 21 interrupt status"]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 interrupt status"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_STATUS")
            .field("int", &self.int())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 interrupt status"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W<RTC_GPIO_STATUS_SPEC> {
        INT_W::new(self, 10)
    }
}
#[doc = "RTC GPIO 0 ~ 21 interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_gpio_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_gpio_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_STATUS_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_status::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_gpio_status::W`](W) writer structure"]
impl crate::Writable for RTC_GPIO_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_GPIO_STATUS to value 0"]
impl crate::Resettable for RTC_GPIO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
