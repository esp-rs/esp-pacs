#[doc = "Register `GPIO_RTC_CALIB_VALUE` reader"]
pub type R = crate::R<GPIO_RTC_CALIB_VALUE_SPEC>;
#[doc = "Register `GPIO_RTC_CALIB_VALUE` writer"]
pub type W = crate::W<GPIO_RTC_CALIB_VALUE_SPEC>;
#[doc = "Field `RTC_CALIB_VALUE` reader - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
pub type RTC_CALIB_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_CALIB_VALUE` writer - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
pub type RTC_CALIB_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `RTC_CALIB_RDY_REAL` reader - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_REAL_R = crate::BitReader;
#[doc = "Field `RTC_CALIB_RDY_REAL` writer - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_REAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC_CALIB_RDY` reader - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_R = crate::BitReader;
#[doc = "Field `RTC_CALIB_RDY` writer - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:19 - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
    #[inline(always)]
    pub fn rtc_calib_value(&self) -> RTC_CALIB_VALUE_R {
        RTC_CALIB_VALUE_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 30 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    pub fn rtc_calib_rdy_real(&self) -> RTC_CALIB_RDY_REAL_R {
        RTC_CALIB_RDY_REAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    pub fn rtc_calib_rdy(&self) -> RTC_CALIB_RDY_R {
        RTC_CALIB_RDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_RTC_CALIB_VALUE")
            .field(
                "rtc_calib_rdy",
                &format_args!("{}", self.rtc_calib_rdy().bit()),
            )
            .field(
                "rtc_calib_rdy_real",
                &format_args!("{}", self.rtc_calib_rdy_real().bit()),
            )
            .field(
                "rtc_calib_value",
                &format_args!("{}", self.rtc_calib_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_RTC_CALIB_VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_calib_value(&mut self) -> RTC_CALIB_VALUE_W<GPIO_RTC_CALIB_VALUE_SPEC, 0> {
        RTC_CALIB_VALUE_W::new(self)
    }
    #[doc = "Bit 30 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_calib_rdy_real(&mut self) -> RTC_CALIB_RDY_REAL_W<GPIO_RTC_CALIB_VALUE_SPEC, 30> {
        RTC_CALIB_RDY_REAL_W::new(self)
    }
    #[doc = "Bit 31 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_calib_rdy(&mut self) -> RTC_CALIB_RDY_W<GPIO_RTC_CALIB_VALUE_SPEC, 31> {
        RTC_CALIB_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "0: during RTC-clock-calibration; 1: RTC-clock-calibration is done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_rtc_calib_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_rtc_calib_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_RTC_CALIB_VALUE_SPEC;
impl crate::RegisterSpec for GPIO_RTC_CALIB_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_rtc_calib_value::R`](R) reader structure"]
impl crate::Readable for GPIO_RTC_CALIB_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_rtc_calib_value::W`](W) writer structure"]
impl crate::Writable for GPIO_RTC_CALIB_VALUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_RTC_CALIB_VALUE to value 0"]
impl crate::Resettable for GPIO_RTC_CALIB_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
