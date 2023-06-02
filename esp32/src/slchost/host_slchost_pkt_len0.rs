#[doc = "Register `HOST_SLCHOST_PKT_LEN0` reader"]
pub struct R(crate::R<HOST_SLCHOST_PKT_LEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_PKT_LEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_PKT_LEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_PKT_LEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_HOSTSLC0_LEN0` reader - "]
pub type HOST_HOSTSLC0_LEN0_R = crate::FieldReader<u32, u32>;
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
            .field(
                "host_hostslc0_len0",
                &format_args!("{}", self.host_hostslc0_len0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_PKT_LEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_pkt_len0](index.html) module"]
pub struct HOST_SLCHOST_PKT_LEN0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_PKT_LEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_pkt_len0::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_PKT_LEN0 to value 0"]
impl crate::Resettable for HOST_SLCHOST_PKT_LEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
