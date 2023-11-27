#[doc = "Register `SAR2_HW_WAKEUP` reader"]
pub type R = crate::R<SAR2_HW_WAKEUP_SPEC>;
#[doc = "Register `SAR2_HW_WAKEUP` writer"]
pub type W = crate::W<SAR2_HW_WAKEUP_SPEC>;
#[doc = "Field `ADC2_HW_READ_EN_I` reader - Enable hardware automatic sampling."]
pub type ADC2_HW_READ_EN_I_R = crate::BitReader;
#[doc = "Field `ADC2_HW_READ_EN_I` writer - Enable hardware automatic sampling."]
pub type ADC2_HW_READ_EN_I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_HW_READ_RATE_I` reader - Hardware automatic sampling rate."]
pub type ADC2_HW_READ_RATE_I_R = crate::FieldReader<u16>;
#[doc = "Field `ADC2_HW_READ_RATE_I` writer - Hardware automatic sampling rate."]
pub type ADC2_HW_READ_RATE_I_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable hardware automatic sampling."]
    #[inline(always)]
    pub fn adc2_hw_read_en_i(&self) -> ADC2_HW_READ_EN_I_R {
        ADC2_HW_READ_EN_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - Hardware automatic sampling rate."]
    #[inline(always)]
    pub fn adc2_hw_read_rate_i(&self) -> ADC2_HW_READ_RATE_I_R {
        ADC2_HW_READ_RATE_I_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_HW_WAKEUP")
            .field(
                "adc2_hw_read_en_i",
                &format_args!("{}", self.adc2_hw_read_en_i().bit()),
            )
            .field(
                "adc2_hw_read_rate_i",
                &format_args!("{}", self.adc2_hw_read_rate_i().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR2_HW_WAKEUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hardware automatic sampling."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_hw_read_en_i(&mut self) -> ADC2_HW_READ_EN_I_W<SAR2_HW_WAKEUP_SPEC> {
        ADC2_HW_READ_EN_I_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - Hardware automatic sampling rate."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_hw_read_rate_i(&mut self) -> ADC2_HW_READ_RATE_I_W<SAR2_HW_WAKEUP_SPEC> {
        ADC2_HW_READ_RATE_I_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hardware automatic sampling registers for wakeup function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_hw_wakeup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_hw_wakeup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2_HW_WAKEUP_SPEC;
impl crate::RegisterSpec for SAR2_HW_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_hw_wakeup::R`](R) reader structure"]
impl crate::Readable for SAR2_HW_WAKEUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar2_hw_wakeup::W`](W) writer structure"]
impl crate::Writable for SAR2_HW_WAKEUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR2_HW_WAKEUP to value 0xc8"]
impl crate::Resettable for SAR2_HW_WAKEUP_SPEC {
    const RESET_VALUE: Self::Ux = 0xc8;
}
