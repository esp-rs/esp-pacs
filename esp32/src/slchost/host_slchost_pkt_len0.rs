#[doc = "Register `HOST_SLCHOST_PKT_LEN0` reader"]
pub type R = crate::R<HOST_SLCHOST_PKT_LEN0_SPEC>;
#[doc = "Field `HOST_HOSTSLC0_LEN0` reader - "]
pub type HOST_HOSTSLC0_LEN0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_hostslc0_len0(&self) -> HOST_HOSTSLC0_LEN0_R {
        HOST_HOSTSLC0_LEN0_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_PKT_LEN0")
            .field("host_hostslc0_len0", &self.host_hostslc0_len0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_pkt_len0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_PKT_LEN0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_PKT_LEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_pkt_len0::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN0_SPEC {}
#[doc = "`reset()` method sets HOST_SLCHOST_PKT_LEN0 to value 0"]
impl crate::Resettable for HOST_SLCHOST_PKT_LEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
