#[doc = "Register `HOST_SLCHOST_PKT_LEN` reader"]
pub type R = crate::R<HOST_SLCHOST_PKT_LEN_SPEC>;
#[doc = "Field `HOST_HOSTSLC0_LEN` reader - "]
pub type HOST_HOSTSLC0_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `HOST_HOSTSLC0_LEN_CHECK` reader - "]
pub type HOST_HOSTSLC0_LEN_CHECK_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len(&self) -> HOST_HOSTSLC0_LEN_R {
        HOST_HOSTSLC0_LEN_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn host_hostslc0_len_check(&self) -> HOST_HOSTSLC0_LEN_CHECK_R {
        HOST_HOSTSLC0_LEN_CHECK_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_PKT_LEN")
            .field(
                "host_hostslc0_len",
                &format_args!("{}", self.host_hostslc0_len().bits()),
            )
            .field(
                "host_hostslc0_len_check",
                &format_args!("{}", self.host_hostslc0_len_check().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_PKT_LEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_pkt_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_PKT_LEN_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_PKT_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_pkt_len::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN_SPEC {}
#[doc = "`reset()` method sets HOST_SLCHOST_PKT_LEN to value 0"]
impl crate::Resettable for HOST_SLCHOST_PKT_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
