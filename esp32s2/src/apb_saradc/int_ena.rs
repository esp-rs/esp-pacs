#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `ADC2_THRES` reader - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_R = crate::BitReader;
#[doc = "Field `ADC2_THRES` writer - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
pub type ADC2_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_THRES` reader - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_R = crate::BitReader;
#[doc = "Field `ADC1_THRES` writer - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
pub type ADC1_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_DONE` reader - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_R = crate::BitReader;
#[doc = "Field `ADC2_DONE` writer - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
pub type ADC2_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_DONE` reader - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_R = crate::BitReader;
#[doc = "Field `ADC1_DONE` writer - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
pub type ADC1_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres(&self) -> ADC2_THRES_R {
        ADC2_THRES_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres(&self) -> ADC1_THRES_R {
        ADC1_THRES_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done(&self) -> ADC2_DONE_R {
        ADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done(&self) -> ADC1_DONE_R {
        ADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("adc2_thres", &self.adc2_thres())
            .field("adc1_thres", &self.adc1_thres())
            .field("adc2_done", &self.adc2_done())
            .field("adc1_done", &self.adc1_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 28 - Enable bit of APB_SARADC_ADC2_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc2_thres(&mut self) -> ADC2_THRES_W<INT_ENA_SPEC> {
        ADC2_THRES_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable bit of APB_SARADC_ADC1_THRES_INT interrupt."]
    #[inline(always)]
    pub fn adc1_thres(&mut self) -> ADC1_THRES_W<INT_ENA_SPEC> {
        ADC1_THRES_W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable bit of APB_SARADC_ADC2_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc2_done(&mut self) -> ADC2_DONE_W<INT_ENA_SPEC> {
        ADC2_DONE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable bit of APB_SARADC_ADC1_DONE_INT interrupt."]
    #[inline(always)]
    pub fn adc1_done(&mut self) -> ADC1_DONE_W<INT_ENA_SPEC> {
        ADC1_DONE_W::new(self, 31)
    }
}
#[doc = "Enable DIG ADC interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
