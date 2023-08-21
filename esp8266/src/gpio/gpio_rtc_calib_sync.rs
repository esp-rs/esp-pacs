#[doc = "Register `GPIO_RTC_CALIB_SYNC` reader"]
pub type R = crate::R<GPIO_RTC_CALIB_SYNC_SPEC>;
#[doc = "Register `GPIO_RTC_CALIB_SYNC` writer"]
pub type W = crate::W<GPIO_RTC_CALIB_SYNC_SPEC>;
#[doc = "Field `RTC_PERIOD_NUM` reader - The cycle number of RTC-clock during RTC-clock-calibration"]
pub type RTC_PERIOD_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_PERIOD_NUM` writer - The cycle number of RTC-clock during RTC-clock-calibration"]
pub type RTC_PERIOD_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `RTC_CALIB_START` reader - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
pub type RTC_CALIB_START_R = crate::BitReader;
#[doc = "Field `RTC_CALIB_START` writer - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
pub type RTC_CALIB_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - The cycle number of RTC-clock during RTC-clock-calibration"]
    #[inline(always)]
    pub fn rtc_period_num(&self) -> RTC_PERIOD_NUM_R {
        RTC_PERIOD_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
    #[inline(always)]
    pub fn rtc_calib_start(&self) -> RTC_CALIB_START_R {
        RTC_CALIB_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_RTC_CALIB_SYNC")
            .field(
                "rtc_calib_start",
                &format_args!("{}", self.rtc_calib_start().bit()),
            )
            .field(
                "rtc_period_num",
                &format_args!("{}", self.rtc_period_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_RTC_CALIB_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The cycle number of RTC-clock during RTC-clock-calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_period_num(&mut self) -> RTC_PERIOD_NUM_W<GPIO_RTC_CALIB_SYNC_SPEC, 0> {
        RTC_PERIOD_NUM_W::new(self)
    }
    #[doc = "Bit 31 - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_calib_start(&mut self) -> RTC_CALIB_START_W<GPIO_RTC_CALIB_SYNC_SPEC, 31> {
        RTC_CALIB_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Positvie edge of this bit will trigger the RTC-clock-calibration process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_rtc_calib_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_rtc_calib_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_RTC_CALIB_SYNC_SPEC;
impl crate::RegisterSpec for GPIO_RTC_CALIB_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_rtc_calib_sync::R`](R) reader structure"]
impl crate::Readable for GPIO_RTC_CALIB_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_rtc_calib_sync::W`](W) writer structure"]
impl crate::Writable for GPIO_RTC_CALIB_SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_RTC_CALIB_SYNC to value 0"]
impl crate::Resettable for GPIO_RTC_CALIB_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
