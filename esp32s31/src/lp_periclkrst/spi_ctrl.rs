#[doc = "Register `SPI_CTRL` reader"]
pub type R = crate::R<SPI_CTRL_SPEC>;
#[doc = "Register `SPI_CTRL` writer"]
pub type W = crate::W<SPI_CTRL_SPEC>;
#[doc = "Field `LP_SPI_CLK_EN` reader - need_des"]
pub type LP_SPI_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_SPI_CLK_EN` writer - need_des"]
pub type LP_SPI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI_RST_EN` reader - need_des"]
pub type LP_SPI_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_SPI_RST_EN` writer - need_des"]
pub type LP_SPI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_spi_clk_en(&self) -> LP_SPI_CLK_EN_R {
        LP_SPI_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_spi_rst_en(&self) -> LP_SPI_RST_EN_R {
        LP_SPI_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CTRL")
            .field("lp_spi_clk_en", &self.lp_spi_clk_en())
            .field("lp_spi_rst_en", &self.lp_spi_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_spi_clk_en(&mut self) -> LP_SPI_CLK_EN_W<'_, SPI_CTRL_SPEC> {
        LP_SPI_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_spi_rst_en(&mut self) -> LP_SPI_RST_EN_W<'_, SPI_CTRL_SPEC> {
        LP_SPI_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0"]
impl crate::Resettable for SPI_CTRL_SPEC {}
