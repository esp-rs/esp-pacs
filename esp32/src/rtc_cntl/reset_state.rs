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
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_STATE")
            .field("reset_cause_procpu", &self.reset_cause_procpu())
            .field("reset_cause_appcpu", &self.reset_cause_appcpu())
            .field("appcpu_stat_vector_sel", &self.appcpu_stat_vector_sel())
            .field("procpu_stat_vector_sel", &self.procpu_stat_vector_sel())
            .finish()
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
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_state::R`](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_state::W`](W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_STATE to value 0x3000"]
impl crate::Resettable for RESET_STATE_SPEC {
    const RESET_VALUE: u32 = 0x3000;
}
