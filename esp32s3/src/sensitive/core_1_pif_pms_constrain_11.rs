#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_11` reader"]
pub struct R(crate::R<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_11` writer"]
pub struct W(crate::W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>;
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
impl From<crate::W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_PIF_PMS_CONSTRAIN_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0` reader - RTCSlow_0 memory split address in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0` writer - RTCSlow_0 memory split address in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_PIF_PMS_CONSTRAIN_11_SPEC, u16, u16, 11, O>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1` reader - RTCSlow_0 memory split address in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1` writer - RTCSlow_0 memory split address in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_PIF_PMS_CONSTRAIN_11_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - RTCSlow_0 memory split address in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_0(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTCSlow_0 memory split address in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
impl W {
    #[doc = "Bits 0:10 - RTCSlow_0 memory split address in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_W<0> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_0_W::new(self)
    }
    #[doc = "Bits 11:21 - RTCSlow_0 memory split address in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_0_spltaddr_world_1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_W<11> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_0_SPLTADDR_WORLD_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 access peripherals permission configuration register 11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_pif_pms_constrain_11](index.html) module"]
pub struct CORE_1_PIF_PMS_CONSTRAIN_11_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_pif_pms_constrain_11::R](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_pif_pms_constrain_11::W](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_11 to value 0x003f_ffff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003f_ffff
    }
}
