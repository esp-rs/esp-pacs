#[doc = "Register `GRXSTSR` reader"]
pub struct R(crate::R<GRXSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `G_CHNUM` reader - "]
pub type G_CHNUM_R = crate::FieldReader;
#[doc = "Field `G_BCNT` reader - "]
pub type G_BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `G_DPID` reader - "]
pub type G_DPID_R = crate::FieldReader;
#[doc = "Field `G_PKTSTS` reader - "]
pub type G_PKTSTS_R = crate::FieldReader;
#[doc = "Field `G_FN` reader - "]
pub type G_FN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn g_chnum(&self) -> G_CHNUM_R {
        G_CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14"]
    #[inline(always)]
    pub fn g_bcnt(&self) -> G_BCNT_R {
        G_BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn g_dpid(&self) -> G_DPID_R {
        G_DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20"]
    #[inline(always)]
    pub fn g_pktsts(&self) -> G_PKTSTS_R {
        G_PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn g_fn(&self) -> G_FN_R {
        G_FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR")
            .field("g_chnum", &format_args!("{}", self.g_chnum().bits()))
            .field("g_bcnt", &format_args!("{}", self.g_bcnt().bits()))
            .field("g_dpid", &format_args!("{}", self.g_dpid().bits()))
            .field("g_pktsts", &format_args!("{}", self.g_pktsts().bits()))
            .field("g_fn", &format_args!("{}", self.g_fn().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GRXSTSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsr](index.html) module"]
pub struct GRXSTSR_SPEC;
impl crate::RegisterSpec for GRXSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxstsr::R](R) reader structure"]
impl crate::Readable for GRXSTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRXSTSR to value 0"]
impl crate::Resettable for GRXSTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
