#[doc = "Register `PERI_CLK_CTRL22` reader"]
pub type R = crate::R<PERI_CLK_CTRL22_SPEC>;
#[doc = "Register `PERI_CLK_CTRL22` writer"]
pub type W = crate::W<PERI_CLK_CTRL22_SPEC>;
#[doc = "Field `LEDC_CLK_SRC_SEL` reader - Reserved"]
pub type LEDC_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `LEDC_CLK_SRC_SEL` writer - Reserved"]
pub type LEDC_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEDC_CLK_EN` reader - Reserved"]
pub type LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - Reserved"]
pub type LEDC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_CLK_SRC_SEL` reader - Reserved"]
pub type RMT_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `RMT_CLK_SRC_SEL` writer - Reserved"]
pub type RMT_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RMT_CLK_EN` reader - Reserved"]
pub type RMT_CLK_EN_R = crate::BitReader;
#[doc = "Field `RMT_CLK_EN` writer - Reserved"]
pub type RMT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_CLK_DIV_NUM` reader - Reserved"]
pub type RMT_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `RMT_CLK_DIV_NUM` writer - Reserved"]
pub type RMT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RMT_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type RMT_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `RMT_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type RMT_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RMT_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type RMT_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `RMT_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type RMT_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_CLK_SRC_SEL` reader - Reserved"]
pub type ADC_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `ADC_CLK_SRC_SEL` writer - Reserved"]
pub type ADC_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn ledc_clk_src_sel(&self) -> LEDC_CLK_SRC_SEL_R {
        LEDC_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_src_sel(&self) -> RMT_CLK_SRC_SEL_R {
        RMT_CLK_SRC_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_num(&self) -> RMT_CLK_DIV_NUM_R {
        RMT_CLK_DIV_NUM_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_numerator(&self) -> RMT_CLK_DIV_NUMERATOR_R {
        RMT_CLK_DIV_NUMERATOR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_denominator(&self) -> RMT_CLK_DIV_DENOMINATOR_R {
        RMT_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_src_sel(&self) -> ADC_CLK_SRC_SEL_R {
        ADC_CLK_SRC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL22")
            .field("ledc_clk_src_sel", &self.ledc_clk_src_sel())
            .field("ledc_clk_en", &self.ledc_clk_en())
            .field("rmt_clk_src_sel", &self.rmt_clk_src_sel())
            .field("rmt_clk_en", &self.rmt_clk_en())
            .field("rmt_clk_div_num", &self.rmt_clk_div_num())
            .field("rmt_clk_div_numerator", &self.rmt_clk_div_numerator())
            .field("rmt_clk_div_denominator", &self.rmt_clk_div_denominator())
            .field("adc_clk_src_sel", &self.adc_clk_src_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_clk_src_sel(&mut self) -> LEDC_CLK_SRC_SEL_W<PERI_CLK_CTRL22_SPEC> {
        LEDC_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<PERI_CLK_CTRL22_SPEC> {
        LEDC_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_src_sel(&mut self) -> RMT_CLK_SRC_SEL_W<PERI_CLK_CTRL22_SPEC> {
        RMT_CLK_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<PERI_CLK_CTRL22_SPEC> {
        RMT_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_div_num(&mut self) -> RMT_CLK_DIV_NUM_W<PERI_CLK_CTRL22_SPEC> {
        RMT_CLK_DIV_NUM_W::new(self, 6)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_div_numerator(&mut self) -> RMT_CLK_DIV_NUMERATOR_W<PERI_CLK_CTRL22_SPEC> {
        RMT_CLK_DIV_NUMERATOR_W::new(self, 14)
    }
    #[doc = "Bits 22:29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_div_denominator(&mut self) -> RMT_CLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL22_SPEC> {
        RMT_CLK_DIV_DENOMINATOR_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_src_sel(&mut self) -> ADC_CLK_SRC_SEL_W<PERI_CLK_CTRL22_SPEC> {
        ADC_CLK_SRC_SEL_W::new(self, 30)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL22_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl22::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL22_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl22::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL22_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL22 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL22_SPEC {
    const RESET_VALUE: u32 = 0;
}
