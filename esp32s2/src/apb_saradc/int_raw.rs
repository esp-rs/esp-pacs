///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Field `ADC2_THRES` reader - Raw bit of APB_SARADC_ADC2_THRES_INT interrupt.
pub type ADC2_THRES_R = crate::BitReader;
///Field `ADC1_THRES` reader - Raw bit of APB_SARADC_ADC1_THRES_INT interrupt.
pub type ADC1_THRES_R = crate::BitReader;
///Field `ADC2_DONE` reader - Raw bit of APB_SARADC_ADC2_DONE_INT interrupt.
pub type ADC2_DONE_R = crate::BitReader;
///Field `ADC1_DONE` reader - Raw bit of APB_SARADC_ADC1_DONE_INT interrupt.
pub type ADC1_DONE_R = crate::BitReader;
impl R {
    ///Bit 28 - Raw bit of APB_SARADC_ADC2_THRES_INT interrupt.
    #[inline(always)]
    pub fn adc2_thres(&self) -> ADC2_THRES_R {
        ADC2_THRES_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Raw bit of APB_SARADC_ADC1_THRES_INT interrupt.
    #[inline(always)]
    pub fn adc1_thres(&self) -> ADC1_THRES_R {
        ADC1_THRES_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Raw bit of APB_SARADC_ADC2_DONE_INT interrupt.
    #[inline(always)]
    pub fn adc2_done(&self) -> ADC2_DONE_R {
        ADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Raw bit of APB_SARADC_ADC1_DONE_INT interrupt.
    #[inline(always)]
    pub fn adc1_done(&self) -> ADC1_DONE_R {
        ADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("adc2_thres", &self.adc2_thres())
            .field("adc1_thres", &self.adc1_thres())
            .field("adc2_done", &self.adc2_done())
            .field("adc1_done", &self.adc1_done())
            .finish()
    }
}
/**DIG ADC interrupt raw bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
