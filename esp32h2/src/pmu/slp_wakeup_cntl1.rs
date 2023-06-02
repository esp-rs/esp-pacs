#[doc = "Register `SLP_WAKEUP_CNTL1` reader"]
pub struct R(crate::R<SLP_WAKEUP_CNTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CNTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CNTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CNTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_WAKEUP_CNTL1` writer"]
pub struct W(crate::W<SLP_WAKEUP_CNTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKEUP_CNTL1_SPEC>;
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
impl From<crate::W<SLP_WAKEUP_CNTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKEUP_CNTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_REJECT_ENA` reader - need_des"]
pub type SLEEP_REJECT_ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLEEP_REJECT_ENA` writer - need_des"]
pub type SLEEP_REJECT_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLP_WAKEUP_CNTL1_SPEC, 31, O, u32, u32>;
#[doc = "Field `SLP_REJECT_EN` reader - need_des"]
pub type SLP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_EN` writer - need_des"]
pub type SLP_REJECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLP_WAKEUP_CNTL1_SPEC, O>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SLEEP_REJECT_ENA_R {
        SLEEP_REJECT_ENA_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn slp_reject_en(&self) -> SLP_REJECT_EN_R {
        SLP_REJECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL1")
            .field(
                "sleep_reject_ena",
                &format_args!("{}", self.sleep_reject_ena().bits()),
            )
            .field(
                "slp_reject_en",
                &format_args!("{}", self.slp_reject_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_reject_ena(&mut self) -> SLEEP_REJECT_ENA_W<0> {
        SLEEP_REJECT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_en(&mut self) -> SLP_REJECT_EN_W<31> {
        SLP_REJECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cntl1](index.html) module"]
pub struct SLP_WAKEUP_CNTL1_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cntl1::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_wakeup_cntl1::W](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL1 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
