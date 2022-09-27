#[doc = "Register `WRTPRT` reader"]
pub struct R(crate::R<WRTPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRTPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRTPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRTPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRITE_PROTECT` reader - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented."]
pub type WRITE_PROTECT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented."]
    #[inline(always)]
    pub fn write_protect(&self) -> WRITE_PROTECT_R {
        WRITE_PROTECT_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Card write protection (WP) status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtprt](index.html) module"]
pub struct WRTPRT_SPEC;
impl crate::RegisterSpec for WRTPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrtprt::R](R) reader structure"]
impl crate::Readable for WRTPRT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRTPRT to value 0"]
impl crate::Resettable for WRTPRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
