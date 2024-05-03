#[doc = "Register `WAKEUP1` reader"]
pub type R = crate::R<WAKEUP1_SPEC>;
#[doc = "Register `WAKEUP1` writer"]
pub type W = crate::W<WAKEUP1_SPEC>;
#[doc = "Field `SAR1_WAKEUP_TH_LOW` reader - Lower threshold."]
pub type SAR1_WAKEUP_TH_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `SAR1_WAKEUP_TH_LOW` writer - Lower threshold."]
pub type SAR1_WAKEUP_TH_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR1_WAKEUP_TH_HIGH` reader - Upper threshold."]
pub type SAR1_WAKEUP_TH_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `SAR1_WAKEUP_TH_HIGH` writer - Upper threshold."]
pub type SAR1_WAKEUP_TH_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR1_WAKEUP_OVER_UPPER_TH` reader - Indicates that this wakeup event arose from exceeding upper threshold."]
pub type SAR1_WAKEUP_OVER_UPPER_TH_R = crate::BitReader;
#[doc = "Field `SAR1_WAKEUP_EN` reader - Wakeup function enable."]
pub type SAR1_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `SAR1_WAKEUP_EN` writer - Wakeup function enable."]
pub type SAR1_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_WAKEUP_MODE` reader - 0:absolute value comparison mode. 1: relative value comparison mode."]
pub type SAR1_WAKEUP_MODE_R = crate::BitReader;
#[doc = "Field `SAR1_WAKEUP_MODE` writer - 0:absolute value comparison mode. 1: relative value comparison mode."]
pub type SAR1_WAKEUP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Lower threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_th_low(&self) -> SAR1_WAKEUP_TH_LOW_R {
        SAR1_WAKEUP_TH_LOW_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 14:25 - Upper threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_th_high(&self) -> SAR1_WAKEUP_TH_HIGH_R {
        SAR1_WAKEUP_TH_HIGH_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bit 29 - Indicates that this wakeup event arose from exceeding upper threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_over_upper_th(&self) -> SAR1_WAKEUP_OVER_UPPER_TH_R {
        SAR1_WAKEUP_OVER_UPPER_TH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wakeup function enable."]
    #[inline(always)]
    pub fn sar1_wakeup_en(&self) -> SAR1_WAKEUP_EN_R {
        SAR1_WAKEUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0:absolute value comparison mode. 1: relative value comparison mode."]
    #[inline(always)]
    pub fn sar1_wakeup_mode(&self) -> SAR1_WAKEUP_MODE_R {
        SAR1_WAKEUP_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP1")
            .field("sar1_wakeup_th_low", &self.sar1_wakeup_th_low().bits())
            .field("sar1_wakeup_th_high", &self.sar1_wakeup_th_high().bits())
            .field(
                "sar1_wakeup_over_upper_th",
                &self.sar1_wakeup_over_upper_th().bit(),
            )
            .field("sar1_wakeup_en", &self.sar1_wakeup_en().bit())
            .field("sar1_wakeup_mode", &self.sar1_wakeup_mode().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WAKEUP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Lower threshold."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_wakeup_th_low(&mut self) -> SAR1_WAKEUP_TH_LOW_W<WAKEUP1_SPEC> {
        SAR1_WAKEUP_TH_LOW_W::new(self, 0)
    }
    #[doc = "Bits 14:25 - Upper threshold."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_wakeup_th_high(&mut self) -> SAR1_WAKEUP_TH_HIGH_W<WAKEUP1_SPEC> {
        SAR1_WAKEUP_TH_HIGH_W::new(self, 14)
    }
    #[doc = "Bit 30 - Wakeup function enable."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_wakeup_en(&mut self) -> SAR1_WAKEUP_EN_W<WAKEUP1_SPEC> {
        SAR1_WAKEUP_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0:absolute value comparison mode. 1: relative value comparison mode."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_wakeup_mode(&mut self) -> SAR1_WAKEUP_MODE_W<WAKEUP1_SPEC> {
        SAR1_WAKEUP_MODE_W::new(self, 31)
    }
}
#[doc = "ADC1 wakeup configuration registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP1_SPEC;
impl crate::RegisterSpec for WAKEUP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup1::R`](R) reader structure"]
impl crate::Readable for WAKEUP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup1::W`](W) writer structure"]
impl crate::Writable for WAKEUP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP1 to value 0x03ff_c000"]
impl crate::Resettable for WAKEUP1_SPEC {
    const RESET_VALUE: u32 = 0x03ff_c000;
}
