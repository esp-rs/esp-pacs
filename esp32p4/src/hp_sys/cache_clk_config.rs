///Register `CACHE_CLK_CONFIG` reader
pub type R = crate::R<CACHE_CLK_CONFIG_SPEC>;
///Register `CACHE_CLK_CONFIG` writer
pub type W = crate::W<CACHE_CLK_CONFIG_SPEC>;
///Field `REG_L2_CACHE_CLK_ON` reader - l2 cahce clk enable
pub type REG_L2_CACHE_CLK_ON_R = crate::BitReader;
///Field `REG_L2_CACHE_CLK_ON` writer - l2 cahce clk enable
pub type REG_L2_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L1_D_CACHE_CLK_ON` reader - l1 dcahce clk enable
pub type REG_L1_D_CACHE_CLK_ON_R = crate::BitReader;
///Field `REG_L1_D_CACHE_CLK_ON` writer - l1 dcahce clk enable
pub type REG_L1_D_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L1_I1_CACHE_CLK_ON` reader - l1 icahce1 clk enable
pub type REG_L1_I1_CACHE_CLK_ON_R = crate::BitReader;
///Field `REG_L1_I1_CACHE_CLK_ON` writer - l1 icahce1 clk enable
pub type REG_L1_I1_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L1_I0_CACHE_CLK_ON` reader - l1 icahce0 clk enable
pub type REG_L1_I0_CACHE_CLK_ON_R = crate::BitReader;
///Field `REG_L1_I0_CACHE_CLK_ON` writer - l1 icahce0 clk enable
pub type REG_L1_I0_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - l2 cahce clk enable
    #[inline(always)]
    pub fn reg_l2_cache_clk_on(&self) -> REG_L2_CACHE_CLK_ON_R {
        REG_L2_CACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - l1 dcahce clk enable
    #[inline(always)]
    pub fn reg_l1_d_cache_clk_on(&self) -> REG_L1_D_CACHE_CLK_ON_R {
        REG_L1_D_CACHE_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - l1 icahce1 clk enable
    #[inline(always)]
    pub fn reg_l1_i1_cache_clk_on(&self) -> REG_L1_I1_CACHE_CLK_ON_R {
        REG_L1_I1_CACHE_CLK_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - l1 icahce0 clk enable
    #[inline(always)]
    pub fn reg_l1_i0_cache_clk_on(&self) -> REG_L1_I0_CACHE_CLK_ON_R {
        REG_L1_I0_CACHE_CLK_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CLK_CONFIG")
            .field("reg_l2_cache_clk_on", &self.reg_l2_cache_clk_on())
            .field("reg_l1_d_cache_clk_on", &self.reg_l1_d_cache_clk_on())
            .field("reg_l1_i1_cache_clk_on", &self.reg_l1_i1_cache_clk_on())
            .field("reg_l1_i0_cache_clk_on", &self.reg_l1_i0_cache_clk_on())
            .finish()
    }
}
impl W {
    ///Bit 0 - l2 cahce clk enable
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_cache_clk_on(&mut self) -> REG_L2_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L2_CACHE_CLK_ON_W::new(self, 0)
    }
    ///Bit 1 - l1 dcahce clk enable
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_d_cache_clk_on(&mut self) -> REG_L1_D_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L1_D_CACHE_CLK_ON_W::new(self, 1)
    }
    ///Bit 4 - l1 icahce1 clk enable
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i1_cache_clk_on(&mut self) -> REG_L1_I1_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L1_I1_CACHE_CLK_ON_W::new(self, 4)
    }
    ///Bit 5 - l1 icahce0 clk enable
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i0_cache_clk_on(&mut self) -> REG_L1_I0_CACHE_CLK_ON_W<CACHE_CLK_CONFIG_SPEC> {
        REG_L1_I0_CACHE_CLK_ON_W::new(self, 5)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`cache_clk_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_clk_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_CLK_CONFIG_SPEC;
impl crate::RegisterSpec for CACHE_CLK_CONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_clk_config::R`](R) reader structure
impl crate::Readable for CACHE_CLK_CONFIG_SPEC {}
///`write(|w| ..)` method takes [`cache_clk_config::W`](W) writer structure
impl crate::Writable for CACHE_CLK_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_CLK_CONFIG to value 0x33
impl crate::Resettable for CACHE_CLK_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x33;
}
