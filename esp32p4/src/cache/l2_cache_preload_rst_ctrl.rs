#[doc = "Register `L2_CACHE_PRELOAD_RST_CTRL` reader"]
pub type R = crate::R<L2_CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_PRELOAD_RST_CTRL` writer"]
pub type W = crate::W<L2_CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_PLD_RST` reader - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L2_CACHE_PLD_RST_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PLD_RST` writer - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L2_CACHE_PLD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l2_cache_pld_rst(&self) -> L2_CACHE_PLD_RST_R {
        L2_CACHE_PLD_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PRELOAD_RST_CTRL")
            .field("l2_cache_pld_rst", &self.l2_cache_pld_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_pld_rst(&mut self) -> L2_CACHE_PLD_RST_W<L2_CACHE_PRELOAD_RST_CTRL_SPEC> {
        L2_CACHE_PLD_RST_W::new(self, 5)
    }
}
#[doc = "Cache Preload Reset control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_preload_rst_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_preload_rst_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_PRELOAD_RST_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_preload_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOAD_RST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_preload_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOAD_RST_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
