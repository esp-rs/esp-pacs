#[doc = "Register `SAR1_HW_WAKEUP` reader"]
pub type R = crate::R<SAR1_HW_WAKEUP_SPEC>;
#[doc = "Register `SAR1_HW_WAKEUP` writer"]
pub type W = crate::W<SAR1_HW_WAKEUP_SPEC>;
#[doc = "Field `ADC1_HW_READ_EN_I` reader - Enable hardware automatic sampling."]
pub type ADC1_HW_READ_EN_I_R = crate::BitReader;
#[doc = "Field `ADC1_HW_READ_EN_I` writer - Enable hardware automatic sampling."]
pub type ADC1_HW_READ_EN_I_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_HW_READ_RATE_I` reader - Hardware automatic sampling rate."]
pub type ADC1_HW_READ_RATE_I_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_HW_READ_RATE_I` writer - Hardware automatic sampling rate."]
pub type ADC1_HW_READ_RATE_I_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable hardware automatic sampling."]
    #[inline(always)]
    pub fn adc1_hw_read_en_i(&self) -> ADC1_HW_READ_EN_I_R {
        ADC1_HW_READ_EN_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - Hardware automatic sampling rate."]
    #[inline(always)]
    pub fn adc1_hw_read_rate_i(&self) -> ADC1_HW_READ_RATE_I_R {
        ADC1_HW_READ_RATE_I_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_HW_WAKEUP")
            .field("adc1_hw_read_en_i", &self.adc1_hw_read_en_i())
            .field("adc1_hw_read_rate_i", &self.adc1_hw_read_rate_i())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable hardware automatic sampling."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_hw_read_en_i(&mut self) -> ADC1_HW_READ_EN_I_W<SAR1_HW_WAKEUP_SPEC> {
        ADC1_HW_READ_EN_I_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - Hardware automatic sampling rate."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_hw_read_rate_i(&mut self) -> ADC1_HW_READ_RATE_I_W<SAR1_HW_WAKEUP_SPEC> {
        ADC1_HW_READ_RATE_I_W::new(self, 1)
    }
}
#[doc = "Hardware automatic sampling registers for wakeup function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_hw_wakeup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_hw_wakeup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_HW_WAKEUP_SPEC;
impl crate::RegisterSpec for SAR1_HW_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_hw_wakeup::R`](R) reader structure"]
impl crate::Readable for SAR1_HW_WAKEUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar1_hw_wakeup::W`](W) writer structure"]
impl crate::Writable for SAR1_HW_WAKEUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR1_HW_WAKEUP to value 0xc8"]
impl crate::Resettable for SAR1_HW_WAKEUP_SPEC {
    const RESET_VALUE: u32 = 0xc8;
}
