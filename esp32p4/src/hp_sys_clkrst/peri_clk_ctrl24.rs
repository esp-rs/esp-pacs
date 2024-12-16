#[doc = "Register `PERI_CLK_CTRL24` reader"]
pub type R = crate::R<PERI_CLK_CTRL24_SPEC>;
#[doc = "Register `PERI_CLK_CTRL24` writer"]
pub type W = crate::W<PERI_CLK_CTRL24_SPEC>;
#[doc = "Field `ADC_SAR1_CLK_DIV_NUM` reader - Reserved"]
pub type ADC_SAR1_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `ADC_SAR1_CLK_DIV_NUM` writer - Reserved"]
pub type ADC_SAR1_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_SAR2_CLK_DIV_NUM` reader - Reserved"]
pub type ADC_SAR2_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `ADC_SAR2_CLK_DIV_NUM` writer - Reserved"]
pub type ADC_SAR2_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVT_CLK_DIV_NUM` reader - Reserved"]
pub type PVT_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PVT_CLK_DIV_NUM` writer - Reserved"]
pub type PVT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVT_CLK_EN` reader - Reserved"]
pub type PVT_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_CLK_EN` writer - Reserved"]
pub type PVT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn adc_sar1_clk_div_num(&self) -> ADC_SAR1_CLK_DIV_NUM_R {
        ADC_SAR1_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn adc_sar2_clk_div_num(&self) -> ADC_SAR2_CLK_DIV_NUM_R {
        ADC_SAR2_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_div_num(&self) -> PVT_CLK_DIV_NUM_R {
        PVT_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_en(&self) -> PVT_CLK_EN_R {
        PVT_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL24")
            .field("adc_sar1_clk_div_num", &self.adc_sar1_clk_div_num())
            .field("adc_sar2_clk_div_num", &self.adc_sar2_clk_div_num())
            .field("pvt_clk_div_num", &self.pvt_clk_div_num())
            .field("pvt_clk_en", &self.pvt_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn adc_sar1_clk_div_num(&mut self) -> ADC_SAR1_CLK_DIV_NUM_W<PERI_CLK_CTRL24_SPEC> {
        ADC_SAR1_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn adc_sar2_clk_div_num(&mut self) -> ADC_SAR2_CLK_DIV_NUM_W<PERI_CLK_CTRL24_SPEC> {
        ADC_SAR2_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_div_num(&mut self) -> PVT_CLK_DIV_NUM_W<PERI_CLK_CTRL24_SPEC> {
        PVT_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_en(&mut self) -> PVT_CLK_EN_W<PERI_CLK_CTRL24_SPEC> {
        PVT_CLK_EN_W::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL24_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl24::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl24::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL24_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL24 to value 0x0404"]
impl crate::Resettable for PERI_CLK_CTRL24_SPEC {
    const RESET_VALUE: u32 = 0x0404;
}
