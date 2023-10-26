#[doc = "Register `RESET_STATE` reader"]
pub type R = crate::R<RESET_STATE_SPEC>;
#[doc = "Register `RESET_STATE` writer"]
pub type W = crate::W<RESET_STATE_SPEC>;
#[doc = "Field `RESET_CAUSE_PROCPU` reader - reset cause of PRO CPU"]
pub type RESET_CAUSE_PROCPU_R = crate::FieldReader;
#[doc = "Field `RESET_CAUSE_PROCPU` writer - reset cause of PRO CPU"]
pub type RESET_CAUSE_PROCPU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` reader - PRO CPU state vector sel"]
pub type STAT_VECTOR_SEL_PROCPU_R = crate::BitReader;
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` writer - PRO CPU state vector sel"]
pub type STAT_VECTOR_SEL_PROCPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` reader - PROCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_PROCPU_R = crate::BitReader;
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` writer - PROCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_PROCPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRESET_MASK_PROCPU` reader - Need add desc"]
pub type DRESET_MASK_PROCPU_R = crate::BitReader;
#[doc = "Field `DRESET_MASK_PROCPU` writer - Need add desc"]
pub type DRESET_MASK_PROCPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&self) -> STAT_VECTOR_SEL_PROCPU_R {
        STAT_VECTOR_SEL_PROCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&self) -> OCD_HALT_ON_RESET_PROCPU_R {
        OCD_HALT_ON_RESET_PROCPU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn dreset_mask_procpu(&self) -> DRESET_MASK_PROCPU_R {
        DRESET_MASK_PROCPU_R::new(((self.bits >> 20) & 1) != 0)
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
                "stat_vector_sel_procpu",
                &format_args!("{}", self.stat_vector_sel_procpu().bit()),
            )
            .field(
                "ocd_halt_on_reset_procpu",
                &format_args!("{}", self.ocd_halt_on_reset_procpu().bit()),
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
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    #[must_use]
    pub fn reset_cause_procpu(&mut self) -> RESET_CAUSE_PROCPU_W<RESET_STATE_SPEC, 0> {
        RESET_CAUSE_PROCPU_W::new(self)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    #[must_use]
    pub fn stat_vector_sel_procpu(&mut self) -> STAT_VECTOR_SEL_PROCPU_W<RESET_STATE_SPEC, 13> {
        STAT_VECTOR_SEL_PROCPU_W::new(self)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    #[must_use]
    pub fn ocd_halt_on_reset_procpu(&mut self) -> OCD_HALT_ON_RESET_PROCPU_W<RESET_STATE_SPEC, 19> {
        OCD_HALT_ON_RESET_PROCPU_W::new(self)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dreset_mask_procpu(&mut self) -> DRESET_MASK_PROCPU_W<RESET_STATE_SPEC, 20> {
        DRESET_MASK_PROCPU_W::new(self)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets RESET_STATE to value 0x2000"]
impl crate::Resettable for RESET_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
