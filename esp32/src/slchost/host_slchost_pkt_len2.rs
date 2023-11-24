#[doc = "Register `HOST_SLCHOST_PKT_LEN2` reader"]
pub type R = crate::R<HOST_SLCHOST_PKT_LEN2_SPEC>;
#[doc = "Field `HOST_HOSTSLC0_LEN2` reader - "]
pub type HOST_HOSTSLC0_LEN2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len2(&self) -> HOST_HOSTSLC0_LEN2_R {
        HOST_HOSTSLC0_LEN2_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_PKT_LEN2")
            .field(
                "host_hostslc0_len2",
                &format_args!("{}", self.host_hostslc0_len2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_PKT_LEN2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_pkt_len2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_PKT_LEN2_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_PKT_LEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_pkt_len2::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN2_SPEC {}
#[doc = "`reset()` method sets HOST_SLCHOST_PKT_LEN2 to value 0"]
impl crate::Resettable for HOST_SLCHOST_PKT_LEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
