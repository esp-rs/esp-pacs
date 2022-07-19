#[doc = "Register `RTC_RESET_STATE` reader"]
pub struct R(crate::R<RTC_RESET_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_RESET_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_RESET_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_RESET_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_RESET_STATE` writer"]
pub struct W(crate::W<RTC_RESET_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_RESET_STATE_SPEC>;
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
impl From<crate::W<RTC_RESET_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_RESET_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_CAUSE_PROCPU` reader - reset cause of PRO CPU"]
pub type RESET_CAUSE_PROCPU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESET_CAUSE_PROCPU` writer - reset cause of PRO CPU"]
pub type RESET_CAUSE_PROCPU_W<'a> = crate::FieldWriter<'a, u32, RTC_RESET_STATE_SPEC, u8, u8, 6, 0>;
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` reader - PRO CPU state vector sel"]
pub type STAT_VECTOR_SEL_PROCPU_R = crate::BitReader<bool>;
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` writer - PRO CPU state vector sel"]
pub type STAT_VECTOR_SEL_PROCPU_W<'a> = crate::BitWriter<'a, u32, RTC_RESET_STATE_SPEC, bool, 13>;
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` reader - PROCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_PROCPU_R = crate::BitReader<bool>;
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` writer - PROCPU OcdHaltOnReset"]
pub type OCD_HALT_ON_RESET_PROCPU_W<'a> = crate::BitWriter<'a, u32, RTC_RESET_STATE_SPEC, bool, 19>;
#[doc = "Field `RTC_DRESET_MASK_PROCPU` reader - Need add desc"]
pub type RTC_DRESET_MASK_PROCPU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_DRESET_MASK_PROCPU` writer - Need add desc"]
pub type RTC_DRESET_MASK_PROCPU_W<'a> = crate::BitWriter<'a, u32, RTC_RESET_STATE_SPEC, bool, 20>;
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
    pub fn rtc_dreset_mask_procpu(&self) -> RTC_DRESET_MASK_PROCPU_R {
        RTC_DRESET_MASK_PROCPU_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - reset cause of PRO CPU"]
    #[inline(always)]
    pub fn reset_cause_procpu(&mut self) -> RESET_CAUSE_PROCPU_W {
        RESET_CAUSE_PROCPU_W::new(self)
    }
    #[doc = "Bit 13 - PRO CPU state vector sel"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&mut self) -> STAT_VECTOR_SEL_PROCPU_W {
        STAT_VECTOR_SEL_PROCPU_W::new(self)
    }
    #[doc = "Bit 19 - PROCPU OcdHaltOnReset"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&mut self) -> OCD_HALT_ON_RESET_PROCPU_W {
        OCD_HALT_ON_RESET_PROCPU_W::new(self)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn rtc_dreset_mask_procpu(&mut self) -> RTC_DRESET_MASK_PROCPU_W {
        RTC_DRESET_MASK_PROCPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_reset_state](index.html) module"]
pub struct RTC_RESET_STATE_SPEC;
impl crate::RegisterSpec for RTC_RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_reset_state::R](R) reader structure"]
impl crate::Readable for RTC_RESET_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_reset_state::W](W) writer structure"]
impl crate::Writable for RTC_RESET_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_RESET_STATE to value 0x2000"]
impl crate::Resettable for RTC_RESET_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
