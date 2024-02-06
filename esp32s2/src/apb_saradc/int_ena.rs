#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `ADC2_THRES_INT_ENA` reader - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC2_THRES_INT_ENA` writer - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_THRES_INT_ENA` reader - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC1_THRES_INT_ENA` writer - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_DONE_INT_ENA` reader - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC2_DONE_INT_ENA` writer - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_DONE_INT_ENA` reader - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ADC1_DONE_INT_ENA` writer - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres_int_ena(&self) -> ADC2_THRES_INT_ENA_R {
        ADC2_THRES_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres_int_ena(&self) -> ADC1_THRES_INT_ENA_R {
        ADC1_THRES_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done_int_ena(&self) -> ADC2_DONE_INT_ENA_R {
        ADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done_int_ena(&self) -> ADC1_DONE_INT_ENA_R {
        ADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "adc2_thres_int_ena",
                &format_args!("{}", self.adc2_thres_int_ena().bit()),
            )
            .field(
                "adc1_thres_int_ena",
                &format_args!("{}", self.adc1_thres_int_ena().bit()),
            )
            .field(
                "adc2_done_int_ena",
                &format_args!("{}", self.adc2_done_int_ena().bit()),
            )
            .field(
                "adc1_done_int_ena",
                &format_args!("{}", self.adc1_done_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 28 - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_thres_int_ena(&mut self) -> ADC2_THRES_INT_ENA_W<INT_ENA_SPEC> {
        ADC2_THRES_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_thres_int_ena(&mut self) -> ADC1_THRES_INT_ENA_W<INT_ENA_SPEC> {
        ADC1_THRES_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc2_done_int_ena(&mut self) -> ADC2_DONE_INT_ENA_W<INT_ENA_SPEC> {
        ADC2_DONE_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn adc1_done_int_ena(&mut self) -> ADC1_DONE_INT_ENA_W<INT_ENA_SPEC> {
        ADC1_DONE_INT_ENA_W::new(self, 31)
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
#[doc = "Enable DIG ADC interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
