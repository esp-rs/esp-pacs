#[doc = "Register `HP_ACTIVE_HP_REGULATOR0` reader"]
pub type R = crate::R<HP_ACTIVE_HP_REGULATOR0_SPEC>;
#[doc = "Register `HP_ACTIVE_HP_REGULATOR0` writer"]
pub type W = crate::W<HP_ACTIVE_HP_REGULATOR0_SPEC>;
#[doc = "Field `LP_DBIAS_VOL` reader - need_des"]
pub type LP_DBIAS_VOL_R = crate::FieldReader;
#[doc = "Field `HP_DBIAS_VOL` reader - need_des"]
pub type HP_DBIAS_VOL_R = crate::FieldReader;
#[doc = "Field `DIG_REGULATOR0_DBIAS_SEL` reader - need_des"]
pub type DIG_REGULATOR0_DBIAS_SEL_R = crate::BitReader;
#[doc = "Field `DIG_REGULATOR0_DBIAS_SEL` writer - need_des"]
pub type DIG_REGULATOR0_DBIAS_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIG_DBIAS_INIT` writer - need_des"]
pub type DIG_DBIAS_INIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD` reader - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD` writer - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD` reader - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD` writer - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_XPD` reader - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_XPD_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_XPD` writer - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_XPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS` reader - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS` writer - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS` reader - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS` writer - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_DBIAS` reader - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_DBIAS` writer - need_des"]
pub type HP_ACTIVE_HP_REGULATOR_DBIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 4:8 - need_des"]
    #[inline(always)]
    pub fn lp_dbias_vol(&self) -> LP_DBIAS_VOL_R {
        LP_DBIAS_VOL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - need_des"]
    #[inline(always)]
    pub fn hp_dbias_vol(&self) -> HP_DBIAS_VOL_R {
        HP_DBIAS_VOL_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn dig_regulator0_dbias_sel(&self) -> DIG_REGULATOR0_DBIAS_SEL_R {
        DIG_REGULATOR0_DBIAS_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_mem_xpd(&self) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_R {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_logic_xpd(&self) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_R {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_xpd(&self) -> HP_ACTIVE_HP_REGULATOR_XPD_R {
        HP_ACTIVE_HP_REGULATOR_XPD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_mem_dbias(&self) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_R {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_logic_dbias(
        &self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_R {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_dbias(&self) -> HP_ACTIVE_HP_REGULATOR_DBIAS_R {
        HP_ACTIVE_HP_REGULATOR_DBIAS_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_HP_REGULATOR0")
            .field(
                "lp_dbias_vol",
                &format_args!("{}", self.lp_dbias_vol().bits()),
            )
            .field(
                "hp_dbias_vol",
                &format_args!("{}", self.hp_dbias_vol().bits()),
            )
            .field(
                "dig_regulator0_dbias_sel",
                &format_args!("{}", self.dig_regulator0_dbias_sel().bit()),
            )
            .field(
                "hp_active_hp_regulator_slp_mem_xpd",
                &format_args!("{}", self.hp_active_hp_regulator_slp_mem_xpd().bit()),
            )
            .field(
                "hp_active_hp_regulator_slp_logic_xpd",
                &format_args!("{}", self.hp_active_hp_regulator_slp_logic_xpd().bit()),
            )
            .field(
                "hp_active_hp_regulator_xpd",
                &format_args!("{}", self.hp_active_hp_regulator_xpd().bit()),
            )
            .field(
                "hp_active_hp_regulator_slp_mem_dbias",
                &format_args!("{}", self.hp_active_hp_regulator_slp_mem_dbias().bits()),
            )
            .field(
                "hp_active_hp_regulator_slp_logic_dbias",
                &format_args!("{}", self.hp_active_hp_regulator_slp_logic_dbias().bits()),
            )
            .field(
                "hp_active_hp_regulator_dbias",
                &format_args!("{}", self.hp_active_hp_regulator_dbias().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_ACTIVE_HP_REGULATOR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dig_regulator0_dbias_sel(
        &mut self,
    ) -> DIG_REGULATOR0_DBIAS_SEL_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 14> {
        DIG_REGULATOR0_DBIAS_SEL_W::new(self)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dig_dbias_init(&mut self) -> DIG_DBIAS_INIT_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 15> {
        DIG_DBIAS_INIT_W::new(self)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_regulator_slp_mem_xpd(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 16> {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_W::new(self)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_regulator_slp_logic_xpd(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 17> {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_W::new(self)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_regulator_xpd(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_XPD_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 18> {
        HP_ACTIVE_HP_REGULATOR_XPD_W::new(self)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_regulator_slp_mem_dbias(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 19> {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_W::new(self)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_regulator_slp_logic_dbias(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 23> {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_W::new(self)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_regulator_dbias(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_DBIAS_W<HP_ACTIVE_HP_REGULATOR0_SPEC, 27> {
        HP_ACTIVE_HP_REGULATOR_DBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_active_hp_regulator0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_hp_regulator0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_HP_REGULATOR0_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_REGULATOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_hp_regulator0::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_REGULATOR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_hp_regulator0::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_REGULATOR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_REGULATOR0 to value 0xc667_7180"]
impl crate::Resettable for HP_ACTIVE_HP_REGULATOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0xc667_7180;
}
