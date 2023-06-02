#[doc = "Register `HOST_SLC0HOST_TOKEN_RDATA` reader"]
pub struct R(crate::R<HOST_SLC0HOST_TOKEN_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0HOST_TOKEN_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0HOST_TOKEN_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0HOST_TOKEN_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLC0_TOKEN0` reader - "]
pub type HOST_SLC0_TOKEN0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOST_SLC0_RX_PF_VALID` reader - "]
pub type HOST_SLC0_RX_PF_VALID_R = crate::BitReader;
#[doc = "Field `HOST_HOSTSLC0_TOKEN1` reader - "]
pub type HOST_HOSTSLC0_TOKEN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOST_SLC0_RX_PF_EOF` reader - "]
pub type HOST_SLC0_RX_PF_EOF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0_token0(&self) -> HOST_SLC0_TOKEN0_R {
        HOST_SLC0_TOKEN0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid(&self) -> HOST_SLC0_RX_PF_VALID_R {
        HOST_SLC0_RX_PF_VALID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_hostslc0_token1(&self) -> HOST_HOSTSLC0_TOKEN1_R {
        HOST_HOSTSLC0_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_eof(&self) -> HOST_SLC0_RX_PF_EOF_R {
        HOST_SLC0_RX_PF_EOF_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC0HOST_TOKEN_RDATA")
            .field(
                "host_slc0_token0",
                &format_args!("{}", self.host_slc0_token0().bits()),
            )
            .field(
                "host_slc0_rx_pf_valid",
                &format_args!("{}", self.host_slc0_rx_pf_valid().bit()),
            )
            .field(
                "host_hostslc0_token1",
                &format_args!("{}", self.host_hostslc0_token1().bits()),
            )
            .field(
                "host_slc0_rx_pf_eof",
                &format_args!("{}", self.host_slc0_rx_pf_eof().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC0HOST_TOKEN_RDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_token_rdata](index.html) module"]
pub struct HOST_SLC0HOST_TOKEN_RDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_TOKEN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0host_token_rdata::R](R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_TOKEN_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_TOKEN_RDATA to value 0"]
impl crate::Resettable for HOST_SLC0HOST_TOKEN_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
