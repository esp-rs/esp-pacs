#[doc = "Register `DBUS_PMS_TBL_LOCK` reader"]
pub struct R(crate::R<DBUS_PMS_TBL_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS_PMS_TBL_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS_PMS_TBL_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS_PMS_TBL_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBUS_PMS_TBL_LOCK` writer"]
pub struct W(crate::W<DBUS_PMS_TBL_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBUS_PMS_TBL_LOCK_SPEC>;
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
impl From<crate::W<DBUS_PMS_TBL_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBUS_PMS_TBL_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBUS_PMS_LOCK` reader - The bit is used to configure the ibus permission control section boundary0"]
pub type DBUS_PMS_LOCK_R = crate::BitReader;
#[doc = "Field `DBUS_PMS_LOCK` writer - The bit is used to configure the ibus permission control section boundary0"]
pub type DBUS_PMS_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, DBUS_PMS_TBL_LOCK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to configure the ibus permission control section boundary0"]
    #[inline(always)]
    pub fn dbus_pms_lock(&self) -> DBUS_PMS_LOCK_R {
        DBUS_PMS_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_PMS_TBL_LOCK")
            .field(
                "dbus_pms_lock",
                &format_args!("{}", self.dbus_pms_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS_PMS_TBL_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to configure the ibus permission control section boundary0"]
    #[inline(always)]
    #[must_use]
    pub fn dbus_pms_lock(&mut self) -> DBUS_PMS_LOCK_W<0> {
        DBUS_PMS_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus_pms_tbl_lock](index.html) module"]
pub struct DBUS_PMS_TBL_LOCK_SPEC;
impl crate::RegisterSpec for DBUS_PMS_TBL_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus_pms_tbl_lock::R](R) reader structure"]
impl crate::Readable for DBUS_PMS_TBL_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbus_pms_tbl_lock::W](W) writer structure"]
impl crate::Writable for DBUS_PMS_TBL_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBUS_PMS_TBL_LOCK to value 0"]
impl crate::Resettable for DBUS_PMS_TBL_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
