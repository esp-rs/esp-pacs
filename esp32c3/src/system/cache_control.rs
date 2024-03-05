#[doc = "Register `CACHE_CONTROL` reader"]
pub type R = crate::R<CACHE_CONTROL_SPEC>;
#[doc = "Register `CACHE_CONTROL` writer"]
pub type W = crate::W<CACHE_CONTROL_SPEC>;
#[doc = "Field `ICACHE_CLK_ON` reader - reg_icache_clk_on"]
pub type ICACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `ICACHE_CLK_ON` writer - reg_icache_clk_on"]
pub type ICACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_RESET` reader - reg_icache_reset"]
pub type ICACHE_RESET_R = crate::BitReader;
#[doc = "Field `ICACHE_RESET` writer - reg_icache_reset"]
pub type ICACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_CLK_ON` reader - reg_dcache_clk_on"]
pub type DCACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `DCACHE_CLK_ON` writer - reg_dcache_clk_on"]
pub type DCACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_RESET` reader - reg_dcache_reset"]
pub type DCACHE_RESET_R = crate::BitReader;
#[doc = "Field `DCACHE_RESET` writer - reg_dcache_reset"]
pub type DCACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_icache_clk_on"]
    #[inline(always)]
    pub fn icache_clk_on(&self) -> ICACHE_CLK_ON_R {
        ICACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_icache_reset"]
    #[inline(always)]
    pub fn icache_reset(&self) -> ICACHE_RESET_R {
        ICACHE_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_dcache_clk_on"]
    #[inline(always)]
    pub fn dcache_clk_on(&self) -> DCACHE_CLK_ON_R {
        DCACHE_CLK_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_dcache_reset"]
    #[inline(always)]
    pub fn dcache_reset(&self) -> DCACHE_RESET_R {
        DCACHE_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONTROL")
            .field(
                "icache_clk_on",
                &format_args!("{}", self.icache_clk_on().bit()),
            )
            .field(
                "icache_reset",
                &format_args!("{}", self.icache_reset().bit()),
            )
            .field(
                "dcache_clk_on",
                &format_args!("{}", self.dcache_clk_on().bit()),
            )
            .field(
                "dcache_reset",
                &format_args!("{}", self.dcache_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_icache_clk_on"]
    #[inline(always)]
    #[must_use]
    pub fn icache_clk_on(&mut self) -> ICACHE_CLK_ON_W<CACHE_CONTROL_SPEC> {
        ICACHE_CLK_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_icache_reset"]
    #[inline(always)]
    #[must_use]
    pub fn icache_reset(&mut self) -> ICACHE_RESET_W<CACHE_CONTROL_SPEC> {
        ICACHE_RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_dcache_clk_on"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_clk_on(&mut self) -> DCACHE_CLK_ON_W<CACHE_CONTROL_SPEC> {
        DCACHE_CLK_ON_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_dcache_reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_reset(&mut self) -> DCACHE_RESET_W<CACHE_CONTROL_SPEC> {
        DCACHE_RESET_W::new(self, 3)
    }
}
#[doc = "cache control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CACHE_CONTROL to value 0x05"]
impl crate::Resettable for CACHE_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
