#[doc = "Register `HOST_SLC1HOST_TOKEN_RDATA` reader"]
pub type R = crate::R<HOST_SLC1HOST_TOKEN_RDATA_SPEC>;
#[doc = "Field `HOST_SLC1_TOKEN0` reader - "]
pub type HOST_SLC1_TOKEN0_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLC1_RX_PF_VALID` reader - "]
pub type HOST_SLC1_RX_PF_VALID_R = crate::BitReader;
#[doc = "Field `HOST_HOSTSLC1_TOKEN1` reader - "]
pub type HOST_HOSTSLC1_TOKEN1_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLC1_RX_PF_EOF` reader - "]
pub type HOST_SLC1_RX_PF_EOF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc1_token0(&self) -> HOST_SLC1_TOKEN0_R {
        HOST_SLC1_TOKEN0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc1_rx_pf_valid(&self) -> HOST_SLC1_RX_PF_VALID_R {
        HOST_SLC1_RX_PF_VALID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_hostslc1_token1(&self) -> HOST_HOSTSLC1_TOKEN1_R {
        HOST_HOSTSLC1_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn host_slc1_rx_pf_eof(&self) -> HOST_SLC1_RX_PF_EOF_R {
        HOST_SLC1_RX_PF_EOF_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC1HOST_TOKEN_RDATA")
            .field("host_slc1_token0", &self.host_slc1_token0())
            .field("host_slc1_rx_pf_valid", &self.host_slc1_rx_pf_valid())
            .field("host_hostslc1_token1", &self.host_hostslc1_token1())
            .field("host_slc1_rx_pf_eof", &self.host_slc1_rx_pf_eof())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slc1host_token_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC1HOST_TOKEN_RDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_TOKEN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc1host_token_rdata::R`](R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_TOKEN_RDATA_SPEC {}
#[doc = "`reset()` method sets HOST_SLC1HOST_TOKEN_RDATA to value 0"]
impl crate::Resettable for HOST_SLC1HOST_TOKEN_RDATA_SPEC {}
