#[doc = "Register `CACHE_CONTROL` reader"]
pub type R = crate::R<CACHE_CONTROL_SPEC>;
#[doc = "Register `CACHE_CONTROL` writer"]
pub type W = crate::W<CACHE_CONTROL_SPEC>;
#[doc = "Field `PRO_ICACHE_CLK_ON` reader - Set this bit to enable clock of i-cache."]
pub type PRO_ICACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_CLK_ON` writer - Set this bit to enable clock of i-cache."]
pub type PRO_ICACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DCACHE_CLK_ON` reader - Set this bit to enable clock of d-cache."]
pub type PRO_DCACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_CLK_ON` writer - Set this bit to enable clock of d-cache."]
pub type PRO_DCACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_RESET` reader - Set this bit to reset cache."]
pub type PRO_CACHE_RESET_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_RESET` writer - Set this bit to reset cache."]
pub type PRO_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable clock of i-cache."]
    #[inline(always)]
    pub fn pro_icache_clk_on(&self) -> PRO_ICACHE_CLK_ON_R {
        PRO_ICACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of d-cache."]
    #[inline(always)]
    pub fn pro_dcache_clk_on(&self) -> PRO_DCACHE_CLK_ON_R {
        PRO_DCACHE_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset cache."]
    #[inline(always)]
    pub fn pro_cache_reset(&self) -> PRO_CACHE_RESET_R {
        PRO_CACHE_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONTROL")
            .field("pro_icache_clk_on", &self.pro_icache_clk_on())
            .field("pro_dcache_clk_on", &self.pro_dcache_clk_on())
            .field("pro_cache_reset", &self.pro_cache_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable clock of i-cache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_clk_on(&mut self) -> PRO_ICACHE_CLK_ON_W<CACHE_CONTROL_SPEC> {
        PRO_ICACHE_CLK_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of d-cache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_clk_on(&mut self) -> PRO_DCACHE_CLK_ON_W<CACHE_CONTROL_SPEC> {
        PRO_DCACHE_CLK_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset cache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_reset(&mut self) -> PRO_CACHE_RESET_W<CACHE_CONTROL_SPEC> {
        PRO_CACHE_RESET_W::new(self, 2)
    }
}
#[doc = "Cache control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CONTROL_SPEC;
impl crate::RegisterSpec for CACHE_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_control::R`](R) reader structure"]
impl crate::Readable for CACHE_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_control::W`](W) writer structure"]
impl crate::Writable for CACHE_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_CONTROL to value 0x03"]
impl crate::Resettable for CACHE_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
