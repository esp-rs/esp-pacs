#[doc = "Register `PERI_CLK_CTRL24` reader"]
pub type R = crate::R<PERI_CLK_CTRL24_SPEC>;
#[doc = "Register `PERI_CLK_CTRL24` writer"]
pub type W = crate::W<PERI_CLK_CTRL24_SPEC>;
#[doc = "Field `REG_ADC_SAR1_CLK_DIV_NUM` reader - Reserved"]
pub type REG_ADC_SAR1_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_ADC_SAR1_CLK_DIV_NUM` writer - Reserved"]
pub type REG_ADC_SAR1_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_ADC_SAR2_CLK_DIV_NUM` reader - Reserved"]
pub type REG_ADC_SAR2_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_ADC_SAR2_CLK_DIV_NUM` writer - Reserved"]
pub type REG_ADC_SAR2_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_PVT_CLK_DIV_NUM` reader - Reserved"]
pub type REG_PVT_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_PVT_CLK_DIV_NUM` writer - Reserved"]
pub type REG_PVT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_PVT_CLK_EN` reader - Reserved"]
pub type REG_PVT_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_PVT_CLK_EN` writer - Reserved"]
pub type REG_PVT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_adc_sar1_clk_div_num(&self) -> REG_ADC_SAR1_CLK_DIV_NUM_R {
        REG_ADC_SAR1_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reg_adc_sar2_clk_div_num(&self) -> REG_ADC_SAR2_CLK_DIV_NUM_R {
        REG_ADC_SAR2_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reg_pvt_clk_div_num(&self) -> REG_PVT_CLK_DIV_NUM_R {
        REG_PVT_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reg_pvt_clk_en(&self) -> REG_PVT_CLK_EN_R {
        REG_PVT_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL24")
            .field(
                "reg_adc_sar1_clk_div_num",
                &format_args!("{}", self.reg_adc_sar1_clk_div_num().bits()),
            )
            .field(
                "reg_adc_sar2_clk_div_num",
                &format_args!("{}", self.reg_adc_sar2_clk_div_num().bits()),
            )
            .field(
                "reg_pvt_clk_div_num",
                &format_args!("{}", self.reg_pvt_clk_div_num().bits()),
            )
            .field(
                "reg_pvt_clk_en",
                &format_args!("{}", self.reg_pvt_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL24_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_adc_sar1_clk_div_num(&mut self) -> REG_ADC_SAR1_CLK_DIV_NUM_W<PERI_CLK_CTRL24_SPEC> {
        REG_ADC_SAR1_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_adc_sar2_clk_div_num(&mut self) -> REG_ADC_SAR2_CLK_DIV_NUM_W<PERI_CLK_CTRL24_SPEC> {
        REG_ADC_SAR2_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pvt_clk_div_num(&mut self) -> REG_PVT_CLK_DIV_NUM_W<PERI_CLK_CTRL24_SPEC> {
        REG_PVT_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pvt_clk_en(&mut self) -> REG_PVT_CLK_EN_W<PERI_CLK_CTRL24_SPEC> {
        REG_PVT_CLK_EN_W::new(self, 24)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL24_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl24::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl24::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL24_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL24 to value 0x0404"]
impl crate::Resettable for PERI_CLK_CTRL24_SPEC {
    const RESET_VALUE: u32 = 0x0404;
}
