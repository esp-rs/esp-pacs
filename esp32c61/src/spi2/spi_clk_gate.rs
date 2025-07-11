#[doc = "Register `SPI_CLK_GATE` reader"]
pub type R = crate::R<SPI_CLK_GATE_SPEC>;
#[doc = "Register `SPI_CLK_GATE` writer"]
pub type W = crate::W<SPI_CLK_GATE_SPEC>;
#[doc = "Field `SPI_CLK_EN` reader - Configures whether or not to enable clock gate.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI_CLK_EN` writer - Configures whether or not to enable clock gate.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SPI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_CLK_ACTIVE` reader - Set this bit to power on the SPI module clock."]
pub type SPI_MST_CLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_MST_CLK_ACTIVE` writer - Set this bit to power on the SPI module clock."]
pub type SPI_MST_CLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_CLK_SEL` reader - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
pub type SPI_MST_CLK_SEL_R = crate::BitReader;
#[doc = "Field `SPI_MST_CLK_SEL` writer - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
pub type SPI_MST_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable clock gate.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CLK_GATE")
            .field("spi_clk_en", &self.spi_clk_en())
            .field("spi_mst_clk_active", &self.spi_mst_clk_active())
            .field("spi_mst_clk_sel", &self.spi_mst_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable clock gate.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W<SPI_CLK_GATE_SPEC> {
        SPI_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to power on the SPI module clock."]
    #[inline(always)]
    pub fn spi_mst_clk_active(&mut self) -> SPI_MST_CLK_ACTIVE_W<SPI_CLK_GATE_SPEC> {
        SPI_MST_CLK_ACTIVE_W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
    #[inline(always)]
    pub fn spi_mst_clk_sel(&mut self) -> SPI_MST_CLK_SEL_W<SPI_CLK_GATE_SPEC> {
        SPI_MST_CLK_SEL_W::new(self, 2)
    }
}
#[doc = "SPI module clock and register clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CLK_GATE_SPEC;
impl crate::RegisterSpec for SPI_CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_clk_gate::R`](R) reader structure"]
impl crate::Readable for SPI_CLK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_clk_gate::W`](W) writer structure"]
impl crate::Writable for SPI_CLK_GATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CLK_GATE to value 0"]
impl crate::Resettable for SPI_CLK_GATE_SPEC {}
