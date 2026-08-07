#[doc = "Register `MEM_CLK_FORCE_ON` reader"]
pub type R = crate::R<MEM_CLK_FORCE_ON_SPEC>;
#[doc = "Register `MEM_CLK_FORCE_ON` writer"]
pub type W = crate::W<MEM_CLK_FORCE_ON_SPEC>;
#[doc = "Field `ICACHE0_MEM_CLK_FO` reader - reserved"]
pub type ICACHE0_MEM_CLK_FO_R = crate::BitReader;
#[doc = "Field `ICACHE0_MEM_CLK_FO` writer - reserved"]
pub type ICACHE0_MEM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_MEM_CLK_FO` reader - reserved"]
pub type ICACHE1_MEM_CLK_FO_R = crate::BitReader;
#[doc = "Field `ICACHE1_MEM_CLK_FO` writer - reserved"]
pub type ICACHE1_MEM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_MEM_CLK_FO` reader - reserved"]
pub type DCACHE_MEM_CLK_FO_R = crate::BitReader;
#[doc = "Field `DCACHE_MEM_CLK_FO` writer - reserved"]
pub type DCACHE_MEM_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn icache0_mem_clk_fo(&self) -> ICACHE0_MEM_CLK_FO_R {
        ICACHE0_MEM_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn icache1_mem_clk_fo(&self) -> ICACHE1_MEM_CLK_FO_R {
        ICACHE1_MEM_CLK_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn dcache_mem_clk_fo(&self) -> DCACHE_MEM_CLK_FO_R {
        DCACHE_MEM_CLK_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CLK_FORCE_ON")
            .field("icache0_mem_clk_fo", &self.icache0_mem_clk_fo())
            .field("icache1_mem_clk_fo", &self.icache1_mem_clk_fo())
            .field("dcache_mem_clk_fo", &self.dcache_mem_clk_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn icache0_mem_clk_fo(&mut self) -> ICACHE0_MEM_CLK_FO_W<'_, MEM_CLK_FORCE_ON_SPEC> {
        ICACHE0_MEM_CLK_FO_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn icache1_mem_clk_fo(&mut self) -> ICACHE1_MEM_CLK_FO_W<'_, MEM_CLK_FORCE_ON_SPEC> {
        ICACHE1_MEM_CLK_FO_W::new(self, 1)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn dcache_mem_clk_fo(&mut self) -> DCACHE_MEM_CLK_FO_W<'_, MEM_CLK_FORCE_ON_SPEC> {
        DCACHE_MEM_CLK_FO_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_clk_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_clk_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CLK_FORCE_ON_SPEC;
impl crate::RegisterSpec for MEM_CLK_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clk_force_on::R`](R) reader structure"]
impl crate::Readable for MEM_CLK_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_clk_force_on::W`](W) writer structure"]
impl crate::Writable for MEM_CLK_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CLK_FORCE_ON to value 0"]
impl crate::Resettable for MEM_CLK_FORCE_ON_SPEC {}
