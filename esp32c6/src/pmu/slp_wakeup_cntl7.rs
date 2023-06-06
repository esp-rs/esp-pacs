#[doc = "Register `SLP_WAKEUP_CNTL7` reader"]
pub struct R(crate::R<SLP_WAKEUP_CNTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CNTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CNTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CNTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_WAKEUP_CNTL7` writer"]
pub struct W(crate::W<SLP_WAKEUP_CNTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKEUP_CNTL7_SPEC>;
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
impl From<crate::W<SLP_WAKEUP_CNTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKEUP_CNTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANA_WAIT_TARGET` reader - need_des"]
pub type ANA_WAIT_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `ANA_WAIT_TARGET` writer - need_des"]
pub type ANA_WAIT_TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLP_WAKEUP_CNTL7_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn ana_wait_target(&self) -> ANA_WAIT_TARGET_R {
        ANA_WAIT_TARGET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL7")
            .field(
                "ana_wait_target",
                &format_args!("{}", self.ana_wait_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_wait_target(&mut self) -> ANA_WAIT_TARGET_W<16> {
        ANA_WAIT_TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cntl7](index.html) module"]
pub struct SLP_WAKEUP_CNTL7_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cntl7::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_wakeup_cntl7::W](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL7 to value 0x0001_0000"]
impl crate::Resettable for SLP_WAKEUP_CNTL7_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
