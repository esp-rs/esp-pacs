#[doc = "Register `GDMA_CONF` reader"]
pub type R = crate::R<GDMA_CONF_SPEC>;
#[doc = "Register `GDMA_CONF` writer"]
pub type W = crate::W<GDMA_CONF_SPEC>;
#[doc = "Field `GDMA_CLK_EN` reader - Set 1 to enable gdma clock"]
pub type GDMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `GDMA_CLK_EN` writer - Set 1 to enable gdma clock"]
pub type GDMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_RST_EN` reader - Set 0 to reset gdma module"]
pub type GDMA_RST_EN_R = crate::BitReader;
#[doc = "Field `GDMA_RST_EN` writer - Set 0 to reset gdma module"]
pub type GDMA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable gdma clock"]
    #[inline(always)]
    pub fn gdma_clk_en(&self) -> GDMA_CLK_EN_R {
        GDMA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset gdma module"]
    #[inline(always)]
    pub fn gdma_rst_en(&self) -> GDMA_RST_EN_R {
        GDMA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_CONF")
            .field("gdma_clk_en", &self.gdma_clk_en().bit())
            .field("gdma_rst_en", &self.gdma_rst_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GDMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable gdma clock"]
    #[inline(always)]
    #[must_use]
    pub fn gdma_clk_en(&mut self) -> GDMA_CLK_EN_W<GDMA_CONF_SPEC> {
        GDMA_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset gdma module"]
    #[inline(always)]
    #[must_use]
    pub fn gdma_rst_en(&mut self) -> GDMA_RST_EN_W<GDMA_CONF_SPEC> {
        GDMA_RST_EN_W::new(self, 1)
    }
}
#[doc = "GDMA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdma_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdma_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDMA_CONF_SPEC;
impl crate::RegisterSpec for GDMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_conf::R`](R) reader structure"]
impl crate::Readable for GDMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdma_conf::W`](W) writer structure"]
impl crate::Writable for GDMA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDMA_CONF to value 0x01"]
impl crate::Resettable for GDMA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
