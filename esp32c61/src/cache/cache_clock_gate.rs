#[doc = "Register `CACHE_CLOCK_GATE` reader"]
pub type R = crate::R<CACHE_CLOCK_GATE_SPEC>;
#[doc = "Register `CACHE_CLOCK_GATE` writer"]
pub type W = crate::W<CACHE_CLOCK_GATE_SPEC>;
#[doc = "Field `CACHE_CLK_EN` reader - The bit is used to enable clock gate when access all registers in this module."]
pub type CACHE_CLK_EN_R = crate::BitReader;
#[doc = "Field `CACHE_CLK_EN` writer - The bit is used to enable clock gate when access all registers in this module."]
pub type CACHE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable clock gate when access all registers in this module."]
    #[inline(always)]
    pub fn cache_clk_en(&self) -> CACHE_CLK_EN_R {
        CACHE_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CLOCK_GATE")
            .field("cache_clk_en", &self.cache_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable clock gate when access all registers in this module."]
    #[inline(always)]
    pub fn cache_clk_en(&mut self) -> CACHE_CLK_EN_W<CACHE_CLOCK_GATE_SPEC> {
        CACHE_CLK_EN_W::new(self, 0)
    }
}
#[doc = "Clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CACHE_CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_clock_gate::R`](R) reader structure"]
impl crate::Readable for CACHE_CLOCK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_clock_gate::W`](W) writer structure"]
impl crate::Writable for CACHE_CLOCK_GATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_CLOCK_GATE to value 0"]
impl crate::Resettable for CACHE_CLOCK_GATE_SPEC {}
