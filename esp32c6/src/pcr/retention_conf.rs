#[doc = "Register `RETENTION_CONF` reader"]
pub type R = crate::R<RETENTION_CONF_SPEC>;
#[doc = "Register `RETENTION_CONF` writer"]
pub type W = crate::W<RETENTION_CONF_SPEC>;
#[doc = "Field `RETENTION_CLK_EN` reader - Set 1 to enable retention clock"]
pub type RETENTION_CLK_EN_R = crate::BitReader;
#[doc = "Field `RETENTION_CLK_EN` writer - Set 1 to enable retention clock"]
pub type RETENTION_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETENTION_RST_EN` reader - Set 0 to reset retention module"]
pub type RETENTION_RST_EN_R = crate::BitReader;
#[doc = "Field `RETENTION_RST_EN` writer - Set 0 to reset retention module"]
pub type RETENTION_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable retention clock"]
    #[inline(always)]
    pub fn retention_clk_en(&self) -> RETENTION_CLK_EN_R {
        RETENTION_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset retention module"]
    #[inline(always)]
    pub fn retention_rst_en(&self) -> RETENTION_RST_EN_R {
        RETENTION_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CONF")
            .field("retention_clk_en", &self.retention_clk_en().bit())
            .field("retention_rst_en", &self.retention_rst_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable retention clock"]
    #[inline(always)]
    #[must_use]
    pub fn retention_clk_en(&mut self) -> RETENTION_CLK_EN_W<RETENTION_CONF_SPEC> {
        RETENTION_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset retention module"]
    #[inline(always)]
    #[must_use]
    pub fn retention_rst_en(&mut self) -> RETENTION_RST_EN_W<RETENTION_CONF_SPEC> {
        RETENTION_RST_EN_W::new(self, 1)
    }
}
#[doc = "retention configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CONF_SPEC;
impl crate::RegisterSpec for RETENTION_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_conf::R`](R) reader structure"]
impl crate::Readable for RETENTION_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_conf::W`](W) writer structure"]
impl crate::Writable for RETENTION_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETENTION_CONF to value 0"]
impl crate::Resettable for RETENTION_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
