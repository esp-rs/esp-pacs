#[doc = "Register `DSTS` reader"]
pub struct R(crate::R<DSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPSTS` reader - "]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - "]
pub type ENUMSPD_R = crate::FieldReader;
#[doc = "Field `ERRTICERR` reader - "]
pub type ERRTICERR_R = crate::BitReader;
#[doc = "Field `SOFFN` reader - "]
pub type SOFFN_R = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - "]
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn errticerr(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &format_args!("{}", self.suspsts().bit()))
            .field("enumspd", &format_args!("{}", self.enumspd().bits()))
            .field("errticerr", &format_args!("{}", self.errticerr().bit()))
            .field("soffn", &format_args!("{}", self.soffn().bits()))
            .field("devlnsts", &format_args!("{}", self.devlnsts().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](index.html) module"]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsts::R](R) reader structure"]
impl crate::Readable for DSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
