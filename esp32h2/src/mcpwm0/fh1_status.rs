#[doc = "Register `FH1_STATUS` reader"]
pub struct R(crate::R<FH1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TZ1_CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
pub type TZ1_CBC_ON_R = crate::BitReader;
#[doc = "Field `TZ1_OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going"]
pub type TZ1_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn tz1_cbc_on(&self) -> TZ1_CBC_ON_R {
        TZ1_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going"]
    #[inline(always)]
    pub fn tz1_ost_on(&self) -> TZ1_OST_ON_R {
        TZ1_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH1_STATUS")
            .field("tz1_cbc_on", &format_args!("{}", self.tz1_cbc_on().bit()))
            .field("tz1_ost_on", &format_args!("{}", self.tz1_ost_on().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status of fault events.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh1_status](index.html) module"]
pub struct FH1_STATUS_SPEC;
impl crate::RegisterSpec for FH1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh1_status::R](R) reader structure"]
impl crate::Readable for FH1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FH1_STATUS to value 0"]
impl crate::Resettable for FH1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
