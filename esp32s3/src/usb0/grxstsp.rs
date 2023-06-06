#[doc = "Register `GRXSTSP` reader"]
pub struct R(crate::R<GRXSTSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHNUM` reader - "]
pub type CHNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - "]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - "]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - "]
pub type PKTSTS_R = crate::FieldReader;
#[doc = "Field `FN` reader - "]
pub type FN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSP")
            .field("chnum", &format_args!("{}", self.chnum().bits()))
            .field("bcnt", &format_args!("{}", self.bcnt().bits()))
            .field("dpid", &format_args!("{}", self.dpid().bits()))
            .field("pktsts", &format_args!("{}", self.pktsts().bits()))
            .field("fn_", &format_args!("{}", self.fn_().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GRXSTSP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp](index.html) module"]
pub struct GRXSTSP_SPEC;
impl crate::RegisterSpec for GRXSTSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxstsp::R](R) reader structure"]
impl crate::Readable for GRXSTSP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRXSTSP to value 0"]
impl crate::Resettable for GRXSTSP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
