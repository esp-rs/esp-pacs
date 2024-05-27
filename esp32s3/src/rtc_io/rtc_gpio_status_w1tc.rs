///Register `RTC_GPIO_STATUS_W1TC` writer
pub type W = crate::W<RTC_GPIO_STATUS_W1TC_SPEC>;
///Field `RTC_GPIO_STATUS_INT_W1TC` writer - RTC GPIO 0 ~ 21 interrupt status write 1 to clear
pub type RTC_GPIO_STATUS_INT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_GPIO_STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 10:31 - RTC GPIO 0 ~ 21 interrupt status write 1 to clear
    #[inline(always)]
    #[must_use]
    pub fn rtc_gpio_status_int_w1tc(
        &mut self,
    ) -> RTC_GPIO_STATUS_INT_W1TC_W<RTC_GPIO_STATUS_W1TC_SPEC> {
        RTC_GPIO_STATUS_INT_W1TC_W::new(self, 10)
    }
}
/**One clear RTC GPIO 0 ~ 21 interrupt status

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTC_GPIO_STATUS_W1TC_SPEC;
impl crate::RegisterSpec for RTC_GPIO_STATUS_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rtc_gpio_status_w1tc::W`](W) writer structure
impl crate::Writable for RTC_GPIO_STATUS_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_GPIO_STATUS_W1TC to value 0
impl crate::Resettable for RTC_GPIO_STATUS_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
