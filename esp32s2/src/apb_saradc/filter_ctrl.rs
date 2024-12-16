#[doc = "Register `FILTER_CTRL` reader"]
pub type R = crate::R<FILTER_CTRL_SPEC>;
#[doc = "Register `FILTER_CTRL` writer"]
pub type W = crate::W<FILTER_CTRL_SPEC>;
#[doc = "Field `ADC2_FILTER_RESET` reader - Reset ADC2 filter."]
pub type ADC2_FILTER_RESET_R = crate::BitReader;
#[doc = "Field `ADC2_FILTER_RESET` writer - Reset ADC2 filter."]
pub type ADC2_FILTER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_FILTER_RESET` reader - Reset ADC1 filter."]
pub type ADC1_FILTER_RESET_R = crate::BitReader;
#[doc = "Field `ADC1_FILTER_RESET` writer - Reset ADC1 filter."]
pub type ADC1_FILTER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_FILTER_FACTOR` reader - Set filter factor for DIG ADC2 CRTL."]
pub type ADC2_FILTER_FACTOR_R = crate::FieldReader;
#[doc = "Field `ADC2_FILTER_FACTOR` writer - Set filter factor for DIG ADC2 CRTL."]
pub type ADC2_FILTER_FACTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADC1_FILTER_FACTOR` reader - Set filter factor for DIG ADC1 CRTL."]
pub type ADC1_FILTER_FACTOR_R = crate::FieldReader;
#[doc = "Field `ADC1_FILTER_FACTOR` writer - Set filter factor for DIG ADC1 CRTL."]
pub type ADC1_FILTER_FACTOR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADC2_FILTER_EN` reader - Enable DIG ADC2 CRTL filter."]
pub type ADC2_FILTER_EN_R = crate::BitReader;
#[doc = "Field `ADC2_FILTER_EN` writer - Enable DIG ADC2 CRTL filter."]
pub type ADC2_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_FILTER_EN` reader - Enable DIG ADC1 CRTL filter."]
pub type ADC1_FILTER_EN_R = crate::BitReader;
#[doc = "Field `ADC1_FILTER_EN` writer - Enable DIG ADC1 CRTL filter."]
pub type ADC1_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset ADC2 filter."]
    #[inline(always)]
    pub fn adc2_filter_reset(&self) -> ADC2_FILTER_RESET_R {
        ADC2_FILTER_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset ADC1 filter."]
    #[inline(always)]
    pub fn adc1_filter_reset(&self) -> ADC1_FILTER_RESET_R {
        ADC1_FILTER_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Set filter factor for DIG ADC2 CRTL."]
    #[inline(always)]
    pub fn adc2_filter_factor(&self) -> ADC2_FILTER_FACTOR_R {
        ADC2_FILTER_FACTOR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - Set filter factor for DIG ADC1 CRTL."]
    #[inline(always)]
    pub fn adc1_filter_factor(&self) -> ADC1_FILTER_FACTOR_R {
        ADC1_FILTER_FACTOR_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - Enable DIG ADC2 CRTL filter."]
    #[inline(always)]
    pub fn adc2_filter_en(&self) -> ADC2_FILTER_EN_R {
        ADC2_FILTER_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable DIG ADC1 CRTL filter."]
    #[inline(always)]
    pub fn adc1_filter_en(&self) -> ADC1_FILTER_EN_R {
        ADC1_FILTER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL")
            .field("adc2_filter_reset", &self.adc2_filter_reset())
            .field("adc1_filter_reset", &self.adc1_filter_reset())
            .field("adc2_filter_factor", &self.adc2_filter_factor())
            .field("adc1_filter_factor", &self.adc1_filter_factor())
            .field("adc2_filter_en", &self.adc2_filter_en())
            .field("adc1_filter_en", &self.adc1_filter_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reset ADC2 filter."]
    #[inline(always)]
    pub fn adc2_filter_reset(&mut self) -> ADC2_FILTER_RESET_W<FILTER_CTRL_SPEC> {
        ADC2_FILTER_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reset ADC1 filter."]
    #[inline(always)]
    pub fn adc1_filter_reset(&mut self) -> ADC1_FILTER_RESET_W<FILTER_CTRL_SPEC> {
        ADC1_FILTER_RESET_W::new(self, 1)
    }
    #[doc = "Bits 16:22 - Set filter factor for DIG ADC2 CRTL."]
    #[inline(always)]
    pub fn adc2_filter_factor(&mut self) -> ADC2_FILTER_FACTOR_W<FILTER_CTRL_SPEC> {
        ADC2_FILTER_FACTOR_W::new(self, 16)
    }
    #[doc = "Bits 23:29 - Set filter factor for DIG ADC1 CRTL."]
    #[inline(always)]
    pub fn adc1_filter_factor(&mut self) -> ADC1_FILTER_FACTOR_W<FILTER_CTRL_SPEC> {
        ADC1_FILTER_FACTOR_W::new(self, 23)
    }
    #[doc = "Bit 30 - Enable DIG ADC2 CRTL filter."]
    #[inline(always)]
    pub fn adc2_filter_en(&mut self) -> ADC2_FILTER_EN_W<FILTER_CTRL_SPEC> {
        ADC2_FILTER_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable DIG ADC1 CRTL filter."]
    #[inline(always)]
    pub fn adc1_filter_en(&mut self) -> ADC1_FILTER_EN_W<FILTER_CTRL_SPEC> {
        ADC1_FILTER_EN_W::new(self, 31)
    }
}
#[doc = "Configure the settings of DIG ADC2 filter\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CTRL_SPEC;
impl crate::RegisterSpec for FILTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_ctrl::R`](R) reader structure"]
impl crate::Readable for FILTER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_ctrl::W`](W) writer structure"]
impl crate::Writable for FILTER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER_CTRL to value 0x2040_0000"]
impl crate::Resettable for FILTER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2040_0000;
}
