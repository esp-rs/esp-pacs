#[doc = "Register `SAR_MEAS2_MUX` reader"]
pub type R = crate::R<SAR_MEAS2_MUX_SPEC>;
#[doc = "Register `SAR_MEAS2_MUX` writer"]
pub type W = crate::W<SAR_MEAS2_MUX_SPEC>;
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAR2_RTC_FORCE` reader - In sleep, force to use RTC to control ADC."]
pub type SAR2_RTC_FORCE_R = crate::BitReader;
#[doc = "Field `SAR2_RTC_FORCE` writer - In sleep, force to use RTC to control ADC."]
pub type SAR2_RTC_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - In sleep, force to use RTC to control ADC."]
    #[inline(always)]
    pub fn sar2_rtc_force(&self) -> SAR2_RTC_FORCE_R {
        SAR2_RTC_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS2_MUX")
            .field(
                "sar2_pwdet_cct",
                &format_args!("{}", self.sar2_pwdet_cct().bits()),
            )
            .field(
                "sar2_rtc_force",
                &format_args!("{}", self.sar2_rtc_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS2_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT, PA power detector capacitance tuning."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<SAR_MEAS2_MUX_SPEC> {
        SAR2_PWDET_CCT_W::new(self, 28)
    }
    #[doc = "Bit 31 - In sleep, force to use RTC to control ADC."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_rtc_force(&mut self) -> SAR2_RTC_FORCE_W<SAR_MEAS2_MUX_SPEC> {
        SAR2_RTC_FORCE_W::new(self, 31)
    }
}
#[doc = "Select the controller for SAR ADC2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_mux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS2_MUX_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas2_mux::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS2_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas2_mux::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS2_MUX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_MEAS2_MUX to value 0"]
impl crate::Resettable for SAR_MEAS2_MUX_SPEC {
    const RESET_VALUE: u32 = 0;
}
