#[doc = "Register `THRES_CTRL` reader"]
pub type R = crate::R<THRES_CTRL_SPEC>;
#[doc = "Register `THRES_CTRL` writer"]
pub type W = crate::W<THRES_CTRL_SPEC>;
#[doc = "Field `CLK_EN` reader - Clock gate enable."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Clock gate enable."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_THRES_MODE` reader - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC2_THRES_MODE_R = crate::BitReader;
#[doc = "Field `ADC2_THRES_MODE` writer - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC2_THRES_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_THRES_MODE` reader - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC1_THRES_MODE_R = crate::BitReader;
#[doc = "Field `ADC1_THRES_MODE` writer - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC1_THRES_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_THRES` reader - ADC2 threshold."]
pub type ADC2_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `ADC2_THRES` writer - ADC2 threshold."]
pub type ADC2_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ADC1_THRES` reader - ADC1 threshold."]
pub type ADC1_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `ADC1_THRES` writer - ADC1 threshold."]
pub type ADC1_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ADC2_THRES_EN` reader - Enable ADC2 threshold monitor."]
pub type ADC2_THRES_EN_R = crate::BitReader;
#[doc = "Field `ADC2_THRES_EN` writer - Enable ADC2 threshold monitor."]
pub type ADC2_THRES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_THRES_EN` reader - Enable ADC1 threshold monitor."]
pub type ADC1_THRES_EN_R = crate::BitReader;
#[doc = "Field `ADC1_THRES_EN` writer - Enable ADC1 threshold monitor."]
pub type ADC1_THRES_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc2_thres_mode(&self) -> ADC2_THRES_MODE_R {
        ADC2_THRES_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc1_thres_mode(&self) -> ADC1_THRES_MODE_R {
        ADC1_THRES_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:16 - ADC2 threshold."]
    #[inline(always)]
    pub fn adc2_thres(&self) -> ADC2_THRES_R {
        ADC2_THRES_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
    #[doc = "Bits 17:29 - ADC1 threshold."]
    #[inline(always)]
    pub fn adc1_thres(&self) -> ADC1_THRES_R {
        ADC1_THRES_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - Enable ADC2 threshold monitor."]
    #[inline(always)]
    pub fn adc2_thres_en(&self) -> ADC2_THRES_EN_R {
        ADC2_THRES_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable ADC1 threshold monitor."]
    #[inline(always)]
    pub fn adc1_thres_en(&self) -> ADC1_THRES_EN_R {
        ADC1_THRES_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field("clk_en", &self.clk_en())
            .field("adc2_thres_mode", &self.adc2_thres_mode())
            .field("adc1_thres_mode", &self.adc1_thres_mode())
            .field("adc2_thres", &self.adc2_thres())
            .field("adc1_thres", &self.adc1_thres())
            .field("adc2_thres_en", &self.adc2_thres_en())
            .field("adc1_thres_en", &self.adc1_thres_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<THRES_CTRL_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc2_thres_mode(&mut self) -> ADC2_THRES_MODE_W<THRES_CTRL_SPEC> {
        ADC2_THRES_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc1_thres_mode(&mut self) -> ADC1_THRES_MODE_W<THRES_CTRL_SPEC> {
        ADC1_THRES_MODE_W::new(self, 3)
    }
    #[doc = "Bits 4:16 - ADC2 threshold."]
    #[inline(always)]
    pub fn adc2_thres(&mut self) -> ADC2_THRES_W<THRES_CTRL_SPEC> {
        ADC2_THRES_W::new(self, 4)
    }
    #[doc = "Bits 17:29 - ADC1 threshold."]
    #[inline(always)]
    pub fn adc1_thres(&mut self) -> ADC1_THRES_W<THRES_CTRL_SPEC> {
        ADC1_THRES_W::new(self, 17)
    }
    #[doc = "Bit 30 - Enable ADC2 threshold monitor."]
    #[inline(always)]
    pub fn adc2_thres_en(&mut self) -> ADC2_THRES_EN_W<THRES_CTRL_SPEC> {
        ADC2_THRES_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable ADC1 threshold monitor."]
    #[inline(always)]
    pub fn adc1_thres_en(&mut self) -> ADC1_THRES_EN_W<THRES_CTRL_SPEC> {
        ADC1_THRES_EN_W::new(self, 31)
    }
}
#[doc = "Configure monitor threshold for DIG ADC2\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {}
