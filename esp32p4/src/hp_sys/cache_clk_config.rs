#[doc = "Register `CACHE_CLK_CONFIG` reader"]
pub type R = crate::R<CACHE_CLK_CONFIG_SPEC>;
#[doc = "Register `CACHE_CLK_CONFIG` writer"]
pub type W = crate::W<CACHE_CLK_CONFIG_SPEC>;
#[doc = "Field `REG_L2_CACHE_CLK_ON` reader - l2 cahce clk enable"]
pub type REG_L2_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `REG_L2_CACHE_CLK_ON` writer - l2 cahce clk enable"]
pub type REG_L2_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_D_CACHE_CLK_ON` reader - l1 dcahce clk enable"]
pub type REG_L1_D_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `REG_L1_D_CACHE_CLK_ON` writer - l1 dcahce clk enable"]
pub type REG_L1_D_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_I1_CACHE_CLK_ON` reader - l1 icahce1 clk enable"]
pub type REG_L1_I1_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `REG_L1_I1_CACHE_CLK_ON` writer - l1 icahce1 clk enable"]
pub type REG_L1_I1_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_I0_CACHE_CLK_ON` reader - l1 icahce0 clk enable"]
pub type REG_L1_I0_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `REG_L1_I0_CACHE_CLK_ON` writer - l1 icahce0 clk enable"]
pub type REG_L1_I0_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - l2 cahce clk enable"]
    #[inline(always)]
    pub fn reg_l2_cache_clk_on(&self) -> REG_L2_CACHE_CLK_ON_R {
        REG_L2_CACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - l1 dcahce clk enable"]
    #[inline(always)]
    pub fn reg_l1_d_cache_clk_on(&self) -> REG_L1_D_CACHE_CLK_ON_R {
        REG_L1_D_CACHE_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - l1 icahce1 clk enable"]
    #[inline(always)]
    pub fn reg_l1_i1_cache_clk_on(&self) -> REG_L1_I1_CACHE_CLK_ON_R {
        REG_L1_I1_CACHE_CLK_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - l1 icahce0 clk enable"]
    #[inline(always)]
    pub fn reg_l1_i0_cache_clk_on(&self) -> REG_L1_I0_CACHE_CLK_ON_R {
        REG_L1_I0_CACHE_CLK_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CLK_CONFIG")
            .field(
                "reg_l2_cache_clk_on",
                &format_args!("{}", self.reg_l2_cache_clk_on().bit()),
            )
            .field(
                "reg_l1_d_cache_clk_on",
                &format_args!("{}", self.reg_l1_d_cache_clk_on().bit()),
            )
            .field(
                "reg_l1_i1_cache_clk_on",
                &format_args!("{}", self.reg_l1_i1_cache_clk_on().bit()),
            )
            .field(
                "reg_l1_i0_cache_clk_on",
                &format_args!("{}", self.reg_l1_i0_cache_clk_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_CLK_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - l2 cahce clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_cache_clk_on(&mut self) -> REG_L2_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L2_CACHE_CLK_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - l1 dcahce clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_d_cache_clk_on(&mut self) -> REG_L1_D_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L1_D_CACHE_CLK_ON_W::new(self, 1)
    }
    #[doc = "Bit 4 - l1 icahce1 clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i1_cache_clk_on(&mut self) -> REG_L1_I1_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L1_I1_CACHE_CLK_ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - l1 icahce0 clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i0_cache_clk_on(&mut self) -> REG_L1_I0_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L1_I0_CACHE_CLK_ON_W::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_clk_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_clk_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CLK_CONFIG_SPEC;
impl crate::RegisterSpec for CACHE_CLK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_clk_config::R`](R) reader structure"]
impl crate::Readable for CACHE_CLK_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_clk_config::W`](W) writer structure"]
impl crate::Writable for CACHE_CLK_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_CLK_CONFIG to value 0x33"]
impl crate::Resettable for CACHE_CLK_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x33;
}
