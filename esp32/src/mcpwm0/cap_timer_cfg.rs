#[doc = "Register `CAP_TIMER_CFG` reader"]
pub struct R(crate::R<CAP_TIMER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_TIMER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_TIMER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_TIMER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_TIMER_CFG` writer"]
pub struct W(crate::W<CAP_TIMER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_TIMER_CFG_SPEC>;
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
impl From<crate::W<CAP_TIMER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_TIMER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_TIMER_EN` reader - "]
pub type CAP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CAP_TIMER_EN` writer - "]
pub type CAP_TIMER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAP_TIMER_CFG_SPEC, O>;
#[doc = "Field `CAP_SYNCI_EN` reader - "]
pub type CAP_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `CAP_SYNCI_EN` writer - "]
pub type CAP_SYNCI_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAP_TIMER_CFG_SPEC, O>;
#[doc = "Field `CAP_SYNCI_SEL` reader - "]
pub type CAP_SYNCI_SEL_R = crate::FieldReader;
#[doc = "Field `CAP_SYNCI_SEL` writer - "]
pub type CAP_SYNCI_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CAP_TIMER_CFG_SPEC, 3, O>;
#[doc = "Field `CAP_SYNC_SW` writer - "]
pub type CAP_SYNC_SW_W<'a, const O: u8> = crate::BitWriter<'a, CAP_TIMER_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cap_timer_en(&self) -> CAP_TIMER_EN_R {
        CAP_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cap_synci_en(&self) -> CAP_SYNCI_EN_R {
        CAP_SYNCI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn cap_synci_sel(&self) -> CAP_SYNCI_SEL_R {
        CAP_SYNCI_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_CFG")
            .field(
                "cap_timer_en",
                &format_args!("{}", self.cap_timer_en().bit()),
            )
            .field(
                "cap_synci_en",
                &format_args!("{}", self.cap_synci_en().bit()),
            )
            .field(
                "cap_synci_sel",
                &format_args!("{}", self.cap_synci_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_TIMER_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cap_timer_en(&mut self) -> CAP_TIMER_EN_W<0> {
        CAP_TIMER_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cap_synci_en(&mut self) -> CAP_SYNCI_EN_W<1> {
        CAP_SYNCI_EN_W::new(self)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    #[must_use]
    pub fn cap_synci_sel(&mut self) -> CAP_SYNCI_SEL_W<2> {
        CAP_SYNCI_SEL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cap_sync_sw(&mut self) -> CAP_SYNC_SW_W<5> {
        CAP_SYNC_SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_timer_cfg](index.html) module"]
pub struct CAP_TIMER_CFG_SPEC;
impl crate::RegisterSpec for CAP_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_timer_cfg::R](R) reader structure"]
impl crate::Readable for CAP_TIMER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_timer_cfg::W](W) writer structure"]
impl crate::Writable for CAP_TIMER_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_TIMER_CFG to value 0"]
impl crate::Resettable for CAP_TIMER_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
