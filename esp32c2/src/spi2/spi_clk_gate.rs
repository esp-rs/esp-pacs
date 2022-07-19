#[doc = "Register `SPI_CLK_GATE` reader"]
pub struct R(crate::R<SPI_CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CLK_GATE` writer"]
pub struct W(crate::W<SPI_CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CLK_GATE_SPEC>;
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
impl From<crate::W<SPI_CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_CLK_EN` reader - Set this bit to enable clk gate"]
pub type SPI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_CLK_EN` writer - Set this bit to enable clk gate"]
pub type SPI_CLK_EN_W<'a> = crate::BitWriter<'a, u32, SPI_CLK_GATE_SPEC, bool, 0>;
#[doc = "Field `SPI_MST_CLK_ACTIVE` reader - Set this bit to power on the SPI module clock."]
pub type SPI_MST_CLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MST_CLK_ACTIVE` writer - Set this bit to power on the SPI module clock."]
pub type SPI_MST_CLK_ACTIVE_W<'a> = crate::BitWriter<'a, u32, SPI_CLK_GATE_SPEC, bool, 1>;
#[doc = "Field `SPI_MST_CLK_SEL` reader - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
pub type SPI_MST_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MST_CLK_SEL` writer - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
pub type SPI_MST_CLK_SEL_W<'a> = crate::BitWriter<'a, u32, SPI_CLK_GATE_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to power on the SPI module clock."]
    #[inline(always)]
    pub fn spi_mst_clk_active(&self) -> SPI_MST_CLK_ACTIVE_R {
        SPI_MST_CLK_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
    #[inline(always)]
    pub fn spi_mst_clk_sel(&self) -> SPI_MST_CLK_SEL_R {
        SPI_MST_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W {
        SPI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to power on the SPI module clock."]
    #[inline(always)]
    pub fn spi_mst_clk_active(&mut self) -> SPI_MST_CLK_ACTIVE_W {
        SPI_MST_CLK_ACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
    #[inline(always)]
    pub fn spi_mst_clk_sel(&mut self) -> SPI_MST_CLK_SEL_W {
        SPI_MST_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI module clock and register clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_clk_gate](index.html) module"]
pub struct SPI_CLK_GATE_SPEC;
impl crate::RegisterSpec for SPI_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_clk_gate::R](R) reader structure"]
impl crate::Readable for SPI_CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_clk_gate::W](W) writer structure"]
impl crate::Writable for SPI_CLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CLK_GATE to value 0"]
impl crate::Resettable for SPI_CLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
