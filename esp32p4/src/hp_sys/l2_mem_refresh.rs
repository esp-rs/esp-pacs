#[doc = "Register `L2_MEM_REFRESH` reader"]
pub type R = crate::R<L2_MEM_REFRESH_SPEC>;
#[doc = "Register `L2_MEM_REFRESH` writer"]
pub type W = crate::W<L2_MEM_REFRESH_SPEC>;
#[doc = "Field `REG_L2_MEM_UNIT0_REFERSH_EN` reader - NA"]
pub type REG_L2_MEM_UNIT0_REFERSH_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT0_REFERSH_EN` writer - NA"]
pub type REG_L2_MEM_UNIT0_REFERSH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_UNIT1_REFERSH_EN` reader - NA"]
pub type REG_L2_MEM_UNIT1_REFERSH_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT1_REFERSH_EN` writer - NA"]
pub type REG_L2_MEM_UNIT1_REFERSH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_UNIT2_REFERSH_EN` reader - NA"]
pub type REG_L2_MEM_UNIT2_REFERSH_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT2_REFERSH_EN` writer - NA"]
pub type REG_L2_MEM_UNIT2_REFERSH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_UNIT3_REFERSH_EN` reader - NA"]
pub type REG_L2_MEM_UNIT3_REFERSH_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT3_REFERSH_EN` writer - NA"]
pub type REG_L2_MEM_UNIT3_REFERSH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_UNIT4_REFERSH_EN` reader - NA"]
pub type REG_L2_MEM_UNIT4_REFERSH_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT4_REFERSH_EN` writer - NA"]
pub type REG_L2_MEM_UNIT4_REFERSH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_UNIT5_REFERSH_EN` reader - NA"]
pub type REG_L2_MEM_UNIT5_REFERSH_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT5_REFERSH_EN` writer - NA"]
pub type REG_L2_MEM_UNIT5_REFERSH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_REFERSH_CNT_RESET` reader - Set 1 to reset l2mem_refresh_cnt"]
pub type REG_L2_MEM_REFERSH_CNT_RESET_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_REFERSH_CNT_RESET` writer - Set 1 to reset l2mem_refresh_cnt"]
pub type REG_L2_MEM_REFERSH_CNT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_UNIT0_REFRESH_DONE` reader - NA"]
pub type REG_L2_MEM_UNIT0_REFRESH_DONE_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT1_REFRESH_DONE` reader - NA"]
pub type REG_L2_MEM_UNIT1_REFRESH_DONE_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT2_REFRESH_DONE` reader - NA"]
pub type REG_L2_MEM_UNIT2_REFRESH_DONE_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT3_REFRESH_DONE` reader - NA"]
pub type REG_L2_MEM_UNIT3_REFRESH_DONE_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT4_REFRESH_DONE` reader - NA"]
pub type REG_L2_MEM_UNIT4_REFRESH_DONE_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_UNIT5_REFRESH_DONE` reader - NA"]
pub type REG_L2_MEM_UNIT5_REFRESH_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit0_refersh_en(&self) -> REG_L2_MEM_UNIT0_REFERSH_EN_R {
        REG_L2_MEM_UNIT0_REFERSH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit1_refersh_en(&self) -> REG_L2_MEM_UNIT1_REFERSH_EN_R {
        REG_L2_MEM_UNIT1_REFERSH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit2_refersh_en(&self) -> REG_L2_MEM_UNIT2_REFERSH_EN_R {
        REG_L2_MEM_UNIT2_REFERSH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit3_refersh_en(&self) -> REG_L2_MEM_UNIT3_REFERSH_EN_R {
        REG_L2_MEM_UNIT3_REFERSH_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit4_refersh_en(&self) -> REG_L2_MEM_UNIT4_REFERSH_EN_R {
        REG_L2_MEM_UNIT4_REFERSH_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit5_refersh_en(&self) -> REG_L2_MEM_UNIT5_REFERSH_EN_R {
        REG_L2_MEM_UNIT5_REFERSH_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to reset l2mem_refresh_cnt"]
    #[inline(always)]
    pub fn reg_l2_mem_refersh_cnt_reset(&self) -> REG_L2_MEM_REFERSH_CNT_RESET_R {
        REG_L2_MEM_REFERSH_CNT_RESET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit0_refresh_done(&self) -> REG_L2_MEM_UNIT0_REFRESH_DONE_R {
        REG_L2_MEM_UNIT0_REFRESH_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit1_refresh_done(&self) -> REG_L2_MEM_UNIT1_REFRESH_DONE_R {
        REG_L2_MEM_UNIT1_REFRESH_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit2_refresh_done(&self) -> REG_L2_MEM_UNIT2_REFRESH_DONE_R {
        REG_L2_MEM_UNIT2_REFRESH_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit3_refresh_done(&self) -> REG_L2_MEM_UNIT3_REFRESH_DONE_R {
        REG_L2_MEM_UNIT3_REFRESH_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit4_refresh_done(&self) -> REG_L2_MEM_UNIT4_REFRESH_DONE_R {
        REG_L2_MEM_UNIT4_REFRESH_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_unit5_refresh_done(&self) -> REG_L2_MEM_UNIT5_REFRESH_DONE_R {
        REG_L2_MEM_UNIT5_REFRESH_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_REFRESH")
            .field(
                "reg_l2_mem_unit0_refersh_en",
                &self.reg_l2_mem_unit0_refersh_en(),
            )
            .field(
                "reg_l2_mem_unit1_refersh_en",
                &self.reg_l2_mem_unit1_refersh_en(),
            )
            .field(
                "reg_l2_mem_unit2_refersh_en",
                &self.reg_l2_mem_unit2_refersh_en(),
            )
            .field(
                "reg_l2_mem_unit3_refersh_en",
                &self.reg_l2_mem_unit3_refersh_en(),
            )
            .field(
                "reg_l2_mem_unit4_refersh_en",
                &self.reg_l2_mem_unit4_refersh_en(),
            )
            .field(
                "reg_l2_mem_unit5_refersh_en",
                &self.reg_l2_mem_unit5_refersh_en(),
            )
            .field(
                "reg_l2_mem_refersh_cnt_reset",
                &self.reg_l2_mem_refersh_cnt_reset(),
            )
            .field(
                "reg_l2_mem_unit0_refresh_done",
                &self.reg_l2_mem_unit0_refresh_done(),
            )
            .field(
                "reg_l2_mem_unit1_refresh_done",
                &self.reg_l2_mem_unit1_refresh_done(),
            )
            .field(
                "reg_l2_mem_unit2_refresh_done",
                &self.reg_l2_mem_unit2_refresh_done(),
            )
            .field(
                "reg_l2_mem_unit3_refresh_done",
                &self.reg_l2_mem_unit3_refresh_done(),
            )
            .field(
                "reg_l2_mem_unit4_refresh_done",
                &self.reg_l2_mem_unit4_refresh_done(),
            )
            .field(
                "reg_l2_mem_unit5_refresh_done",
                &self.reg_l2_mem_unit5_refresh_done(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_unit0_refersh_en(
        &mut self,
    ) -> REG_L2_MEM_UNIT0_REFERSH_EN_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_UNIT0_REFERSH_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_unit1_refersh_en(
        &mut self,
    ) -> REG_L2_MEM_UNIT1_REFERSH_EN_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_UNIT1_REFERSH_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_unit2_refersh_en(
        &mut self,
    ) -> REG_L2_MEM_UNIT2_REFERSH_EN_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_UNIT2_REFERSH_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_unit3_refersh_en(
        &mut self,
    ) -> REG_L2_MEM_UNIT3_REFERSH_EN_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_UNIT3_REFERSH_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_unit4_refersh_en(
        &mut self,
    ) -> REG_L2_MEM_UNIT4_REFERSH_EN_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_UNIT4_REFERSH_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_unit5_refersh_en(
        &mut self,
    ) -> REG_L2_MEM_UNIT5_REFERSH_EN_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_UNIT5_REFERSH_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 1 to reset l2mem_refresh_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_refersh_cnt_reset(
        &mut self,
    ) -> REG_L2_MEM_REFERSH_CNT_RESET_W<L2_MEM_REFRESH_SPEC> {
        REG_L2_MEM_REFERSH_CNT_RESET_W::new(self, 6)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_refresh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_refresh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_REFRESH_SPEC;
impl crate::RegisterSpec for L2_MEM_REFRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_refresh::R`](R) reader structure"]
impl crate::Readable for L2_MEM_REFRESH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_refresh::W`](W) writer structure"]
impl crate::Writable for L2_MEM_REFRESH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_MEM_REFRESH to value 0x40"]
impl crate::Resettable for L2_MEM_REFRESH_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
