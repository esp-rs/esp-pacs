#[doc = "Register `TIMER2_SYNC` reader"]
pub struct R(crate::R<TIMER2_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_SYNC` writer"]
pub struct W(crate::W<TIMER2_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_SYNC_SPEC>;
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
impl From<crate::W<TIMER2_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER2_SYNCI_EN` reader - When set, timer reloading with phase on sync input event is enabled."]
pub type TIMER2_SYNCI_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2_SYNCI_EN` writer - When set, timer reloading with phase on sync input event is enabled."]
pub type TIMER2_SYNCI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER2_SYNC_SPEC, bool, O>;
#[doc = "Field `SW` reader - Toggling this bit will trigger a software sync."]
pub type SW_R = crate::BitReader<bool>;
#[doc = "Field `SW` writer - Toggling this bit will trigger a software sync."]
pub type SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER2_SYNC_SPEC, bool, O>;
#[doc = "Field `TIMER2_SYNCO_SEL` reader - PWM timer2 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
pub type TIMER2_SYNCO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER2_SYNCO_SEL` writer - PWM timer2 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
pub type TIMER2_SYNCO_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2_SYNC_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIMER2_PHASE` reader - phase for timer reload on sync event"]
pub type TIMER2_PHASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER2_PHASE` writer - phase for timer reload on sync event"]
pub type TIMER2_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2_SYNC_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    pub fn timer2_synci_en(&self) -> TIMER2_SYNCI_EN_R {
        TIMER2_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PWM timer2 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
    #[inline(always)]
    pub fn timer2_synco_sel(&self) -> TIMER2_SYNCO_SEL_R {
        TIMER2_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:20 - phase for timer reload on sync event"]
    #[inline(always)]
    pub fn timer2_phase(&self) -> TIMER2_PHASE_R {
        TIMER2_PHASE_R::new((self.bits >> 4) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - When set, timer reloading with phase on sync input event is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn timer2_synci_en(&mut self) -> TIMER2_SYNCI_EN_W<0> {
        TIMER2_SYNCI_EN_W::new(self)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync."]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<1> {
        SW_W::new(self)
    }
    #[doc = "Bits 2:3 - PWM timer2 sync_out selection, 0: synci, 1: TEZ, 2: TEP, otherwise:sync out is software sync"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_synco_sel(&mut self) -> TIMER2_SYNCO_SEL_W<2> {
        TIMER2_SYNCO_SEL_W::new(self)
    }
    #[doc = "Bits 4:20 - phase for timer reload on sync event"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_phase(&mut self) -> TIMER2_PHASE_W<4> {
        TIMER2_PHASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM timer2 sync function configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_sync](index.html) module"]
pub struct TIMER2_SYNC_SPEC;
impl crate::RegisterSpec for TIMER2_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_sync::R](R) reader structure"]
impl crate::Readable for TIMER2_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_sync::W](W) writer structure"]
impl crate::Writable for TIMER2_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2_SYNC to value 0"]
impl crate::Resettable for TIMER2_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
