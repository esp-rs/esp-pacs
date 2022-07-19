#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
pub type TIMER0_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
pub type TIMER0_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 0>;
#[doc = "Field `TIMER1_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
pub type TIMER1_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
pub type TIMER1_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 1>;
#[doc = "Field `TIMER2_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
pub type TIMER2_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
pub type TIMER2_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 2>;
#[doc = "Field `TIMER3_OVF_INT_ENA` reader - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
pub type TIMER3_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMER3_OVF_INT_ENA` writer - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
pub type TIMER3_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 3>;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub type DUTY_CHNG_END_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_CH0_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
pub type DUTY_CHNG_END_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 4>;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub type DUTY_CHNG_END_CH1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_CH1_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
pub type DUTY_CHNG_END_CH1_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 5>;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub type DUTY_CHNG_END_CH2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_CH2_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
pub type DUTY_CHNG_END_CH2_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 6>;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub type DUTY_CHNG_END_CH3_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_CH3_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
pub type DUTY_CHNG_END_CH3_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 7>;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub type DUTY_CHNG_END_CH4_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_CH4_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
pub type DUTY_CHNG_END_CH4_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 8>;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_ENA` reader - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub type DUTY_CHNG_END_CH5_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_CHNG_END_CH5_INT_ENA` writer - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
pub type DUTY_CHNG_END_CH5_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 9>;
#[doc = "Field `OVF_CNT_CH0_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
pub type OVF_CNT_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_CH0_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
pub type OVF_CNT_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 10>;
#[doc = "Field `OVF_CNT_CH1_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
pub type OVF_CNT_CH1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_CH1_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
pub type OVF_CNT_CH1_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 11>;
#[doc = "Field `OVF_CNT_CH2_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
pub type OVF_CNT_CH2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_CH2_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
pub type OVF_CNT_CH2_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 12>;
#[doc = "Field `OVF_CNT_CH3_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
pub type OVF_CNT_CH3_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_CH3_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
pub type OVF_CNT_CH3_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 13>;
#[doc = "Field `OVF_CNT_CH4_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
pub type OVF_CNT_CH4_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_CH4_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
pub type OVF_CNT_CH4_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 14>;
#[doc = "Field `OVF_CNT_CH5_INT_ENA` reader - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
pub type OVF_CNT_CH5_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OVF_CNT_CH5_INT_ENA` writer - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
pub type OVF_CNT_CH5_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer0_ovf_int_ena(&self) -> TIMER0_OVF_INT_ENA_R {
        TIMER0_OVF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer1_ovf_int_ena(&self) -> TIMER1_OVF_INT_ENA_R {
        TIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer2_ovf_int_ena(&self) -> TIMER2_OVF_INT_ENA_R {
        TIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer3_ovf_int_ena(&self) -> TIMER3_OVF_INT_ENA_R {
        TIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_ena(&self) -> DUTY_CHNG_END_CH0_INT_ENA_R {
        DUTY_CHNG_END_CH0_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_ena(&self) -> DUTY_CHNG_END_CH1_INT_ENA_R {
        DUTY_CHNG_END_CH1_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_ena(&self) -> DUTY_CHNG_END_CH2_INT_ENA_R {
        DUTY_CHNG_END_CH2_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_ena(&self) -> DUTY_CHNG_END_CH3_INT_ENA_R {
        DUTY_CHNG_END_CH3_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_ena(&self) -> DUTY_CHNG_END_CH4_INT_ENA_R {
        DUTY_CHNG_END_CH4_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_ena(&self) -> DUTY_CHNG_END_CH5_INT_ENA_R {
        DUTY_CHNG_END_CH5_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_ena(&self) -> OVF_CNT_CH0_INT_ENA_R {
        OVF_CNT_CH0_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_ena(&self) -> OVF_CNT_CH1_INT_ENA_R {
        OVF_CNT_CH1_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_ena(&self) -> OVF_CNT_CH2_INT_ENA_R {
        OVF_CNT_CH2_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_ena(&self) -> OVF_CNT_CH3_INT_ENA_R {
        OVF_CNT_CH3_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_ena(&self) -> OVF_CNT_CH4_INT_ENA_R {
        OVF_CNT_CH4_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_ena(&self) -> OVF_CNT_CH5_INT_ENA_R {
        OVF_CNT_CH5_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the LEDC_TIMER0_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer0_ovf_int_ena(&mut self) -> TIMER0_OVF_INT_ENA_W {
        TIMER0_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the LEDC_TIMER1_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer1_ovf_int_ena(&mut self) -> TIMER1_OVF_INT_ENA_W {
        TIMER1_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the LEDC_TIMER2_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer2_ovf_int_ena(&mut self) -> TIMER2_OVF_INT_ENA_W {
        TIMER2_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the LEDC_TIMER3_OVF_INT interrupt."]
    #[inline(always)]
    pub fn timer3_ovf_int_ena(&mut self) -> TIMER3_OVF_INT_ENA_W {
        TIMER3_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH0_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0_int_ena(&mut self) -> DUTY_CHNG_END_CH0_INT_ENA_W {
        DUTY_CHNG_END_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH1_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1_int_ena(&mut self) -> DUTY_CHNG_END_CH1_INT_ENA_W {
        DUTY_CHNG_END_CH1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH2_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2_int_ena(&mut self) -> DUTY_CHNG_END_CH2_INT_ENA_W {
        DUTY_CHNG_END_CH2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH3_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3_int_ena(&mut self) -> DUTY_CHNG_END_CH3_INT_ENA_W {
        DUTY_CHNG_END_CH3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH4_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4_int_ena(&mut self) -> DUTY_CHNG_END_CH4_INT_ENA_W {
        DUTY_CHNG_END_CH4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the LEDC_DUTY_CHNG_END_CH5_INT interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5_int_ena(&mut self) -> DUTY_CHNG_END_CH5_INT_ENA_W {
        DUTY_CHNG_END_CH5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the LEDC_OVF_CNT_CH0_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0_int_ena(&mut self) -> OVF_CNT_CH0_INT_ENA_W {
        OVF_CNT_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the LEDC_OVF_CNT_CH1_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1_int_ena(&mut self) -> OVF_CNT_CH1_INT_ENA_W {
        OVF_CNT_CH1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the LEDC_OVF_CNT_CH2_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2_int_ena(&mut self) -> OVF_CNT_CH2_INT_ENA_W {
        OVF_CNT_CH2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the LEDC_OVF_CNT_CH3_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3_int_ena(&mut self) -> OVF_CNT_CH3_INT_ENA_W {
        OVF_CNT_CH3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the LEDC_OVF_CNT_CH4_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4_int_ena(&mut self) -> OVF_CNT_CH4_INT_ENA_W {
        OVF_CNT_CH4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the LEDC_OVF_CNT_CH5_INT interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5_int_ena(&mut self) -> OVF_CNT_CH5_INT_ENA_W {
        OVF_CNT_CH5_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
