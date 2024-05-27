///Register `SPI2_CONF` reader
pub type R = crate::R<SPI2_CONF_SPEC>;
///Register `SPI2_CONF` writer
pub type W = crate::W<SPI2_CONF_SPEC>;
///Field `SPI2_CLK_EN` reader - Set 1 to enable spi2 apb clock
pub type SPI2_CLK_EN_R = crate::BitReader;
///Field `SPI2_CLK_EN` writer - Set 1 to enable spi2 apb clock
pub type SPI2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2_RST_EN` reader - Set 0 to reset spi2 module
pub type SPI2_RST_EN_R = crate::BitReader;
///Field `SPI2_RST_EN` writer - Set 0 to reset spi2 module
pub type SPI2_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2_READY` reader - Query this field after reset spi2 module
pub type SPI2_READY_R = crate::BitReader;
impl R {
    ///Bit 0 - Set 1 to enable spi2 apb clock
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset spi2 module
    #[inline(always)]
    pub fn spi2_rst_en(&self) -> SPI2_RST_EN_R {
        SPI2_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Query this field after reset spi2 module
    #[inline(always)]
    pub fn spi2_ready(&self) -> SPI2_READY_R {
        SPI2_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2_CONF")
            .field("spi2_clk_en", &self.spi2_clk_en())
            .field("spi2_rst_en", &self.spi2_rst_en())
            .field("spi2_ready", &self.spi2_ready())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable spi2 apb clock
    #[inline(always)]
    #[must_use]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W<SPI2_CONF_SPEC> {
        SPI2_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset spi2 module
    #[inline(always)]
    #[must_use]
    pub fn spi2_rst_en(&mut self) -> SPI2_RST_EN_W<SPI2_CONF_SPEC> {
        SPI2_RST_EN_W::new(self, 1)
    }
}
/**SPI2 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi2_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPI2_CONF_SPEC;
impl crate::RegisterSpec for SPI2_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spi2_conf::R`](R) reader structure
impl crate::Readable for SPI2_CONF_SPEC {}
///`write(|w| ..)` method takes [`spi2_conf::W`](W) writer structure
impl crate::Writable for SPI2_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI2_CONF to value 0x05
impl crate::Resettable for SPI2_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
