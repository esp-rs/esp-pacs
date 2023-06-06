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
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0` reader - RTCFast memory split address in world 0 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0` writer - RTCFast memory split address in world 0 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_9_SPEC, 11, O, u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1` reader - RTCFast memory split address in world 1 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1` writer - RTCFast memory split address in world 1 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_9_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - RTCFast memory split address in world 0 for core0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTCFast memory split address in world 1 for core0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_9")
            .field(
                "core_0_pif_pms_constrain_rtcfast_spltaddr_world_0",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_rtcfast_spltaddr_world_0()
                        .bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_rtcfast_spltaddr_world_1",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_rtcfast_spltaddr_world_1()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - RTCFast memory split address in world 0 for core0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcfast_spltaddr_world_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0_W::new(self)
    }
    #[doc = "Bits 11:21 - RTCFast memory split address in world 1 for core0."]
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
#[doc = "Core0 access peripherals permission configuration register 9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_9](index.html) module"]
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
