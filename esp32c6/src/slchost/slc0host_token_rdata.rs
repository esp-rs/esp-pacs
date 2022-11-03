#[doc = "Register `SLC0HOST_TOKEN_RDATA` reader"]
pub struct R(crate::R<SLC0HOST_TOKEN_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC0HOST_TOKEN_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC0HOST_TOKEN_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC0HOST_TOKEN_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_TOKEN0` reader - *******Description***********"]
pub type SLC0_TOKEN0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC0_RX_PF_VALID` reader - *******Description***********"]
pub type SLC0_RX_PF_VALID_R = crate::BitReader<bool>;
#[doc = "Field `HOSTSLCHOST_SLC0_TOKEN1` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_TOKEN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC0_RX_PF_EOF` reader - *******Description***********"]
pub type SLC0_RX_PF_EOF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_token0(&self) -> SLC0_TOKEN0_R {
        SLC0_TOKEN0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_rx_pf_valid(&self) -> SLC0_RX_PF_VALID_R {
        SLC0_RX_PF_VALID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_token1(&self) -> HOSTSLCHOST_SLC0_TOKEN1_R {
        HOSTSLCHOST_SLC0_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_rx_pf_eof(&self) -> SLC0_RX_PF_EOF_R {
        SLC0_RX_PF_EOF_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc0host_token_rdata](index.html) module"]
pub struct SLC0HOST_TOKEN_RDATA_SPEC;
impl crate::RegisterSpec for SLC0HOST_TOKEN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc0host_token_rdata::R](R) reader structure"]
impl crate::Readable for SLC0HOST_TOKEN_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLC0HOST_TOKEN_RDATA to value 0"]
impl crate::Resettable for SLC0HOST_TOKEN_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
