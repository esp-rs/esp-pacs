#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_5` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_5` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR` reader - BackUp access rtcfast_spltaddr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR` writer - BackUp access rtcfast_spltaddr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_5_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - BackUp access rtcfast_spltaddr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_spltaddr(
        &self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_5")
            .field(
                "backup_bus_pms_constrain_rtcfast_spltaddr",
                &format_args!(
                    "{}",
                    self.backup_bus_pms_constrain_rtcfast_spltaddr().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - BackUp access rtcfast_spltaddr permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rtcfast_spltaddr(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_W<0> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_5](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_5::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_5::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_5 to value 0x07ff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff;
}
