#[doc = "Register `APB_ADC_CLKM_CONF` reader"]
pub struct R(crate::R<APB_ADC_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_ADC_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_ADC_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_ADC_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_ADC_CLKM_CONF` writer"]
pub struct W(crate::W<APB_ADC_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_ADC_CLKM_CONF_SPEC>;
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
impl From<crate::W<APB_ADC_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_ADC_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_CLKM_DIV_NUM` reader - Integral I2S clock divider value"]
pub type REG_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_CLKM_DIV_NUM` writer - Integral I2S clock divider value"]
pub type REG_CLKM_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, APB_ADC_CLKM_CONF_SPEC, 8, O>;
#[doc = "Field `REG_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub type REG_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `REG_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub type REG_CLKM_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, APB_ADC_CLKM_CONF_SPEC, 6, O>;
#[doc = "Field `REG_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub type REG_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `REG_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub type REG_CLKM_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, APB_ADC_CLKM_CONF_SPEC, 6, O>;
#[doc = "Field `CLK_EN` reader - Need add description"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Need add description"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB_ADC_CLKM_CONF_SPEC, O>;
#[doc = "Field `REG_CLK_SEL` reader - Set this bit to enable clk_apll"]
pub type REG_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `REG_CLK_SEL` writer - Set this bit to enable clk_apll"]
pub type REG_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, APB_ADC_CLKM_CONF_SPEC, 2, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkm_div_num(&mut self) -> REG_CLKM_DIV_NUM_W<0> {
        REG_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:13 - Fractional clock divider numerator value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkm_div_b(&mut self) -> REG_CLKM_DIV_B_W<8> {
        REG_CLKM_DIV_B_W::new(self)
    }
    #[doc = "Bits 14:19 - Fractional clock divider denominator value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clkm_div_a(&mut self) -> REG_CLKM_DIV_A_W<14> {
        REG_CLKM_DIV_A_W::new(self)
    }
    #[doc = "Bit 20 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<20> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 21:22 - Set this bit to enable clk_apll"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clk_sel(&mut self) -> REG_CLK_SEL_W<21> {
        REG_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_adc_clkm_conf](index.html) module"]
pub struct APB_ADC_CLKM_CONF_SPEC;
impl crate::RegisterSpec for APB_ADC_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_adc_clkm_conf::R](R) reader structure"]
impl crate::Readable for APB_ADC_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_adc_clkm_conf::W](W) writer structure"]
impl crate::Writable for APB_ADC_CLKM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_ADC_CLKM_CONF to value 0x04"]
impl crate::Resettable for APB_ADC_CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
