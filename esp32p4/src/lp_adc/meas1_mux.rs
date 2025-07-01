#[doc = "Register `MEAS1_MUX` reader"]
pub type R = crate::R<MEAS1_MUX_SPEC>;
#[doc = "Register `MEAS1_MUX` writer"]
pub type W = crate::W<MEAS1_MUX_SPEC>;
#[doc = "Field `SAR1_DIG_FORCE` reader - 1: SAR ADC1 controlled by DIG ADC1 CTRL."]
pub type SAR1_DIG_FORCE_R = crate::BitReader;
#[doc = "Field `SAR1_DIG_FORCE` writer - 1: SAR ADC1 controlled by DIG ADC1 CTRL."]
pub type SAR1_DIG_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - 1: SAR ADC1 controlled by DIG ADC1 CTRL."]
    #[inline(always)]
    pub fn sar1_dig_force(&self) -> SAR1_DIG_FORCE_R {
        SAR1_DIG_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEAS1_MUX")
            .field("sar1_dig_force", &self.sar1_dig_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - 1: SAR ADC1 controlled by DIG ADC1 CTRL."]
    #[inline(always)]
    pub fn sar1_dig_force(&mut self) -> SAR1_DIG_FORCE_W<MEAS1_MUX_SPEC> {
        SAR1_DIG_FORCE_W::new(self, 31)
    }
}
#[doc = "SAR ADC1 MUX register.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas1_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas1_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEAS1_MUX_SPEC;
impl crate::RegisterSpec for MEAS1_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meas1_mux::R`](R) reader structure"]
impl crate::Readable for MEAS1_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meas1_mux::W`](W) writer structure"]
impl crate::Writable for MEAS1_MUX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEAS1_MUX to value 0"]
impl crate::Resettable for MEAS1_MUX_SPEC {}
