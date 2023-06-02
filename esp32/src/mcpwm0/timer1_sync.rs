#[doc = "Register `TIMER1_SYNC` reader"]
pub struct R(crate::R<TIMER1_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_SYNC` writer"]
pub struct W(crate::W<TIMER1_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SYNC_SPEC>;
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
impl From<crate::W<TIMER1_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_SYNCI_EN` reader - "]
pub type TIMER1_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `TIMER1_SYNCI_EN` writer - "]
pub type TIMER1_SYNCI_EN_W<'a, const O: u8> = crate::BitWriter<'a, TIMER1_SYNC_SPEC, O>;
#[doc = "Field `SW` reader - "]
pub type SW_R = crate::BitReader;
#[doc = "Field `SW` writer - "]
pub type SW_W<'a, const O: u8> = crate::BitWriter<'a, TIMER1_SYNC_SPEC, O>;
#[doc = "Field `TIMER1_SYNCO_SEL` reader - "]
pub type TIMER1_SYNCO_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER1_SYNCO_SEL` writer - "]
pub type TIMER1_SYNCO_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER1_SYNC_SPEC, 2, O>;
#[doc = "Field `TIMER1_PHASE` reader - "]
pub type TIMER1_PHASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER1_PHASE` writer - "]
pub type TIMER1_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, TIMER1_SYNC_SPEC, 16, O, u16, u16>;
#[doc = "Field `TIMER1_PHASE_DIRECTION` reader - "]
pub type TIMER1_PHASE_DIRECTION_R = crate::BitReader;
#[doc = "Field `TIMER1_PHASE_DIRECTION` writer - "]
pub type TIMER1_PHASE_DIRECTION_W<'a, const O: u8> = crate::BitWriter<'a, TIMER1_SYNC_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer1_synci_en(&self) -> TIMER1_SYNCI_EN_R {
        TIMER1_SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn timer1_synco_sel(&self) -> TIMER1_SYNCO_SEL_R {
        TIMER1_SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn timer1_phase(&self) -> TIMER1_PHASE_R {
        TIMER1_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn timer1_phase_direction(&self) -> TIMER1_PHASE_DIRECTION_R {
        TIMER1_PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1_SYNC")
            .field(
                "timer1_synci_en",
                &format_args!("{}", self.timer1_synci_en().bit()),
            )
            .field("sw", &format_args!("{}", self.sw().bit()))
            .field(
                "timer1_synco_sel",
                &format_args!("{}", self.timer1_synco_sel().bits()),
            )
            .field(
                "timer1_phase",
                &format_args!("{}", self.timer1_phase().bits()),
            )
            .field(
                "timer1_phase_direction",
                &format_args!("{}", self.timer1_phase_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER1_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_synci_en(&mut self) -> TIMER1_SYNCI_EN_W<0> {
        TIMER1_SYNCI_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<1> {
        SW_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_synco_sel(&mut self) -> TIMER1_SYNCO_SEL_W<2> {
        TIMER1_SYNCO_SEL_W::new(self)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_phase(&mut self) -> TIMER1_PHASE_W<4> {
        TIMER1_PHASE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_phase_direction(&mut self) -> TIMER1_PHASE_DIRECTION_W<20> {
        TIMER1_PHASE_DIRECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_sync](index.html) module"]
pub struct TIMER1_SYNC_SPEC;
impl crate::RegisterSpec for TIMER1_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_sync::R](R) reader structure"]
impl crate::Readable for TIMER1_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_sync::W](W) writer structure"]
impl crate::Writable for TIMER1_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1_SYNC to value 0"]
impl crate::Resettable for TIMER1_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
