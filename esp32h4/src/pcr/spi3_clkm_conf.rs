#[doc = "Register `SPI3_CLKM_CONF` reader"]
pub type R = crate::R<SPI3_CLKM_CONF_SPEC>;
#[doc = "Register `SPI3_CLKM_CONF` writer"]
pub type W = crate::W<SPI3_CLKM_CONF_SPEC>;
#[doc = "Field `SPI3_CLKM_DIV_NUM` reader - The integral part of the frequency divider factor of the spi3_mst clock."]
pub type SPI3_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SPI3_CLKM_DIV_NUM` writer - The integral part of the frequency divider factor of the spi3_mst clock."]
pub type SPI3_CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI3_CLKM_SEL` reader - Configures the clock source of SPI3.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F160M_CLK\\\\ 2: RC_FAST_CLK\\\\ 3: PLL_F120M_CLK\\\\"]
pub type SPI3_CLKM_SEL_R = crate::FieldReader;
#[doc = "Field `SPI3_CLKM_SEL` writer - Configures the clock source of SPI3.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F160M_CLK\\\\ 2: RC_FAST_CLK\\\\ 3: PLL_F120M_CLK\\\\"]
pub type SPI3_CLKM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI3_CLKM_EN` reader - Set 1 to enable spi3 function clock"]
pub type SPI3_CLKM_EN_R = crate::BitReader;
#[doc = "Field `SPI3_CLKM_EN` writer - Set 1 to enable spi3 function clock"]
pub type SPI3_CLKM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the spi3_mst clock."]
    #[inline(always)]
    pub fn spi3_clkm_div_num(&self) -> SPI3_CLKM_DIV_NUM_R {
        SPI3_CLKM_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Configures the clock source of SPI3.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F160M_CLK\\\\ 2: RC_FAST_CLK\\\\ 3: PLL_F120M_CLK\\\\"]
    #[inline(always)]
    pub fn spi3_clkm_sel(&self) -> SPI3_CLKM_SEL_R {
        SPI3_CLKM_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable spi3 function clock"]
    #[inline(always)]
    pub fn spi3_clkm_en(&self) -> SPI3_CLKM_EN_R {
        SPI3_CLKM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI3_CLKM_CONF")
            .field("spi3_clkm_div_num", &self.spi3_clkm_div_num())
            .field("spi3_clkm_sel", &self.spi3_clkm_sel())
            .field("spi3_clkm_en", &self.spi3_clkm_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the spi3_mst clock."]
    #[inline(always)]
    pub fn spi3_clkm_div_num(&mut self) -> SPI3_CLKM_DIV_NUM_W<'_, SPI3_CLKM_CONF_SPEC> {
        SPI3_CLKM_DIV_NUM_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - Configures the clock source of SPI3.\\\\ 0 (default): XTAL_CLK\\\\ 1: PLL_F160M_CLK\\\\ 2: RC_FAST_CLK\\\\ 3: PLL_F120M_CLK\\\\"]
    #[inline(always)]
    pub fn spi3_clkm_sel(&mut self) -> SPI3_CLKM_SEL_W<'_, SPI3_CLKM_CONF_SPEC> {
        SPI3_CLKM_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable spi3 function clock"]
    #[inline(always)]
    pub fn spi3_clkm_en(&mut self) -> SPI3_CLKM_EN_W<'_, SPI3_CLKM_CONF_SPEC> {
        SPI3_CLKM_EN_W::new(self, 22)
    }
}
#[doc = "SPI3_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi3_clkm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi3_clkm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI3_CLKM_CONF_SPEC;
impl crate::RegisterSpec for SPI3_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi3_clkm_conf::R`](R) reader structure"]
impl crate::Readable for SPI3_CLKM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi3_clkm_conf::W`](W) writer structure"]
impl crate::Writable for SPI3_CLKM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI3_CLKM_CONF to value 0"]
impl crate::Resettable for SPI3_CLKM_CONF_SPEC {}
