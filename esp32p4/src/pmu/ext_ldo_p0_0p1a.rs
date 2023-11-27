#[doc = "Register `EXT_LDO_P0_0P1A` reader"]
pub type R = crate::R<EXT_LDO_P0_0P1A_SPEC>;
#[doc = "Register `EXT_LDO_P0_0P1A` writer"]
pub type W = crate::W<EXT_LDO_P0_0P1A_SPEC>;
#[doc = "Field `_0P1A_FORCE_TIEH_SEL_0` reader - need_des"]
pub type _0P1A_FORCE_TIEH_SEL_0_R = crate::BitReader;
#[doc = "Field `_0P1A_FORCE_TIEH_SEL_0` writer - need_des"]
pub type _0P1A_FORCE_TIEH_SEL_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P1A_XPD_0` reader - need_des"]
pub type _0P1A_XPD_0_R = crate::BitReader;
#[doc = "Field `_0P1A_XPD_0` writer - need_des"]
pub type _0P1A_XPD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P1A_TIEH_SEL_0` reader - need_des"]
pub type _0P1A_TIEH_SEL_0_R = crate::FieldReader;
#[doc = "Field `_0P1A_TIEH_SEL_0` writer - need_des"]
pub type _0P1A_TIEH_SEL_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `_0P1A_TIEH_POS_EN_0` reader - need_des"]
pub type _0P1A_TIEH_POS_EN_0_R = crate::BitReader;
#[doc = "Field `_0P1A_TIEH_POS_EN_0` writer - need_des"]
pub type _0P1A_TIEH_POS_EN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P1A_TIEH_NEG_EN_0` reader - need_des"]
pub type _0P1A_TIEH_NEG_EN_0_R = crate::BitReader;
#[doc = "Field `_0P1A_TIEH_NEG_EN_0` writer - need_des"]
pub type _0P1A_TIEH_NEG_EN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P1A_TIEH_0` reader - need_des"]
pub type _0P1A_TIEH_0_R = crate::BitReader;
#[doc = "Field `_0P1A_TIEH_0` writer - need_des"]
pub type _0P1A_TIEH_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P1A_TARGET1_0` reader - need_des"]
pub type _0P1A_TARGET1_0_R = crate::FieldReader;
#[doc = "Field `_0P1A_TARGET1_0` writer - need_des"]
pub type _0P1A_TARGET1_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `_0P1A_TARGET0_0` reader - need_des"]
pub type _0P1A_TARGET0_0_R = crate::FieldReader;
#[doc = "Field `_0P1A_TARGET0_0` writer - need_des"]
pub type _0P1A_TARGET0_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `_0P1A_LDO_CNT_PRESCALER_SEL_0` reader - need_des"]
pub type _0P1A_LDO_CNT_PRESCALER_SEL_0_R = crate::BitReader;
#[doc = "Field `_0P1A_LDO_CNT_PRESCALER_SEL_0` writer - need_des"]
pub type _0P1A_LDO_CNT_PRESCALER_SEL_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn _0p1a_force_tieh_sel_0(&self) -> _0P1A_FORCE_TIEH_SEL_0_R {
        _0P1A_FORCE_TIEH_SEL_0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn _0p1a_xpd_0(&self) -> _0P1A_XPD_0_R {
        _0P1A_XPD_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - need_des"]
    #[inline(always)]
    pub fn _0p1a_tieh_sel_0(&self) -> _0P1A_TIEH_SEL_0_R {
        _0P1A_TIEH_SEL_0_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn _0p1a_tieh_pos_en_0(&self) -> _0P1A_TIEH_POS_EN_0_R {
        _0P1A_TIEH_POS_EN_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn _0p1a_tieh_neg_en_0(&self) -> _0P1A_TIEH_NEG_EN_0_R {
        _0P1A_TIEH_NEG_EN_0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn _0p1a_tieh_0(&self) -> _0P1A_TIEH_0_R {
        _0P1A_TIEH_0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn _0p1a_target1_0(&self) -> _0P1A_TARGET1_0_R {
        _0P1A_TARGET1_0_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn _0p1a_target0_0(&self) -> _0P1A_TARGET0_0_R {
        _0P1A_TARGET0_0_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn _0p1a_ldo_cnt_prescaler_sel_0(&self) -> _0P1A_LDO_CNT_PRESCALER_SEL_0_R {
        _0P1A_LDO_CNT_PRESCALER_SEL_0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_P0_0P1A")
            .field(
                "_0p1a_force_tieh_sel_0",
                &format_args!("{}", self._0p1a_force_tieh_sel_0().bit()),
            )
            .field("_0p1a_xpd_0", &format_args!("{}", self._0p1a_xpd_0().bit()))
            .field(
                "_0p1a_tieh_sel_0",
                &format_args!("{}", self._0p1a_tieh_sel_0().bits()),
            )
            .field(
                "_0p1a_tieh_pos_en_0",
                &format_args!("{}", self._0p1a_tieh_pos_en_0().bit()),
            )
            .field(
                "_0p1a_tieh_neg_en_0",
                &format_args!("{}", self._0p1a_tieh_neg_en_0().bit()),
            )
            .field(
                "_0p1a_tieh_0",
                &format_args!("{}", self._0p1a_tieh_0().bit()),
            )
            .field(
                "_0p1a_target1_0",
                &format_args!("{}", self._0p1a_target1_0().bits()),
            )
            .field(
                "_0p1a_target0_0",
                &format_args!("{}", self._0p1a_target0_0().bits()),
            )
            .field(
                "_0p1a_ldo_cnt_prescaler_sel_0",
                &format_args!("{}", self._0p1a_ldo_cnt_prescaler_sel_0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_LDO_P0_0P1A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_force_tieh_sel_0(&mut self) -> _0P1A_FORCE_TIEH_SEL_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_FORCE_TIEH_SEL_0_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_xpd_0(&mut self) -> _0P1A_XPD_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_XPD_0_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_sel_0(&mut self) -> _0P1A_TIEH_SEL_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_TIEH_SEL_0_W::new(self, 9)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_pos_en_0(&mut self) -> _0P1A_TIEH_POS_EN_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_TIEH_POS_EN_0_W::new(self, 12)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_neg_en_0(&mut self) -> _0P1A_TIEH_NEG_EN_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_TIEH_NEG_EN_0_W::new(self, 13)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_0(&mut self) -> _0P1A_TIEH_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_TIEH_0_W::new(self, 14)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_target1_0(&mut self) -> _0P1A_TARGET1_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_TARGET1_0_W::new(self, 15)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_target0_0(&mut self) -> _0P1A_TARGET0_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_TARGET0_0_W::new(self, 23)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_ldo_cnt_prescaler_sel_0(
        &mut self,
    ) -> _0P1A_LDO_CNT_PRESCALER_SEL_0_W<EXT_LDO_P0_0P1A_SPEC> {
        _0P1A_LDO_CNT_PRESCALER_SEL_0_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_LDO_P0_0P1A_SPEC;
impl crate::RegisterSpec for EXT_LDO_P0_0P1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_p0_0p1a::R`](R) reader structure"]
impl crate::Readable for EXT_LDO_P0_0P1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_p0_0p1a::W`](W) writer structure"]
impl crate::Writable for EXT_LDO_P0_0P1A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_LDO_P0_0P1A to value 0x4020_0100"]
impl crate::Resettable for EXT_LDO_P0_0P1A_SPEC {
    const RESET_VALUE: Self::Ux = 0x4020_0100;
}
