///Register `LP_AONCLKRST_LP_CLK_PO_EN` reader
pub type R = crate::R<LP_AONCLKRST_LP_CLK_PO_EN_SPEC>;
///Register `LP_AONCLKRST_LP_CLK_PO_EN` writer
pub type W = crate::W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC>;
///Field `LP_AONCLKRST_CLK_CORE_EFUSE_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_CORE_EFUSE_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_CORE_EFUSE_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_CORE_EFUSE_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_LP_BUS_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_LP_BUS_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_LP_BUS_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_LP_BUS_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_AON_SLOW_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_AON_SLOW_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_AON_SLOW_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_AON_SLOW_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_AON_FAST_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_AON_FAST_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_AON_FAST_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_AON_FAST_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_SLOW_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_SLOW_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_SLOW_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_SLOW_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_FAST_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_FAST_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_FAST_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_FAST_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_FOSC_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_FOSC_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_FOSC_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_FOSC_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_RC32K_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_RC32K_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_RC32K_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_RC32K_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_SXTAL_OEN` reader - need_des
pub type LP_AONCLKRST_CLK_SXTAL_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_SXTAL_OEN` writer - need_des
pub type LP_AONCLKRST_CLK_SXTAL_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_AONCLKRST_CLK_SOSC_OEN` reader - 1'b1: probe sosc clk on 1'b0: probe sosc clk off
pub type LP_AONCLKRST_CLK_SOSC_OEN_R = crate::BitReader;
///Field `LP_AONCLKRST_CLK_SOSC_OEN` writer - 1'b1: probe sosc clk on 1'b0: probe sosc clk off
pub type LP_AONCLKRST_CLK_SOSC_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_core_efuse_oen(&self) -> LP_AONCLKRST_CLK_CORE_EFUSE_OEN_R {
        LP_AONCLKRST_CLK_CORE_EFUSE_OEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_lp_bus_oen(&self) -> LP_AONCLKRST_CLK_LP_BUS_OEN_R {
        LP_AONCLKRST_CLK_LP_BUS_OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_aon_slow_oen(&self) -> LP_AONCLKRST_CLK_AON_SLOW_OEN_R {
        LP_AONCLKRST_CLK_AON_SLOW_OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_aon_fast_oen(&self) -> LP_AONCLKRST_CLK_AON_FAST_OEN_R {
        LP_AONCLKRST_CLK_AON_FAST_OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_slow_oen(&self) -> LP_AONCLKRST_CLK_SLOW_OEN_R {
        LP_AONCLKRST_CLK_SLOW_OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_fast_oen(&self) -> LP_AONCLKRST_CLK_FAST_OEN_R {
        LP_AONCLKRST_CLK_FAST_OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_fosc_oen(&self) -> LP_AONCLKRST_CLK_FOSC_OEN_R {
        LP_AONCLKRST_CLK_FOSC_OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_rc32k_oen(&self) -> LP_AONCLKRST_CLK_RC32K_OEN_R {
        LP_AONCLKRST_CLK_RC32K_OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_sxtal_oen(&self) -> LP_AONCLKRST_CLK_SXTAL_OEN_R {
        LP_AONCLKRST_CLK_SXTAL_OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 1'b1: probe sosc clk on 1'b0: probe sosc clk off
    #[inline(always)]
    pub fn lp_aonclkrst_clk_sosc_oen(&self) -> LP_AONCLKRST_CLK_SOSC_OEN_R {
        LP_AONCLKRST_CLK_SOSC_OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_LP_CLK_PO_EN")
            .field(
                "lp_aonclkrst_clk_core_efuse_oen",
                &self.lp_aonclkrst_clk_core_efuse_oen(),
            )
            .field(
                "lp_aonclkrst_clk_lp_bus_oen",
                &self.lp_aonclkrst_clk_lp_bus_oen(),
            )
            .field(
                "lp_aonclkrst_clk_aon_slow_oen",
                &self.lp_aonclkrst_clk_aon_slow_oen(),
            )
            .field(
                "lp_aonclkrst_clk_aon_fast_oen",
                &self.lp_aonclkrst_clk_aon_fast_oen(),
            )
            .field(
                "lp_aonclkrst_clk_slow_oen",
                &self.lp_aonclkrst_clk_slow_oen(),
            )
            .field(
                "lp_aonclkrst_clk_fast_oen",
                &self.lp_aonclkrst_clk_fast_oen(),
            )
            .field(
                "lp_aonclkrst_clk_fosc_oen",
                &self.lp_aonclkrst_clk_fosc_oen(),
            )
            .field(
                "lp_aonclkrst_clk_rc32k_oen",
                &self.lp_aonclkrst_clk_rc32k_oen(),
            )
            .field(
                "lp_aonclkrst_clk_sxtal_oen",
                &self.lp_aonclkrst_clk_sxtal_oen(),
            )
            .field(
                "lp_aonclkrst_clk_sosc_oen",
                &self.lp_aonclkrst_clk_sosc_oen(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_core_efuse_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_CORE_EFUSE_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_CORE_EFUSE_OEN_W::new(self, 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_lp_bus_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_LP_BUS_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_LP_BUS_OEN_W::new(self, 1)
    }
    ///Bit 2 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_aon_slow_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_AON_SLOW_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_AON_SLOW_OEN_W::new(self, 2)
    }
    ///Bit 3 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_aon_fast_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_AON_FAST_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_AON_FAST_OEN_W::new(self, 3)
    }
    ///Bit 4 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_slow_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_SLOW_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_SLOW_OEN_W::new(self, 4)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_fast_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_FAST_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_FAST_OEN_W::new(self, 5)
    }
    ///Bit 6 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_fosc_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_FOSC_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_FOSC_OEN_W::new(self, 6)
    }
    ///Bit 7 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_rc32k_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_RC32K_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_RC32K_OEN_W::new(self, 7)
    }
    ///Bit 8 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_sxtal_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_SXTAL_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_SXTAL_OEN_W::new(self, 8)
    }
    ///Bit 9 - 1'b1: probe sosc clk on 1'b0: probe sosc clk off
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_clk_sosc_oen(
        &mut self,
    ) -> LP_AONCLKRST_CLK_SOSC_OEN_W<LP_AONCLKRST_LP_CLK_PO_EN_SPEC> {
        LP_AONCLKRST_CLK_SOSC_OEN_W::new(self, 9)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_clk_po_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_po_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_AONCLKRST_LP_CLK_PO_EN_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_LP_CLK_PO_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_aonclkrst_lp_clk_po_en::R`](R) reader structure
impl crate::Readable for LP_AONCLKRST_LP_CLK_PO_EN_SPEC {}
///`write(|w| ..)` method takes [`lp_aonclkrst_lp_clk_po_en::W`](W) writer structure
impl crate::Writable for LP_AONCLKRST_LP_CLK_PO_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_AONCLKRST_LP_CLK_PO_EN to value 0
impl crate::Resettable for LP_AONCLKRST_LP_CLK_PO_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
