#[doc = "Register `RESET_STATE` reader"]
pub type R = crate::R<RESET_STATE_SPEC>;
#[doc = "Register `RESET_STATE` writer"]
pub type W = crate::W<RESET_STATE_SPEC>;
#[doc = "Field `RESET_CAUSE_PROCPU` reader - reset cause of PRO CPU"]
pub type RESET_CAUSE_PROCPU_R = crate::FieldReader;
#[doc = "Field `RESET_CAUSE_APPCPU` reader - reset cause of APP CPU"]
pub type RESET_CAUSE_APPCPU_R = crate::FieldReader;
#[doc = "Field `APPCPU_STAT_VECTOR_SEL` reader - APP CPU state vector sel"]
pub type APPCPU_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `APPCPU_STAT_VECTOR_SEL` writer - APP CPU state vector sel"]
pub type APPCPU_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROCPU_STAT_VECTOR_SEL` reader - PRO CPU state vector sel"]
pub type PROCPU_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `PROCPU_STAT_VECTOR_SEL` writer - PRO CPU state vector sel"]
pub type PROCPU_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_FLAG_PROCPU` reader - PRO CPU reset_flag"]
pub type RESET_FLAG_PROCPU_R = crate::BitReader;
#[doc = "Field `RESET_FLAG_APPCPU` reader - APP CPU reset flag"]
pub type RESET_FLAG_APPCPU_R = crate::BitReader;
#[doc = "Field `RESET_FLAG_PROCPU_CLR` writer - clear PRO CPU reset_flag"]
pub type RESET_FLAG_PROCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_FLAG_APPCPU_CLR` writer - clear APP CPU reset flag"]
pub type RESET_FLAG_APPCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPCPU_OCD_HALT_ON_RESET` reader - APPCPU OcdHaltOnReset"]
pub type APPCPU_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `APPCPU_OCD_HALT_ON_RESET` writer - APPCPU OcdHaltOnReset"]
pub type APPCPU_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROCPU_OCD_HALT_ON_RESET` reader - PROCPU OcdHaltOnReset"]
pub type PROCPU_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `PROCPU_OCD_HALT_ON_RESET` writer - PROCPU OcdHaltOnReset"]
pub type PROCPU_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_FLAG_JTAG_PROCPU` reader - jtag reset flag"]
pub type RESET_FLAG_JTAG_PROCPU_R = crate::BitReader;
#[doc = "Field `RESET_FLAG_JTAG_APPCPU` reader - jtag reset flag"]
pub type RESET_FLAG_JTAG_APPCPU_R = crate::BitReader;
#[doc = "Field `RESET_FLAG_JTAG_PROCPU_CLR` writer - clear jtag reset flag"]
pub type RESET_FLAG_JTAG_PROCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_FLAG_JTAG_APPCPU_CLR` writer - clear jtag reset flag"]
pub type RESET_FLAG_JTAG_APPCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_DRESET_MASK` reader - bypass cpu1 dreset"]
pub type APP_DRESET_MASK_R = crate::BitReader;
#[doc = "Field `APP_DRESET_MASK` writer - bypass cpu1 dreset"]
pub type APP_DRESET_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRESET_MASK` reader - bypass cpu0 dreset"]
pub type PRO_DRESET_MASK_R = crate::BitReader;
#[doc = "Field `PRO_DRESET_MASK` writer - bypass cpu0 dreset"]
pub type PRO_DRESET_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn appcpu_stat_vector_sel(&self) -> APPCPU_STAT_VECTOR_SEL_R {
        APPCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&self) -> PROCPU_STAT_VECTOR_SEL_R {
        PROCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PRO CPU reset_flag"]
    #[inline(always)]
    pub fn reset_flag_procpu(&self) -> RESET_FLAG_PROCPU_R {
        RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - APP CPU reset flag"]
    #[inline(always)]
    pub fn reset_flag_appcpu(&self) -> RESET_FLAG_APPCPU_R {
        RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - APPCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn appcpu_ocd_halt_on_reset(&self) -> APPCPU_OCD_HALT_ON_RESET_R {
        APPCPU_OCD_HALT_ON_RESET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn procpu_ocd_halt_on_reset(&self) -> PROCPU_OCD_HALT_ON_RESET_R {
        PROCPU_OCD_HALT_ON_RESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - jtag reset flag"]
    #[inline(always)]
    pub fn reset_flag_jtag_procpu(&self) -> RESET_FLAG_JTAG_PROCPU_R {
        RESET_FLAG_JTAG_PROCPU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - jtag reset flag"]
    #[inline(always)]
    pub fn reset_flag_jtag_appcpu(&self) -> RESET_FLAG_JTAG_APPCPU_R {
        RESET_FLAG_JTAG_APPCPU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - bypass cpu1 dreset"]
    #[inline(always)]
    pub fn app_dreset_mask(&self) -> APP_DRESET_MASK_R {
        APP_DRESET_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - bypass cpu0 dreset"]
    #[inline(always)]
    pub fn pro_dreset_mask(&self) -> PRO_DRESET_MASK_R {
        PRO_DRESET_MASK_R::new(((self.bits >> 25) & 1) != 0)
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
                "appcpu_stat_vector_sel",
                &format_args!("{}", self.appcpu_stat_vector_sel().bit()),
            )
            .field(
                "procpu_stat_vector_sel",
                &format_args!("{}", self.procpu_stat_vector_sel().bit()),
            )
            .field(
                "reset_flag_procpu",
                &format_args!("{}", self.reset_flag_procpu().bit()),
            )
            .field(
                "reset_flag_appcpu",
                &format_args!("{}", self.reset_flag_appcpu().bit()),
            )
            .field(
                "appcpu_ocd_halt_on_reset",
                &format_args!("{}", self.appcpu_ocd_halt_on_reset().bit()),
            )
            .field(
                "procpu_ocd_halt_on_reset",
                &format_args!("{}", self.procpu_ocd_halt_on_reset().bit()),
            )
            .field(
                "reset_flag_jtag_procpu",
                &format_args!("{}", self.reset_flag_jtag_procpu().bit()),
            )
            .field(
                "reset_flag_jtag_appcpu",
                &format_args!("{}", self.reset_flag_jtag_appcpu().bit()),
            )
            .field(
                "app_dreset_mask",
                &format_args!("{}", self.app_dreset_mask().bit()),
            )
            .field(
                "pro_dreset_mask",
                &format_args!("{}", self.pro_dreset_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    #[must_use]
    pub fn appcpu_stat_vector_sel(&mut self) -> APPCPU_STAT_VECTOR_SEL_W<RESET_STATE_SPEC> {
        APPCPU_STAT_VECTOR_SEL_W::new(self, 12)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    #[must_use]
    pub fn procpu_stat_vector_sel(&mut self) -> PROCPU_STAT_VECTOR_SEL_W<RESET_STATE_SPEC> {
        PROCPU_STAT_VECTOR_SEL_W::new(self, 13)
    }
    #[doc = "Bit 16 - clear PRO CPU reset_flag"]
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_procpu_clr(&mut self) -> RESET_FLAG_PROCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_PROCPU_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear APP CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_appcpu_clr(&mut self) -> RESET_FLAG_APPCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_APPCPU_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - APPCPU OcdHaltOnReset"]
    #[inline(always)]
    #[must_use]
    pub fn appcpu_ocd_halt_on_reset(&mut self) -> APPCPU_OCD_HALT_ON_RESET_W<RESET_STATE_SPEC> {
        APPCPU_OCD_HALT_ON_RESET_W::new(self, 18)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    #[must_use]
    pub fn procpu_ocd_halt_on_reset(&mut self) -> PROCPU_OCD_HALT_ON_RESET_W<RESET_STATE_SPEC> {
        PROCPU_OCD_HALT_ON_RESET_W::new(self, 19)
    }
    #[doc = "Bit 22 - clear jtag reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_jtag_procpu_clr(&mut self) -> RESET_FLAG_JTAG_PROCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_JTAG_PROCPU_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23 - clear jtag reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_jtag_appcpu_clr(&mut self) -> RESET_FLAG_JTAG_APPCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_JTAG_APPCPU_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24 - bypass cpu1 dreset"]
    #[inline(always)]
    #[must_use]
    pub fn app_dreset_mask(&mut self) -> APP_DRESET_MASK_W<RESET_STATE_SPEC> {
        APP_DRESET_MASK_W::new(self, 24)
    }
    #[doc = "Bit 25 - bypass cpu0 dreset"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dreset_mask(&mut self) -> PRO_DRESET_MASK_W<RESET_STATE_SPEC> {
        PRO_DRESET_MASK_W::new(self, 25)
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
#[doc = "get reset state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_state::R`](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_state::W`](W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_STATE to value 0x3000"]
impl crate::Resettable for RESET_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x3000;
}
