#[doc = "Register `APB_ADC_CLKM_CONF` reader"]
pub type R = crate::R<APB_ADC_CLKM_CONF_SPEC>;
#[doc = "Register `APB_ADC_CLKM_CONF` writer"]
pub type W = crate::W<APB_ADC_CLKM_CONF_SPEC>;
#[doc = "Field `REG_CLKM_DIV_NUM` reader - Integral I2S clock divider value"]
pub type REG_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_CLKM_DIV_NUM` writer - Integral I2S clock divider value"]
pub type REG_CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub type REG_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `REG_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub type REG_CLKM_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REG_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub type REG_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `REG_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub type REG_CLKM_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLK_EN` reader - Need add description"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Need add description"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CLK_SEL` reader - Set this bit to enable clk_apll"]
pub type REG_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `REG_CLK_SEL` writer - Set this bit to enable clk_apll"]
pub type REG_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    pub fn reg_clkm_div_num(&self) -> REG_CLKM_DIV_NUM_R {
        REG_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn reg_clkm_div_b(&self) -> REG_CLKM_DIV_B_R {
        REG_CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn reg_clkm_div_a(&self) -> REG_CLKM_DIV_A_R {
        REG_CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - Need add description"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Set this bit to enable clk_apll"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> REG_CLK_SEL_R {
        REG_CLK_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_ADC_CLKM_CONF")
            .field(
                "reg_clkm_div_num",
                &format_args!("{}", self.reg_clkm_div_num().bits()),
            )
            .field(
                "reg_clkm_div_b",
                &format_args!("{}", self.reg_clkm_div_b().bits()),
            )
            .field(
                "reg_clkm_div_a",
                &format_args!("{}", self.reg_clkm_div_a().bits()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "reg_clk_sel",
                &format_args!("{}", self.reg_clk_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_ADC_CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkm_div_num(&mut self) -> REG_CLKM_DIV_NUM_W<APB_ADC_CLKM_CONF_SPEC> {
        REG_CLKM_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Fractional clock divider numerator value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkm_div_b(&mut self) -> REG_CLKM_DIV_B_W<APB_ADC_CLKM_CONF_SPEC> {
        REG_CLKM_DIV_B_W::new(self, 8)
    }
    #[doc = "Bits 14:19 - Fractional clock divider denominator value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkm_div_a(&mut self) -> REG_CLKM_DIV_A_W<APB_ADC_CLKM_CONF_SPEC> {
        REG_CLKM_DIV_A_W::new(self, 14)
    }
    #[doc = "Bit 20 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<APB_ADC_CLKM_CONF_SPEC> {
        CLK_EN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Set this bit to enable clk_apll"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clk_sel(&mut self) -> REG_CLK_SEL_W<APB_ADC_CLKM_CONF_SPEC> {
        REG_CLK_SEL_W::new(self, 21)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_clkm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_clkm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_ADC_CLKM_CONF_SPEC;
impl crate::RegisterSpec for APB_ADC_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_adc_clkm_conf::R`](R) reader structure"]
impl crate::Readable for APB_ADC_CLKM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_adc_clkm_conf::W`](W) writer structure"]
impl crate::Writable for APB_ADC_CLKM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_ADC_CLKM_CONF to value 0x04"]
impl crate::Resettable for APB_ADC_CLKM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
