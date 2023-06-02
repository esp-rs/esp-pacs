#[doc = "Register `RESET_STATE` reader"]
pub struct R(crate::R<RESET_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STATE` writer"]
pub struct W(crate::W<RESET_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATE_SPEC>;
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
impl From<crate::W<RESET_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_CAUSE_PROCPU` reader - reset cause of PRO CPU"]
pub type RESET_CAUSE_PROCPU_R = crate::FieldReader;
#[doc = "Field `RESET_CAUSE_APPCPU` reader - reset cause of APP CPU"]
pub type RESET_CAUSE_APPCPU_R = crate::FieldReader;
#[doc = "Field `STAT_VECTOR_SEL_APPCPU` reader - APP CPU state vector sel"]
pub type STAT_VECTOR_SEL_APPCPU_R = crate::BitReader;
#[doc = "Field `STAT_VECTOR_SEL_APPCPU` writer - APP CPU state vector sel"]
pub type STAT_VECTOR_SEL_APPCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` reader - PRO CPU state vector sel"]
pub type STAT_VECTOR_SEL_PROCPU_R = crate::BitReader;
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` writer - PRO CPU state vector sel"]
pub type STAT_VECTOR_SEL_PROCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `ALL_RESET_FLAG_PROCPU` reader - PRO CPU reset_flag"]
pub type ALL_RESET_FLAG_PROCPU_R = crate::BitReader;
#[doc = "Field `ALL_RESET_FLAG_APPCPU` reader - APP CPU reset flag"]
pub type ALL_RESET_FLAG_APPCPU_R = crate::BitReader;
#[doc = "Field `ALL_RESET_FLAG_CLR_PROCPU` writer - clear PRO CPU reset_flag"]
pub type ALL_RESET_FLAG_CLR_PROCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `ALL_RESET_FLAG_CLR_APPCPU` writer - clear APP CPU reset flag"]
pub type ALL_RESET_FLAG_CLR_APPCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `OCD_HALT_ON_RESET_APPCPU` reader - APPCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_APPCPU_R = crate::BitReader;
#[doc = "Field `OCD_HALT_ON_RESET_APPCPU` writer - APPCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_APPCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` reader - PROCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_PROCPU_R = crate::BitReader;
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` writer - PROCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_PROCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `JTAG_RESET_FLAG_PROCPU` reader - configure jtag reset configure"]
pub type JTAG_RESET_FLAG_PROCPU_R = crate::BitReader;
#[doc = "Field `JTAG_RESET_FLAG_APPCPU` reader - configure jtag reset configure"]
pub type JTAG_RESET_FLAG_APPCPU_R = crate::BitReader;
#[doc = "Field `JTAG_RESET_FLAG_CLR_PROCPU` writer - configure jtag reset configure"]
pub type JTAG_RESET_FLAG_CLR_PROCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `JTAG_RESET_FLAG_CLR_APPCPU` writer - configure jtag reset configure"]
pub type JTAG_RESET_FLAG_CLR_APPCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `DRESET_MASK_APPCPU` reader - configure dreset configure"]
pub type DRESET_MASK_APPCPU_R = crate::BitReader;
#[doc = "Field `DRESET_MASK_APPCPU` writer - configure dreset configure"]
pub type DRESET_MASK_APPCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
#[doc = "Field `DRESET_MASK_PROCPU` reader - configure dreset configure"]
pub type DRESET_MASK_PROCPU_R = crate::BitReader;
#[doc = "Field `DRESET_MASK_PROCPU` writer - configure dreset configure"]
pub type DRESET_MASK_PROCPU_W<'a, const O: u8> = crate::BitWriter<'a, RESET_STATE_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - reset cause of APP CPU"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn stat_vector_sel_appcpu(&self) -> STAT_VECTOR_SEL_APPCPU_R {
        STAT_VECTOR_SEL_APPCPU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&self) -> STAT_VECTOR_SEL_PROCPU_R {
        STAT_VECTOR_SEL_PROCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PRO CPU reset_flag"]
    #[inline(always)]
    pub fn all_reset_flag_procpu(&self) -> ALL_RESET_FLAG_PROCPU_R {
        ALL_RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - APP CPU reset flag"]
    #[inline(always)]
    pub fn all_reset_flag_appcpu(&self) -> ALL_RESET_FLAG_APPCPU_R {
        ALL_RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - APPCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_appcpu(&self) -> OCD_HALT_ON_RESET_APPCPU_R {
        OCD_HALT_ON_RESET_APPCPU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&self) -> OCD_HALT_ON_RESET_PROCPU_R {
        OCD_HALT_ON_RESET_PROCPU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - configure jtag reset configure"]
    #[inline(always)]
    pub fn jtag_reset_flag_procpu(&self) -> JTAG_RESET_FLAG_PROCPU_R {
        JTAG_RESET_FLAG_PROCPU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - configure jtag reset configure"]
    #[inline(always)]
    pub fn jtag_reset_flag_appcpu(&self) -> JTAG_RESET_FLAG_APPCPU_R {
        JTAG_RESET_FLAG_APPCPU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - configure dreset configure"]
    #[inline(always)]
    pub fn dreset_mask_appcpu(&self) -> DRESET_MASK_APPCPU_R {
        DRESET_MASK_APPCPU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - configure dreset configure"]
    #[inline(always)]
    pub fn dreset_mask_procpu(&self) -> DRESET_MASK_PROCPU_R {
        DRESET_MASK_PROCPU_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_STATE")
            .field(
                "reset_cause_procpu",
                &format_args!("{}", self.reset_cause_procpu().bits()),
            )
            .field(
                "reset_cause_appcpu",
                &format_args!("{}", self.reset_cause_appcpu().bits()),
            )
            .field(
                "stat_vector_sel_appcpu",
                &format_args!("{}", self.stat_vector_sel_appcpu().bit()),
            )
            .field(
                "stat_vector_sel_procpu",
                &format_args!("{}", self.stat_vector_sel_procpu().bit()),
            )
            .field(
                "all_reset_flag_procpu",
                &format_args!("{}", self.all_reset_flag_procpu().bit()),
            )
            .field(
                "all_reset_flag_appcpu",
                &format_args!("{}", self.all_reset_flag_appcpu().bit()),
            )
            .field(
                "ocd_halt_on_reset_appcpu",
                &format_args!("{}", self.ocd_halt_on_reset_appcpu().bit()),
            )
            .field(
                "ocd_halt_on_reset_procpu",
                &format_args!("{}", self.ocd_halt_on_reset_procpu().bit()),
            )
            .field(
                "jtag_reset_flag_procpu",
                &format_args!("{}", self.jtag_reset_flag_procpu().bit()),
            )
            .field(
                "jtag_reset_flag_appcpu",
                &format_args!("{}", self.jtag_reset_flag_appcpu().bit()),
            )
            .field(
                "dreset_mask_appcpu",
                &format_args!("{}", self.dreset_mask_appcpu().bit()),
            )
            .field(
                "dreset_mask_procpu",
                &format_args!("{}", self.dreset_mask_procpu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    #[must_use]
    pub fn stat_vector_sel_appcpu(&mut self) -> STAT_VECTOR_SEL_APPCPU_W<12> {
        STAT_VECTOR_SEL_APPCPU_W::new(self)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    #[must_use]
    pub fn stat_vector_sel_procpu(&mut self) -> STAT_VECTOR_SEL_PROCPU_W<13> {
        STAT_VECTOR_SEL_PROCPU_W::new(self)
    }
    #[doc = "Bit 16 - clear PRO CPU reset_flag"]
    #[inline(always)]
    #[must_use]
    pub fn all_reset_flag_clr_procpu(&mut self) -> ALL_RESET_FLAG_CLR_PROCPU_W<16> {
        ALL_RESET_FLAG_CLR_PROCPU_W::new(self)
    }
    #[doc = "Bit 17 - clear APP CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn all_reset_flag_clr_appcpu(&mut self) -> ALL_RESET_FLAG_CLR_APPCPU_W<17> {
        ALL_RESET_FLAG_CLR_APPCPU_W::new(self)
    }
    #[doc = "Bit 18 - APPCPU OcdHaltOnReset"]
    #[inline(always)]
    #[must_use]
    pub fn ocd_halt_on_reset_appcpu(&mut self) -> OCD_HALT_ON_RESET_APPCPU_W<18> {
        OCD_HALT_ON_RESET_APPCPU_W::new(self)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    #[must_use]
    pub fn ocd_halt_on_reset_procpu(&mut self) -> OCD_HALT_ON_RESET_PROCPU_W<19> {
        OCD_HALT_ON_RESET_PROCPU_W::new(self)
    }
    #[doc = "Bit 22 - configure jtag reset configure"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_reset_flag_clr_procpu(&mut self) -> JTAG_RESET_FLAG_CLR_PROCPU_W<22> {
        JTAG_RESET_FLAG_CLR_PROCPU_W::new(self)
    }
    #[doc = "Bit 23 - configure jtag reset configure"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_reset_flag_clr_appcpu(&mut self) -> JTAG_RESET_FLAG_CLR_APPCPU_W<23> {
        JTAG_RESET_FLAG_CLR_APPCPU_W::new(self)
    }
    #[doc = "Bit 24 - configure dreset configure"]
    #[inline(always)]
    #[must_use]
    pub fn dreset_mask_appcpu(&mut self) -> DRESET_MASK_APPCPU_W<24> {
        DRESET_MASK_APPCPU_W::new(self)
    }
    #[doc = "Bit 25 - configure dreset configure"]
    #[inline(always)]
    #[must_use]
    pub fn dreset_mask_procpu(&mut self) -> DRESET_MASK_PROCPU_W<25> {
        DRESET_MASK_PROCPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state](index.html) module"]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_state::R](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_state::W](W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_STATE to value 0x3000"]
impl crate::Resettable for RESET_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x3000;
}
