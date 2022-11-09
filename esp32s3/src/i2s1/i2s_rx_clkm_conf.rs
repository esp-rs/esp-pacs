#[doc = "Register `I2S_RX_CLKM_CONF` reader"]
pub struct R(crate::R<I2S_RX_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_RX_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_RX_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_RX_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_RX_CLKM_CONF` writer"]
pub struct W(crate::W<I2S_RX_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_RX_CLKM_CONF_SPEC>;
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
impl From<crate::W<I2S_RX_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_RX_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_RX_CLKM_DIV_NUM` reader - Integral I2S clock divider value"]
pub type I2S_RX_CLKM_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_RX_CLKM_DIV_NUM` writer - Integral I2S clock divider value"]
pub type I2S_RX_CLKM_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_RX_CLKM_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `I2S_RX_CLK_ACTIVE` reader - I2S Rx module clock enable signal."]
pub type I2S_RX_CLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `I2S_RX_CLK_ACTIVE` writer - I2S Rx module clock enable signal."]
pub type I2S_RX_CLK_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_RX_CLKM_CONF_SPEC, bool, O>;
#[doc = "Field `I2S_RX_CLK_SEL` reader - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub type I2S_RX_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_RX_CLK_SEL` writer - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub type I2S_RX_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_RX_CLKM_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S_MCLK_SEL` reader - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
pub type I2S_MCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2S_MCLK_SEL` writer - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
pub type I2S_MCLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_RX_CLKM_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    pub fn i2s_rx_clkm_div_num(&self) -> I2S_RX_CLKM_DIV_NUM_R {
        I2S_RX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 26 - I2S Rx module clock enable signal."]
    #[inline(always)]
    pub fn i2s_rx_clk_active(&self) -> I2S_RX_CLK_ACTIVE_R {
        I2S_RX_CLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn i2s_rx_clk_sel(&self) -> I2S_RX_CLK_SEL_R {
        I2S_RX_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
    #[inline(always)]
    pub fn i2s_mclk_sel(&self) -> I2S_MCLK_SEL_R {
        I2S_MCLK_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rx_clkm_div_num(&mut self) -> I2S_RX_CLKM_DIV_NUM_W<0> {
        I2S_RX_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 26 - I2S Rx module clock enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rx_clk_active(&mut self) -> I2S_RX_CLK_ACTIVE_W<26> {
        I2S_RX_CLK_ACTIVE_W::new(self)
    }
    #[doc = "Bits 27:28 - Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rx_clk_sel(&mut self) -> I2S_RX_CLK_SEL_W<27> {
        I2S_RX_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 29 - 0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_mclk_sel(&mut self) -> I2S_MCLK_SEL_W<29> {
        I2S_MCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rx_clkm_conf](index.html) module"]
pub struct I2S_RX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for I2S_RX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_rx_clkm_conf::R](R) reader structure"]
impl crate::Readable for I2S_RX_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_rx_clkm_conf::W](W) writer structure"]
impl crate::Writable for I2S_RX_CLKM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_RX_CLKM_CONF to value 0x02"]
impl crate::Resettable for I2S_RX_CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
