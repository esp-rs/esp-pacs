#[doc = "Register `SDIO_PAD_BIST_CFG` writer"]
pub type W = crate::W<SDIO_PAD_BIST_CFG_SPEC>;
#[doc = "Field `SDIO_PAD_BIST_START` writer - Write 1 to start sdio pad bist"]
pub type SDIO_PAD_BIST_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_PAD_BIST_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start sdio pad bist"]
    #[inline(always)]
    pub fn sdio_pad_bist_start(&mut self) -> SDIO_PAD_BIST_START_W<'_, SDIO_PAD_BIST_CFG_SPEC> {
        SDIO_PAD_BIST_START_W::new(self, 0)
    }
}
#[doc = "sdio pad bist control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_PAD_BIST_CFG_SPEC;
impl crate::RegisterSpec for SDIO_PAD_BIST_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sdio_pad_bist_cfg::W`](W) writer structure"]
impl crate::Writable for SDIO_PAD_BIST_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_PAD_BIST_CFG to value 0"]
impl crate::Resettable for SDIO_PAD_BIST_CFG_SPEC {}
