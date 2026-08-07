#[doc = "Register `SPI3_CONF` reader"]
pub type R = crate::R<SPI3_CONF_SPEC>;
#[doc = "Register `SPI3_CONF` writer"]
pub type W = crate::W<SPI3_CONF_SPEC>;
#[doc = "Field `SPI3_CLK_EN` reader - Set 1 to enable spi3 apb clock"]
pub type SPI3_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI3_CLK_EN` writer - Set 1 to enable spi3 apb clock"]
pub type SPI3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_RST_EN` reader - Set 1 to reset spi3 module"]
pub type SPI3_RST_EN_R = crate::BitReader;
#[doc = "Field `SPI3_RST_EN` writer - Set 1 to reset spi3 module"]
pub type SPI3_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_READY` reader - Query this field after reset spi3 module"]
pub type SPI3_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable spi3 apb clock"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset spi3 module"]
    #[inline(always)]
    pub fn spi3_rst_en(&self) -> SPI3_RST_EN_R {
        SPI3_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset spi3 module"]
    #[inline(always)]
    pub fn spi3_ready(&self) -> SPI3_READY_R {
        SPI3_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI3_CONF")
            .field("spi3_clk_en", &self.spi3_clk_en())
            .field("spi3_rst_en", &self.spi3_rst_en())
            .field("spi3_ready", &self.spi3_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable spi3 apb clock"]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W<'_, SPI3_CONF_SPEC> {
        SPI3_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset spi3 module"]
    #[inline(always)]
    pub fn spi3_rst_en(&mut self) -> SPI3_RST_EN_W<'_, SPI3_CONF_SPEC> {
        SPI3_RST_EN_W::new(self, 1)
    }
}
#[doc = "SPI3 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi3_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi3_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI3_CONF_SPEC;
impl crate::RegisterSpec for SPI3_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi3_conf::R`](R) reader structure"]
impl crate::Readable for SPI3_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi3_conf::W`](W) writer structure"]
impl crate::Writable for SPI3_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI3_CONF to value 0x06"]
impl crate::Resettable for SPI3_CONF_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
