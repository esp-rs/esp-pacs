#[doc = "Register `CACHE_MEM_CLK_FO` reader"]
pub type R = crate::R<CACHE_MEM_CLK_FO_SPEC>;
#[doc = "Register `CACHE_MEM_CLK_FO` writer"]
pub type W = crate::W<CACHE_MEM_CLK_FO_SPEC>;
#[doc = "Field `HP_SYSTEM_CACHE_MEM_CLK_FO` reader - debug"]
pub type HP_SYSTEM_CACHE_MEM_CLK_FO_R = crate::BitReader;
#[doc = "Field `HP_SYSTEM_CACHE_MEM_CLK_FO` writer - debug"]
pub type HP_SYSTEM_CACHE_MEM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - debug"]
    #[inline(always)]
    pub fn hp_system_cache_mem_clk_fo(&self) -> HP_SYSTEM_CACHE_MEM_CLK_FO_R {
        HP_SYSTEM_CACHE_MEM_CLK_FO_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MEM_CLK_FO")
            .field(
                "hp_system_cache_mem_clk_fo",
                &self.hp_system_cache_mem_clk_fo(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - debug"]
    #[inline(always)]
    pub fn hp_system_cache_mem_clk_fo(
        &mut self,
    ) -> HP_SYSTEM_CACHE_MEM_CLK_FO_W<'_, CACHE_MEM_CLK_FO_SPEC> {
        HP_SYSTEM_CACHE_MEM_CLK_FO_W::new(self, 0)
    }
}
#[doc = "debug\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mem_clk_fo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_mem_clk_fo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MEM_CLK_FO_SPEC;
impl crate::RegisterSpec for CACHE_MEM_CLK_FO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mem_clk_fo::R`](R) reader structure"]
impl crate::Readable for CACHE_MEM_CLK_FO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_mem_clk_fo::W`](W) writer structure"]
impl crate::Writable for CACHE_MEM_CLK_FO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_MEM_CLK_FO to value 0"]
impl crate::Resettable for CACHE_MEM_CLK_FO_SPEC {}
