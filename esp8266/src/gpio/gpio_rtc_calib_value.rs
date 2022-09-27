#[doc = "Register `GPIO_RTC_CALIB_VALUE` reader"]
pub struct R(crate::R<GPIO_RTC_CALIB_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_RTC_CALIB_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_RTC_CALIB_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_RTC_CALIB_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_RTC_CALIB_VALUE` writer"]
pub struct W(crate::W<GPIO_RTC_CALIB_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_RTC_CALIB_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_RTC_CALIB_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_RTC_CALIB_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CALIB_VALUE` reader - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
pub type RTC_CALIB_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_CALIB_VALUE` writer - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
pub type RTC_CALIB_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_RTC_CALIB_VALUE_SPEC, u32, u32, 20, O>;
#[doc = "Field `RTC_CALIB_RDY_REAL` reader - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_REAL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CALIB_RDY_REAL` writer - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_REAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_RTC_CALIB_VALUE_SPEC, bool, O>;
#[doc = "Field `RTC_CALIB_RDY` reader - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_R = crate::BitReader<bool>;
#[doc = "Field `RTC_CALIB_RDY` writer - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub type RTC_CALIB_RDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_RTC_CALIB_VALUE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
    #[inline(always)]
    pub fn rtc_calib_value(&self) -> RTC_CALIB_VALUE_R {
        RTC_CALIB_VALUE_R::new((self.bits & 0x000f_ffff) as u32)
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
impl W {
    #[doc = "Bits 0:19 - The cycle number of clk_xtal (crystal clock) for the RTC_PERIOD_NUM cycles of RTC-clock"]
    #[inline(always)]
    pub fn rtc_calib_value(&mut self) -> RTC_CALIB_VALUE_W<0> {
        RTC_CALIB_VALUE_W::new(self)
    }
    #[doc = "Bit 30 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    pub fn rtc_calib_rdy_real(&mut self) -> RTC_CALIB_RDY_REAL_W<30> {
        RTC_CALIB_RDY_REAL_W::new(self)
    }
    #[doc = "Bit 31 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    pub fn rtc_calib_rdy(&mut self) -> RTC_CALIB_RDY_W<31> {
        RTC_CALIB_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "0: during RTC-clock-calibration; 1: RTC-clock-calibration is done\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_rtc_calib_value](index.html) module"]
pub struct GPIO_RTC_CALIB_VALUE_SPEC;
impl crate::RegisterSpec for GPIO_RTC_CALIB_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_rtc_calib_value::R](R) reader structure"]
impl crate::Readable for GPIO_RTC_CALIB_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_rtc_calib_value::W](W) writer structure"]
impl crate::Writable for GPIO_RTC_CALIB_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_RTC_CALIB_VALUE to value 0"]
impl crate::Resettable for GPIO_RTC_CALIB_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
