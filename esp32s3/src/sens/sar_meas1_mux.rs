#[doc = "Register `SAR_MEAS1_MUX` reader"]
pub type R = crate::R<SAR_MEAS1_MUX_SPEC>;
#[doc = "Register `SAR_MEAS1_MUX` writer"]
pub type W = crate::W<SAR_MEAS1_MUX_SPEC>;
#[doc = "Field `SAR1_DIG_FORCE` reader - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
pub type SAR1_DIG_FORCE_R = crate::BitReader;
#[doc = "Field `SAR1_DIG_FORCE` writer - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
pub type SAR1_DIG_FORCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_dig_force(&self) -> SAR1_DIG_FORCE_R {
        SAR1_DIG_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS1_MUX")
            .field(
                "sar1_dig_force",
                &format_args!("{}", self.sar1_dig_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS1_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - 1: SAR ADC1 controlled by DIG ADC1 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_dig_force(&mut self) -> SAR1_DIG_FORCE_W<SAR_MEAS1_MUX_SPEC, 31> {
        SAR1_DIG_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure saradc1 controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas1_mux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas1_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS1_MUX_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas1_mux::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS1_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas1_mux::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS1_MUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS1_MUX to value 0"]
impl crate::Resettable for SAR_MEAS1_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
