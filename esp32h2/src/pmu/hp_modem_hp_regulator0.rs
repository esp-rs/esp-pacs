#[doc = "Register `HP_MODEM_HP_REGULATOR0` reader"]
pub struct R(crate::R<HP_MODEM_HP_REGULATOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_MODEM_HP_REGULATOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_MODEM_HP_REGULATOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_MODEM_HP_REGULATOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_MODEM_HP_REGULATOR0` writer"]
pub struct W(crate::W<HP_MODEM_HP_REGULATOR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_MODEM_HP_REGULATOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HP_MODEM_HP_REGULATOR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_MODEM_HP_REGULATOR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_MODEM_HP_POWER_DET_BYPASS` reader - need_des"]
pub type HP_MODEM_HP_POWER_DET_BYPASS_R = crate::BitReader;
#[doc = "Field `HP_MODEM_HP_POWER_DET_BYPASS` writer - need_des"]
pub type HP_MODEM_HP_POWER_DET_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, O>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_MEM_XPD` reader - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_R = crate::BitReader;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_MEM_XPD` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, O>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD` reader - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_R = crate::BitReader;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, O>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_XPD` reader - need_des"]
pub type HP_MODEM_HP_REGULATOR_XPD_R = crate::BitReader;
#[doc = "Field `HP_MODEM_HP_REGULATOR_XPD` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_XPD_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, O>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS` reader - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, 4, O>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS` reader - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, 4, O>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_DBIAS` reader - need_des"]
pub type HP_MODEM_HP_REGULATOR_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_MODEM_HP_REGULATOR_DBIAS` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_DBIAS_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_MODEM_HP_REGULATOR0_SPEC, 5, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_power_det_bypass(&self) -> HP_MODEM_HP_POWER_DET_BYPASS_R {
        HP_MODEM_HP_POWER_DET_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_mem_xpd(&self) -> HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_R {
        HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_logic_xpd(&self) -> HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_R {
        HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_xpd(&self) -> HP_MODEM_HP_REGULATOR_XPD_R {
        HP_MODEM_HP_REGULATOR_XPD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_mem_dbias(&self) -> HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_R {
        HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_logic_dbias(&self) -> HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_R {
        HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_dbias(&self) -> HP_MODEM_HP_REGULATOR_DBIAS_R {
        HP_MODEM_HP_REGULATOR_DBIAS_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_HP_REGULATOR0")
            .field(
                "hp_modem_hp_power_det_bypass",
                &format_args!("{}", self.hp_modem_hp_power_det_bypass().bit()),
            )
            .field(
                "hp_modem_hp_regulator_slp_mem_xpd",
                &format_args!("{}", self.hp_modem_hp_regulator_slp_mem_xpd().bit()),
            )
            .field(
                "hp_modem_hp_regulator_slp_logic_xpd",
                &format_args!("{}", self.hp_modem_hp_regulator_slp_logic_xpd().bit()),
            )
            .field(
                "hp_modem_hp_regulator_xpd",
                &format_args!("{}", self.hp_modem_hp_regulator_xpd().bit()),
            )
            .field(
                "hp_modem_hp_regulator_slp_mem_dbias",
                &format_args!("{}", self.hp_modem_hp_regulator_slp_mem_dbias().bits()),
            )
            .field(
                "hp_modem_hp_regulator_slp_logic_dbias",
                &format_args!("{}", self.hp_modem_hp_regulator_slp_logic_dbias().bits()),
            )
            .field(
                "hp_modem_hp_regulator_dbias",
                &format_args!("{}", self.hp_modem_hp_regulator_dbias().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_HP_REGULATOR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_power_det_bypass(&mut self) -> HP_MODEM_HP_POWER_DET_BYPASS_W<0> {
        HP_MODEM_HP_POWER_DET_BYPASS_W::new(self)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_slp_mem_xpd(&mut self) -> HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_W<16> {
        HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_W::new(self)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_slp_logic_xpd(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_W<17> {
        HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_W::new(self)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_xpd(&mut self) -> HP_MODEM_HP_REGULATOR_XPD_W<18> {
        HP_MODEM_HP_REGULATOR_XPD_W::new(self)
    }
    #[doc = "Bits 19:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_slp_mem_dbias(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_W<19> {
        HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_W::new(self)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_slp_logic_dbias(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_W<23> {
        HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_W::new(self)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_dbias(&mut self) -> HP_MODEM_HP_REGULATOR_DBIAS_W<27> {
        HP_MODEM_HP_REGULATOR_DBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_modem_hp_regulator0](index.html) module"]
pub struct HP_MODEM_HP_REGULATOR0_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_REGULATOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_modem_hp_regulator0::R](R) reader structure"]
impl crate::Readable for HP_MODEM_HP_REGULATOR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_modem_hp_regulator0::W](W) writer structure"]
impl crate::Writable for HP_MODEM_HP_REGULATOR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_HP_REGULATOR0 to value 0x8447_0000"]
impl crate::Resettable for HP_MODEM_HP_REGULATOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8447_0000;
}
