#[doc = "Register `EXT_LDO_P1_0P3A` reader"]
pub type R = crate::R<EXT_LDO_P1_0P3A_SPEC>;
#[doc = "Register `EXT_LDO_P1_0P3A` writer"]
pub type W = crate::W<EXT_LDO_P1_0P3A_SPEC>;
#[doc = "Field `0P3A_CNT_CLR_1` writer - PMU_0P3A_CNT_CLR_1"]
pub type _0P3A_CNT_CLR_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0P3A_FORCE_TIEH_SEL_1` reader - PMU_0P3A_FORCE_TIEH_SEL_1"]
pub type _0P3A_FORCE_TIEH_SEL_1_R = crate::BitReader;
#[doc = "Field `0P3A_FORCE_TIEH_SEL_1` writer - PMU_0P3A_FORCE_TIEH_SEL_1"]
pub type _0P3A_FORCE_TIEH_SEL_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0P3A_XPD_1` reader - PMU_0P3A_XPD_1"]
pub type _0P3A_XPD_1_R = crate::BitReader;
#[doc = "Field `0P3A_XPD_1` writer - PMU_0P3A_XPD_1"]
pub type _0P3A_XPD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0P3A_TIEH_SEL_1` reader - PMU_0P3A_TIEH_SEL_1"]
pub type _0P3A_TIEH_SEL_1_R = crate::FieldReader;
#[doc = "Field `0P3A_TIEH_SEL_1` writer - PMU_0P3A_TIEH_SEL_1"]
pub type _0P3A_TIEH_SEL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `0P3A_TIEH_POS_EN_1` reader - PMU_0P3A_TIEH_POS_EN_1"]
pub type _0P3A_TIEH_POS_EN_1_R = crate::BitReader;
#[doc = "Field `0P3A_TIEH_POS_EN_1` writer - PMU_0P3A_TIEH_POS_EN_1"]
pub type _0P3A_TIEH_POS_EN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0P3A_TIEH_NEG_EN_1` reader - PMU_0P3A_TIEH_NEG_EN_1"]
pub type _0P3A_TIEH_NEG_EN_1_R = crate::BitReader;
#[doc = "Field `0P3A_TIEH_NEG_EN_1` writer - PMU_0P3A_TIEH_NEG_EN_1"]
pub type _0P3A_TIEH_NEG_EN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0P3A_TIEH_1` reader - PMU_0P3A_TIEH_1"]
pub type _0P3A_TIEH_1_R = crate::BitReader;
#[doc = "Field `0P3A_TIEH_1` writer - PMU_0P3A_TIEH_1"]
pub type _0P3A_TIEH_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0P3A_TARGET1_1` reader - PMU_0P3A_TARGET1_1"]
pub type _0P3A_TARGET1_1_R = crate::FieldReader;
#[doc = "Field `0P3A_TARGET1_1` writer - PMU_0P3A_TARGET1_1"]
pub type _0P3A_TARGET1_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `0P3A_TARGET0_1` reader - PMU_0P3A_TARGET0_1"]
pub type _0P3A_TARGET0_1_R = crate::FieldReader;
#[doc = "Field `0P3A_TARGET0_1` writer - PMU_0P3A_TARGET0_1"]
pub type _0P3A_TARGET0_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `0P3A_LDO_CNT_PRESCALER_SEL_1` reader - PMU_0P3A_LDO_CNT_PRESCALER_SEL_1"]
pub type _0P3A_LDO_CNT_PRESCALER_SEL_1_R = crate::BitReader;
#[doc = "Field `0P3A_LDO_CNT_PRESCALER_SEL_1` writer - PMU_0P3A_LDO_CNT_PRESCALER_SEL_1"]
pub type _0P3A_LDO_CNT_PRESCALER_SEL_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - PMU_0P3A_FORCE_TIEH_SEL_1"]
    #[inline(always)]
    pub fn _0p3a_force_tieh_sel_1(&self) -> _0P3A_FORCE_TIEH_SEL_1_R {
        _0P3A_FORCE_TIEH_SEL_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PMU_0P3A_XPD_1"]
    #[inline(always)]
    pub fn _0p3a_xpd_1(&self) -> _0P3A_XPD_1_R {
        _0P3A_XPD_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - PMU_0P3A_TIEH_SEL_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_sel_1(&self) -> _0P3A_TIEH_SEL_1_R {
        _0P3A_TIEH_SEL_1_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - PMU_0P3A_TIEH_POS_EN_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_pos_en_1(&self) -> _0P3A_TIEH_POS_EN_1_R {
        _0P3A_TIEH_POS_EN_1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PMU_0P3A_TIEH_NEG_EN_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_neg_en_1(&self) -> _0P3A_TIEH_NEG_EN_1_R {
        _0P3A_TIEH_NEG_EN_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMU_0P3A_TIEH_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_1(&self) -> _0P3A_TIEH_1_R {
        _0P3A_TIEH_1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:22 - PMU_0P3A_TARGET1_1"]
    #[inline(always)]
    pub fn _0p3a_target1_1(&self) -> _0P3A_TARGET1_1_R {
        _0P3A_TARGET1_1_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - PMU_0P3A_TARGET0_1"]
    #[inline(always)]
    pub fn _0p3a_target0_1(&self) -> _0P3A_TARGET0_1_R {
        _0P3A_TARGET0_1_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - PMU_0P3A_LDO_CNT_PRESCALER_SEL_1"]
    #[inline(always)]
    pub fn _0p3a_ldo_cnt_prescaler_sel_1(&self) -> _0P3A_LDO_CNT_PRESCALER_SEL_1_R {
        _0P3A_LDO_CNT_PRESCALER_SEL_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_P1_0P3A")
            .field("_0p3a_force_tieh_sel_1", &self._0p3a_force_tieh_sel_1())
            .field("_0p3a_xpd_1", &self._0p3a_xpd_1())
            .field("_0p3a_tieh_sel_1", &self._0p3a_tieh_sel_1())
            .field("_0p3a_tieh_pos_en_1", &self._0p3a_tieh_pos_en_1())
            .field("_0p3a_tieh_neg_en_1", &self._0p3a_tieh_neg_en_1())
            .field("_0p3a_tieh_1", &self._0p3a_tieh_1())
            .field("_0p3a_target1_1", &self._0p3a_target1_1())
            .field("_0p3a_target0_1", &self._0p3a_target0_1())
            .field(
                "_0p3a_ldo_cnt_prescaler_sel_1",
                &self._0p3a_ldo_cnt_prescaler_sel_1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - PMU_0P3A_CNT_CLR_1"]
    #[inline(always)]
    pub fn _0p3a_cnt_clr_1(&mut self) -> _0P3A_CNT_CLR_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_CNT_CLR_1_W::new(self, 6)
    }
    #[doc = "Bit 7 - PMU_0P3A_FORCE_TIEH_SEL_1"]
    #[inline(always)]
    pub fn _0p3a_force_tieh_sel_1(&mut self) -> _0P3A_FORCE_TIEH_SEL_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_FORCE_TIEH_SEL_1_W::new(self, 7)
    }
    #[doc = "Bit 8 - PMU_0P3A_XPD_1"]
    #[inline(always)]
    pub fn _0p3a_xpd_1(&mut self) -> _0P3A_XPD_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_XPD_1_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - PMU_0P3A_TIEH_SEL_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_sel_1(&mut self) -> _0P3A_TIEH_SEL_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_TIEH_SEL_1_W::new(self, 9)
    }
    #[doc = "Bit 12 - PMU_0P3A_TIEH_POS_EN_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_pos_en_1(&mut self) -> _0P3A_TIEH_POS_EN_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_TIEH_POS_EN_1_W::new(self, 12)
    }
    #[doc = "Bit 13 - PMU_0P3A_TIEH_NEG_EN_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_neg_en_1(&mut self) -> _0P3A_TIEH_NEG_EN_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_TIEH_NEG_EN_1_W::new(self, 13)
    }
    #[doc = "Bit 14 - PMU_0P3A_TIEH_1"]
    #[inline(always)]
    pub fn _0p3a_tieh_1(&mut self) -> _0P3A_TIEH_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_TIEH_1_W::new(self, 14)
    }
    #[doc = "Bits 15:22 - PMU_0P3A_TARGET1_1"]
    #[inline(always)]
    pub fn _0p3a_target1_1(&mut self) -> _0P3A_TARGET1_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_TARGET1_1_W::new(self, 15)
    }
    #[doc = "Bits 23:30 - PMU_0P3A_TARGET0_1"]
    #[inline(always)]
    pub fn _0p3a_target0_1(&mut self) -> _0P3A_TARGET0_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_TARGET0_1_W::new(self, 23)
    }
    #[doc = "Bit 31 - PMU_0P3A_LDO_CNT_PRESCALER_SEL_1"]
    #[inline(always)]
    pub fn _0p3a_ldo_cnt_prescaler_sel_1(
        &mut self,
    ) -> _0P3A_LDO_CNT_PRESCALER_SEL_1_W<'_, EXT_LDO_P1_0P3A_SPEC> {
        _0P3A_LDO_CNT_PRESCALER_SEL_1_W::new(self, 31)
    }
}
#[doc = "PMU_EXT_LDO_P1_0P3A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p3a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_LDO_P1_0P3A_SPEC;
impl crate::RegisterSpec for EXT_LDO_P1_0P3A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_p1_0p3a::R`](R) reader structure"]
impl crate::Readable for EXT_LDO_P1_0P3A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_p1_0p3a::W`](W) writer structure"]
impl crate::Writable for EXT_LDO_P1_0P3A_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_LDO_P1_0P3A to value 0"]
impl crate::Resettable for EXT_LDO_P1_0P3A_SPEC {}
