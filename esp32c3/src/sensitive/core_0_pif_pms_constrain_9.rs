#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_9` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_9` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0` reader - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0` writer - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_9_SPEC, u16, u16, 11, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1` reader - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1` writer - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_9_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
impl W {
    #[doc = "Bits 0:10 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W::new(self)
    }
    #[doc = "Bits 11:21 - core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<11> {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_9](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_9_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_9::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_9::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_9 to value 0x003f_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_9_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_ffff;
}
