#[doc = "Register `REGDMA_CONF` reader"]
pub type R = crate::R<REGDMA_CONF_SPEC>;
#[doc = "Register `REGDMA_CONF` writer"]
pub type W = crate::W<REGDMA_CONF_SPEC>;
#[doc = "Field `REGDMA_CLK_EN` reader - Set 1 to enable regdma clock"]
pub type REGDMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `REGDMA_CLK_EN` writer - Set 1 to enable regdma clock"]
pub type REGDMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_RST_EN` reader - Set 0 to reset regdma module"]
pub type REGDMA_RST_EN_R = crate::BitReader;
#[doc = "Field `REGDMA_RST_EN` writer - Set 0 to reset regdma module"]
pub type REGDMA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable regdma clock"]
    #[inline(always)]
    pub fn regdma_clk_en(&self) -> REGDMA_CLK_EN_R {
        REGDMA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset regdma module"]
    #[inline(always)]
    pub fn regdma_rst_en(&self) -> REGDMA_RST_EN_R {
        REGDMA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CONF")
            .field(
                "regdma_clk_en",
                &format_args!("{}", self.regdma_clk_en().bit()),
            )
            .field(
                "regdma_rst_en",
                &format_args!("{}", self.regdma_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable regdma clock"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_clk_en(&mut self) -> REGDMA_CLK_EN_W<REGDMA_CONF_SPEC> {
        REGDMA_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset regdma module"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_rst_en(&mut self) -> REGDMA_RST_EN_W<REGDMA_CONF_SPEC> {
        REGDMA_RST_EN_W::new(self, 1)
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
#[doc = "REGDMA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regdma_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_conf::R`](R) reader structure"]
impl crate::Readable for REGDMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_conf::W`](W) writer structure"]
impl crate::Writable for REGDMA_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_CONF to value 0"]
impl crate::Resettable for REGDMA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
