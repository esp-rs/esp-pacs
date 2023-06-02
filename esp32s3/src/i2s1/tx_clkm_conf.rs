#[doc = "Register `TX_CLKM_CONF` reader"]
pub struct R(crate::R<TX_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CLKM_CONF` writer"]
pub struct W(crate::W<TX_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CLKM_CONF_SPEC>;
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
impl From<crate::W<TX_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CLKM_DIV_NUM` reader - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
pub type TX_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `TX_CLKM_DIV_NUM` writer - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
pub type TX_CLKM_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, TX_CLKM_CONF_SPEC, 8, O>;
#[doc = "Field `TX_CLK_ACTIVE` reader - I2S Tx module clock enable signal."]
pub type TX_CLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `TX_CLK_ACTIVE` writer - I2S Tx module clock enable signal."]
pub type TX_CLK_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, TX_CLKM_CONF_SPEC, O>;
#[doc = "Field `TX_CLK_SEL` reader - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub type TX_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TX_CLK_SEL` writer - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
pub type TX_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TX_CLKM_CONF_SPEC, 2, O>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TX_CLKM_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
    #[inline(always)]
    pub fn tx_clkm_div_num(&self) -> TX_CLKM_DIV_NUM_R {
        TX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 26 - I2S Tx module clock enable signal."]
    #[inline(always)]
    pub fn tx_clk_active(&self) -> TX_CLK_ACTIVE_R {
        TX_CLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    pub fn tx_clk_sel(&self) -> TX_CLK_SEL_R {
        TX_CLK_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CLKM_CONF")
            .field(
                "tx_clkm_div_num",
                &format_args!("{}", self.tx_clkm_div_num().bits()),
            )
            .field(
                "tx_clk_active",
                &format_args!("{}", self.tx_clk_active().bit()),
            )
            .field("tx_clk_sel", &format_args!("{}", self.tx_clk_sel().bits()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b &lt;= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
    #[inline(always)]
    #[must_use]
    pub fn tx_clkm_div_num(&mut self) -> TX_CLKM_DIV_NUM_W<0> {
        TX_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 26 - I2S Tx module clock enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_active(&mut self) -> TX_CLK_ACTIVE_W<26> {
        TX_CLK_ACTIVE_W::new(self)
    }
    #[doc = "Bits 27:28 - Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_sel(&mut self) -> TX_CLK_SEL_W<27> {
        TX_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to enable clk gate"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<29> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_clkm_conf](index.html) module"]
pub struct TX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for TX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_clkm_conf::R](R) reader structure"]
impl crate::Readable for TX_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_clkm_conf::W](W) writer structure"]
impl crate::Writable for TX_CLKM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CLKM_CONF to value 0x02"]
impl crate::Resettable for TX_CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
