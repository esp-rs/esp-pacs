#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_6` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_6` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L` reader - BackUp access rtcfast_l permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L` writer - BackUp access rtcfast_l permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_6_SPEC, 3, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H` reader - BackUp access rtcfast_h permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H` writer - BackUp access rtcfast_h permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_6_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - BackUp access rtcfast_l permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_l(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - BackUp access rtcfast_h permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_h(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_6")
            .field(
                "backup_bus_pms_constrain_rtcfast_l",
                &format_args!("{}", self.backup_bus_pms_constrain_rtcfast_l().bits()),
            )
            .field(
                "backup_bus_pms_constrain_rtcfast_h",
                &format_args!("{}", self.backup_bus_pms_constrain_rtcfast_h().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - BackUp access rtcfast_l permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rtcfast_l(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W<0> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W::new(self)
    }
    #[doc = "Bits 3:5 - BackUp access rtcfast_h permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rtcfast_h(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W<3> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_6](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_6::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_6::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_6 to value 0x3f"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
