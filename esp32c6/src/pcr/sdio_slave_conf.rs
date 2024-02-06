#[doc = "Register `SDIO_SLAVE_CONF` reader"]
pub type R = crate::R<SDIO_SLAVE_CONF_SPEC>;
#[doc = "Register `SDIO_SLAVE_CONF` writer"]
pub type W = crate::W<SDIO_SLAVE_CONF_SPEC>;
#[doc = "Field `SDIO_SLAVE_CLK_EN` reader - Set 1 to enable sdio_slave clock"]
pub type SDIO_SLAVE_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_SLAVE_CLK_EN` writer - Set 1 to enable sdio_slave clock"]
pub type SDIO_SLAVE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_SLAVE_RST_EN` reader - Set 0 to reset sdio_slave module"]
pub type SDIO_SLAVE_RST_EN_R = crate::BitReader;
#[doc = "Field `SDIO_SLAVE_RST_EN` writer - Set 0 to reset sdio_slave module"]
pub type SDIO_SLAVE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable sdio_slave clock"]
    #[inline(always)]
    pub fn sdio_slave_clk_en(&self) -> SDIO_SLAVE_CLK_EN_R {
        SDIO_SLAVE_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset sdio_slave module"]
    #[inline(always)]
    pub fn sdio_slave_rst_en(&self) -> SDIO_SLAVE_RST_EN_R {
        SDIO_SLAVE_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SLAVE_CONF")
            .field(
                "sdio_slave_clk_en",
                &format_args!("{}", self.sdio_slave_clk_en().bit()),
            )
            .field(
                "sdio_slave_rst_en",
                &format_args!("{}", self.sdio_slave_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_SLAVE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable sdio_slave clock"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_slave_clk_en(&mut self) -> SDIO_SLAVE_CLK_EN_W<SDIO_SLAVE_CONF_SPEC> {
        SDIO_SLAVE_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset sdio_slave module"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_slave_rst_en(&mut self) -> SDIO_SLAVE_RST_EN_W<SDIO_SLAVE_CONF_SPEC> {
        SDIO_SLAVE_RST_EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO_SLAVE configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_slave_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_slave_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_SLAVE_CONF_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_slave_conf::R`](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_slave_conf::W`](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_SLAVE_CONF to value 0x01"]
impl crate::Resettable for SDIO_SLAVE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
