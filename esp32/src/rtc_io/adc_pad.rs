#[doc = "Register `ADC_PAD` reader"]
pub type R = crate::R<ADC_PAD_SPEC>;
#[doc = "Register `ADC_PAD` writer"]
pub type W = crate::W<ADC_PAD_SPEC>;
#[doc = "Field `ADC2_FUN_IE` reader - the input enable of the pad"]
pub type ADC2_FUN_IE_R = crate::BitReader;
#[doc = "Field `ADC2_FUN_IE` writer - the input enable of the pad"]
pub type ADC2_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type ADC2_SLP_IE_R = crate::BitReader;
#[doc = "Field `ADC2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type ADC2_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type ADC2_SLP_SEL_R = crate::BitReader;
#[doc = "Field `ADC2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type ADC2_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_FUN_SEL` reader - the functional selection signal of the pad"]
pub type ADC2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `ADC2_FUN_SEL` writer - the functional selection signal of the pad"]
pub type ADC2_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC1_FUN_IE` reader - the input enable of the pad"]
pub type ADC1_FUN_IE_R = crate::BitReader;
#[doc = "Field `ADC1_FUN_IE` writer - the input enable of the pad"]
pub type ADC1_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type ADC1_SLP_IE_R = crate::BitReader;
#[doc = "Field `ADC1_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type ADC1_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type ADC1_SLP_SEL_R = crate::BitReader;
#[doc = "Field `ADC1_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type ADC1_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_FUN_SEL` reader - the functional selection signal of the pad"]
pub type ADC1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `ADC1_FUN_SEL` writer - the functional selection signal of the pad"]
pub type ADC1_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type ADC2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `ADC2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type ADC2_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type ADC1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `ADC1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type ADC1_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type ADC2_HOLD_R = crate::BitReader;
#[doc = "Field `ADC2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type ADC2_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type ADC1_HOLD_R = crate::BitReader;
#[doc = "Field `ADC1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type ADC1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc2_fun_ie(&self) -> ADC2_FUN_IE_R {
        ADC2_FUN_IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc2_slp_ie(&self) -> ADC2_SLP_IE_R {
        ADC2_SLP_IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_slp_sel(&self) -> ADC2_SLP_SEL_R {
        ADC2_SLP_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_fun_sel(&self) -> ADC2_FUN_SEL_R {
        ADC2_FUN_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc1_fun_ie(&self) -> ADC1_FUN_IE_R {
        ADC1_FUN_IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc1_slp_ie(&self) -> ADC1_SLP_IE_R {
        ADC1_SLP_IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_slp_sel(&self) -> ADC1_SLP_SEL_R {
        ADC1_SLP_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_fun_sel(&self) -> ADC1_FUN_SEL_R {
        ADC1_FUN_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc2_mux_sel(&self) -> ADC2_MUX_SEL_R {
        ADC2_MUX_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc1_mux_sel(&self) -> ADC1_MUX_SEL_R {
        ADC1_MUX_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc2_hold(&self) -> ADC2_HOLD_R {
        ADC2_HOLD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc1_hold(&self) -> ADC1_HOLD_R {
        ADC1_HOLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PAD")
            .field("adc2_fun_ie", &self.adc2_fun_ie())
            .field("adc2_slp_ie", &self.adc2_slp_ie())
            .field("adc2_slp_sel", &self.adc2_slp_sel())
            .field("adc2_fun_sel", &self.adc2_fun_sel())
            .field("adc1_fun_ie", &self.adc1_fun_ie())
            .field("adc1_slp_ie", &self.adc1_slp_ie())
            .field("adc1_slp_sel", &self.adc1_slp_sel())
            .field("adc1_fun_sel", &self.adc1_fun_sel())
            .field("adc2_mux_sel", &self.adc2_mux_sel())
            .field("adc1_mux_sel", &self.adc1_mux_sel())
            .field("adc2_hold", &self.adc2_hold())
            .field("adc1_hold", &self.adc1_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 18 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc2_fun_ie(&mut self) -> ADC2_FUN_IE_W<ADC_PAD_SPEC> {
        ADC2_FUN_IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc2_slp_ie(&mut self) -> ADC2_SLP_IE_W<ADC_PAD_SPEC> {
        ADC2_SLP_IE_W::new(self, 19)
    }
    #[doc = "Bit 20 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_slp_sel(&mut self) -> ADC2_SLP_SEL_W<ADC_PAD_SPEC> {
        ADC2_SLP_SEL_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_fun_sel(&mut self) -> ADC2_FUN_SEL_W<ADC_PAD_SPEC> {
        ADC2_FUN_SEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc1_fun_ie(&mut self) -> ADC1_FUN_IE_W<ADC_PAD_SPEC> {
        ADC1_FUN_IE_W::new(self, 23)
    }
    #[doc = "Bit 24 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc1_slp_ie(&mut self) -> ADC1_SLP_IE_W<ADC_PAD_SPEC> {
        ADC1_SLP_IE_W::new(self, 24)
    }
    #[doc = "Bit 25 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_slp_sel(&mut self) -> ADC1_SLP_SEL_W<ADC_PAD_SPEC> {
        ADC1_SLP_SEL_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_fun_sel(&mut self) -> ADC1_FUN_SEL_W<ADC_PAD_SPEC> {
        ADC1_FUN_SEL_W::new(self, 26)
    }
    #[doc = "Bit 28 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc2_mux_sel(&mut self) -> ADC2_MUX_SEL_W<ADC_PAD_SPEC> {
        ADC2_MUX_SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc1_mux_sel(&mut self) -> ADC1_MUX_SEL_W<ADC_PAD_SPEC> {
        ADC1_MUX_SEL_W::new(self, 29)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc2_hold(&mut self) -> ADC2_HOLD_W<ADC_PAD_SPEC> {
        ADC2_HOLD_W::new(self, 30)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc1_hold(&mut self) -> ADC1_HOLD_W<ADC_PAD_SPEC> {
        ADC1_HOLD_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_pad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_pad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_PAD_SPEC;
impl crate::RegisterSpec for ADC_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_pad::R`](R) reader structure"]
impl crate::Readable for ADC_PAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_pad::W`](W) writer structure"]
impl crate::Writable for ADC_PAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_PAD to value 0"]
impl crate::Resettable for ADC_PAD_SPEC {}
