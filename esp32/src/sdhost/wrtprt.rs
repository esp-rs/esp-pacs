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
pub type WRITE_PROTECT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented."]
    #[inline(always)]
    pub fn write_protect(&self) -> WRITE_PROTECT_R {
        WRITE_PROTECT_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRTPRT")
            .field(
                "write_protect",
                &format_args!("{}", self.write_protect().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WRTPRT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
    const RESET_VALUE: Self::Ux = 0;
}
