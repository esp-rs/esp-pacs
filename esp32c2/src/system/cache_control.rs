///Register `CACHE_CONTROL` reader
pub type R = crate::R<CACHE_CONTROL_SPEC>;
///Register `CACHE_CONTROL` writer
pub type W = crate::W<CACHE_CONTROL_SPEC>;
///Field `ICACHE_CLK_ON` reader - Set 1 to enable icache clock
pub type ICACHE_CLK_ON_R = crate::BitReader;
///Field `ICACHE_CLK_ON` writer - Set 1 to enable icache clock
pub type ICACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_RESET` reader - Set 1 to let icache reset
pub type ICACHE_RESET_R = crate::BitReader;
///Field `ICACHE_RESET` writer - Set 1 to let icache reset
pub type ICACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_CLK_ON` reader - Set 1 to enable dcache clock
pub type DCACHE_CLK_ON_R = crate::BitReader;
///Field `DCACHE_CLK_ON` writer - Set 1 to enable dcache clock
pub type DCACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCACHE_RESET` reader - Set 1 to let dcache reset
pub type DCACHE_RESET_R = crate::BitReader;
///Field `DCACHE_RESET` writer - Set 1 to let dcache reset
pub type DCACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable icache clock
    #[inline(always)]
    pub fn icache_clk_on(&self) -> ICACHE_CLK_ON_R {
        ICACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to let icache reset
    #[inline(always)]
    pub fn icache_reset(&self) -> ICACHE_RESET_R {
        ICACHE_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set 1 to enable dcache clock
    #[inline(always)]
    pub fn dcache_clk_on(&self) -> DCACHE_CLK_ON_R {
        DCACHE_CLK_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to let dcache reset
    #[inline(always)]
    pub fn dcache_reset(&self) -> DCACHE_RESET_R {
        DCACHE_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONTROL")
            .field("icache_clk_on", &self.icache_clk_on())
            .field("icache_reset", &self.icache_reset())
            .field("dcache_clk_on", &self.dcache_clk_on())
            .field("dcache_reset", &self.dcache_reset())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable icache clock
    #[inline(always)]
    #[must_use]
    pub fn icache_clk_on(&mut self) -> ICACHE_CLK_ON_W<CACHE_CONTROL_SPEC> {
        ICACHE_CLK_ON_W::new(self, 0)
    }
    ///Bit 1 - Set 1 to let icache reset
    #[inline(always)]
    #[must_use]
    pub fn icache_reset(&mut self) -> ICACHE_RESET_W<CACHE_CONTROL_SPEC> {
        ICACHE_RESET_W::new(self, 1)
    }
    ///Bit 2 - Set 1 to enable dcache clock
    #[inline(always)]
    #[must_use]
    pub fn dcache_clk_on(&mut self) -> DCACHE_CLK_ON_W<CACHE_CONTROL_SPEC> {
        DCACHE_CLK_ON_W::new(self, 2)
    }
    ///Bit 3 - Set 1 to let dcache reset
    #[inline(always)]
    #[must_use]
    pub fn dcache_reset(&mut self) -> DCACHE_RESET_W<CACHE_CONTROL_SPEC> {
        DCACHE_RESET_W::new(self, 3)
    }
}
/**cache control register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_CONTROL_SPEC;
impl crate::RegisterSpec for CACHE_CONTROL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_control::R`](R) reader structure
impl crate::Readable for CACHE_CONTROL_SPEC {}
///`write(|w| ..)` method takes [`cache_control::W`](W) writer structure
impl crate::Writable for CACHE_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_CONTROL to value 0x05
impl crate::Resettable for CACHE_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
