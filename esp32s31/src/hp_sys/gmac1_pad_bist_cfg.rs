#[doc = "Register `GMAC1_PAD_BIST_CFG` writer"]
pub type W = crate::W<GMAC1_PAD_BIST_CFG_SPEC>;
#[doc = "Field `GMAC1_PAD_BIST_START` writer - Write 1 to start gmac1 pad bist"]
pub type GMAC1_PAD_BIST_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GMAC1_PAD_BIST_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start gmac1 pad bist"]
    #[inline(always)]
    pub fn gmac1_pad_bist_start(&mut self) -> GMAC1_PAD_BIST_START_W<'_, GMAC1_PAD_BIST_CFG_SPEC> {
        GMAC1_PAD_BIST_START_W::new(self, 0)
    }
}
#[doc = "gmac1 pad bist control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac1_pad_bist_cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC1_PAD_BIST_CFG_SPEC;
impl crate::RegisterSpec for GMAC1_PAD_BIST_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gmac1_pad_bist_cfg::W`](W) writer structure"]
impl crate::Writable for GMAC1_PAD_BIST_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC1_PAD_BIST_CFG to value 0"]
impl crate::Resettable for GMAC1_PAD_BIST_CFG_SPEC {}
