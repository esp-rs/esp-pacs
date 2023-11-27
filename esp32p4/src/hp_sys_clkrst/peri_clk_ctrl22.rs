#[doc = "Register `PERI_CLK_CTRL22` reader"]
pub type R = crate::R<PERI_CLK_CTRL22_SPEC>;
#[doc = "Register `PERI_CLK_CTRL22` writer"]
pub type W = crate::W<PERI_CLK_CTRL22_SPEC>;
#[doc = "Field `REG_LEDC_CLK_SRC_SEL` reader - Reserved"]
pub type REG_LEDC_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_LEDC_CLK_SRC_SEL` writer - Reserved"]
pub type REG_LEDC_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_LEDC_CLK_EN` reader - Reserved"]
pub type REG_LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_LEDC_CLK_EN` writer - Reserved"]
pub type REG_LEDC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RMT_CLK_SRC_SEL` reader - Reserved"]
pub type REG_RMT_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_RMT_CLK_SRC_SEL` writer - Reserved"]
pub type REG_RMT_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_RMT_CLK_EN` reader - Reserved"]
pub type REG_RMT_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_RMT_CLK_EN` writer - Reserved"]
pub type REG_RMT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RMT_CLK_DIV_NUM` reader - Reserved"]
pub type REG_RMT_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_RMT_CLK_DIV_NUM` writer - Reserved"]
pub type REG_RMT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_RMT_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type REG_RMT_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `REG_RMT_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type REG_RMT_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_RMT_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type REG_RMT_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `REG_RMT_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type REG_RMT_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_ADC_CLK_SRC_SEL` reader - Reserved"]
pub type REG_ADC_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_ADC_CLK_SRC_SEL` writer - Reserved"]
pub type REG_ADC_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_ledc_clk_src_sel(&self) -> REG_LEDC_CLK_SRC_SEL_R {
        REG_LEDC_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_ledc_clk_en(&self) -> REG_LEDC_CLK_EN_R {
        REG_LEDC_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline(always)]
    pub fn reg_rmt_clk_src_sel(&self) -> REG_RMT_CLK_SRC_SEL_R {
        REG_RMT_CLK_SRC_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reg_rmt_clk_en(&self) -> REG_RMT_CLK_EN_R {
        REG_RMT_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - Reserved"]
    #[inline(always)]
    pub fn reg_rmt_clk_div_num(&self) -> REG_RMT_CLK_DIV_NUM_R {
        REG_RMT_CLK_DIV_NUM_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn reg_rmt_clk_div_numerator(&self) -> REG_RMT_CLK_DIV_NUMERATOR_R {
        REG_RMT_CLK_DIV_NUMERATOR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - Reserved"]
    #[inline(always)]
    pub fn reg_rmt_clk_div_denominator(&self) -> REG_RMT_CLK_DIV_DENOMINATOR_R {
        REG_RMT_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn reg_adc_clk_src_sel(&self) -> REG_ADC_CLK_SRC_SEL_R {
        REG_ADC_CLK_SRC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL22")
            .field(
                "reg_ledc_clk_src_sel",
                &format_args!("{}", self.reg_ledc_clk_src_sel().bits()),
            )
            .field(
                "reg_ledc_clk_en",
                &format_args!("{}", self.reg_ledc_clk_en().bit()),
            )
            .field(
                "reg_rmt_clk_src_sel",
                &format_args!("{}", self.reg_rmt_clk_src_sel().bits()),
            )
            .field(
                "reg_rmt_clk_en",
                &format_args!("{}", self.reg_rmt_clk_en().bit()),
            )
            .field(
                "reg_rmt_clk_div_num",
                &format_args!("{}", self.reg_rmt_clk_div_num().bits()),
            )
            .field(
                "reg_rmt_clk_div_numerator",
                &format_args!("{}", self.reg_rmt_clk_div_numerator().bits()),
            )
            .field(
                "reg_rmt_clk_div_denominator",
                &format_args!("{}", self.reg_rmt_clk_div_denominator().bits()),
            )
            .field(
                "reg_adc_clk_src_sel",
                &format_args!("{}", self.reg_adc_clk_src_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL22_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ledc_clk_src_sel(&mut self) -> REG_LEDC_CLK_SRC_SEL_W<PERI_CLK_CTRL22_SPEC> {
        REG_LEDC_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ledc_clk_en(&mut self) -> REG_LEDC_CLK_EN_W<PERI_CLK_CTRL22_SPEC> {
        REG_LEDC_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rmt_clk_src_sel(&mut self) -> REG_RMT_CLK_SRC_SEL_W<PERI_CLK_CTRL22_SPEC> {
        REG_RMT_CLK_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rmt_clk_en(&mut self) -> REG_RMT_CLK_EN_W<PERI_CLK_CTRL22_SPEC> {
        REG_RMT_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rmt_clk_div_num(&mut self) -> REG_RMT_CLK_DIV_NUM_W<PERI_CLK_CTRL22_SPEC> {
        REG_RMT_CLK_DIV_NUM_W::new(self, 6)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rmt_clk_div_numerator(
        &mut self,
    ) -> REG_RMT_CLK_DIV_NUMERATOR_W<PERI_CLK_CTRL22_SPEC> {
        REG_RMT_CLK_DIV_NUMERATOR_W::new(self, 14)
    }
    #[doc = "Bits 22:29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rmt_clk_div_denominator(
        &mut self,
    ) -> REG_RMT_CLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL22_SPEC> {
        REG_RMT_CLK_DIV_DENOMINATOR_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_adc_clk_src_sel(&mut self) -> REG_ADC_CLK_SRC_SEL_W<PERI_CLK_CTRL22_SPEC> {
        REG_ADC_CLK_SRC_SEL_W::new(self, 30)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL22_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl22::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL22_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl22::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL22_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL22 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
