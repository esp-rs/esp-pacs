#[doc = "Register `GMAC0_PAD_BIST_CFG` writer"]
pub type W = crate::W<GMAC0_PAD_BIST_CFG_SPEC>;
#[doc = "Field `GMAC0_PAD_BIST_START` writer - Write 1 to start gmac0 pad bist"]
pub type GMAC0_PAD_BIST_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GMAC0_PAD_BIST_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start gmac0 pad bist"]
    #[inline(always)]
    pub fn gmac0_pad_bist_start(&mut self) -> GMAC0_PAD_BIST_START_W<'_, GMAC0_PAD_BIST_CFG_SPEC> {
        GMAC0_PAD_BIST_START_W::new(self, 0)
    }
}
#[doc = "gmac0 pad bist control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_PAD_BIST_CFG_SPEC;
impl crate::RegisterSpec for GMAC0_PAD_BIST_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gmac0_pad_bist_cfg::W`](W) writer structure"]
impl crate::Writable for GMAC0_PAD_BIST_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC0_PAD_BIST_CFG to value 0"]
impl crate::Resettable for GMAC0_PAD_BIST_CFG_SPEC {}
