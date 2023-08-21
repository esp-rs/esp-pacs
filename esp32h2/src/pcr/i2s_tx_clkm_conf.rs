#[doc = "Register `I2S_TX_CLKM_CONF` reader"]
pub type R = crate::R<I2S_TX_CLKM_CONF_SPEC>;
#[doc = "Register `I2S_TX_CLKM_CONF` writer"]
pub type W = crate::W<I2S_TX_CLKM_CONF_SPEC>;
#[doc = "Field `I2S_TX_CLKM_DIV_NUM` reader - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
pub type I2S_TX_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `I2S_TX_CLKM_DIV_NUM` writer - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
pub type I2S_TX_CLKM_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `I2S_TX_CLKM_SEL` reader - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub type I2S_TX_CLKM_SEL_R = crate::FieldReader;
#[doc = "Field `I2S_TX_CLKM_SEL` writer - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub type I2S_TX_CLKM_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `I2S_TX_CLKM_EN` reader - Set 1 to enable i2s_tx function clock"]
pub type I2S_TX_CLKM_EN_R = crate::BitReader;
#[doc = "Field `I2S_TX_CLKM_EN` writer - Set 1 to enable i2s_tx function clock"]
pub type I2S_TX_CLKM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 12:19 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
    #[inline(always)]
    pub fn i2s_tx_clkm_div_num(&self) -> I2S_TX_CLKM_DIV_NUM_R {
        I2S_TX_CLKM_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn i2s_tx_clkm_sel(&self) -> I2S_TX_CLKM_SEL_R {
        I2S_TX_CLKM_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable i2s_tx function clock"]
    #[inline(always)]
    pub fn i2s_tx_clkm_en(&self) -> I2S_TX_CLKM_EN_R {
        I2S_TX_CLKM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S_TX_CLKM_CONF")
            .field(
                "i2s_tx_clkm_div_num",
                &format_args!("{}", self.i2s_tx_clkm_div_num().bits()),
            )
            .field(
                "i2s_tx_clkm_sel",
                &format_args!("{}", self.i2s_tx_clkm_sel().bits()),
            )
            .field(
                "i2s_tx_clkm_en",
                &format_args!("{}", self.i2s_tx_clkm_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2S_TX_CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 12:19 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_clkm_div_num(&mut self) -> I2S_TX_CLKM_DIV_NUM_W<I2S_TX_CLKM_CONF_SPEC, 12> {
        I2S_TX_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 20:21 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_clkm_sel(&mut self) -> I2S_TX_CLKM_SEL_W<I2S_TX_CLKM_CONF_SPEC, 20> {
        I2S_TX_CLKM_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable i2s_tx function clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_clkm_en(&mut self) -> I2S_TX_CLKM_EN_W<I2S_TX_CLKM_CONF_SPEC, 22> {
        I2S_TX_CLKM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S_TX_CLKM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_tx_clkm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_tx_clkm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_TX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for I2S_TX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_tx_clkm_conf::R`](R) reader structure"]
impl crate::Readable for I2S_TX_CLKM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_tx_clkm_conf::W`](W) writer structure"]
impl crate::Writable for I2S_TX_CLKM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_TX_CLKM_CONF to value 0x0040_2000"]
impl crate::Resettable for I2S_TX_CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_2000;
}
