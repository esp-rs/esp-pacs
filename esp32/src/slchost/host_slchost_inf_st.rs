#[doc = "Register `HOST_SLCHOST_INF_ST` reader"]
pub type R = crate::R<HOST_SLCHOST_INF_ST_SPEC>;
#[doc = "Field `HOST_SDIO20_MODE` reader - "]
pub type HOST_SDIO20_MODE_R = crate::FieldReader;
#[doc = "Field `HOST_SDIO_NEG_SAMP` reader - "]
pub type HOST_SDIO_NEG_SAMP_R = crate::FieldReader;
#[doc = "Field `HOST_SDIO_QUICK_IN` reader - "]
pub type HOST_SDIO_QUICK_IN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn host_sdio20_mode(&self) -> HOST_SDIO20_MODE_R {
        HOST_SDIO20_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn host_sdio_neg_samp(&self) -> HOST_SDIO_NEG_SAMP_R {
        HOST_SDIO_NEG_SAMP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn host_sdio_quick_in(&self) -> HOST_SDIO_QUICK_IN_R {
        HOST_SDIO_QUICK_IN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_INF_ST")
            .field(
                "host_sdio20_mode",
                &format_args!("{}", self.host_sdio20_mode().bits()),
            )
            .field(
                "host_sdio_neg_samp",
                &format_args!("{}", self.host_sdio_neg_samp().bits()),
            )
            .field(
                "host_sdio_quick_in",
                &format_args!("{}", self.host_sdio_quick_in().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_INF_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_inf_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_INF_ST_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_INF_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_inf_st::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_INF_ST_SPEC {}
#[doc = "`reset()` method sets HOST_SLCHOST_INF_ST to value 0"]
impl crate::Resettable for HOST_SLCHOST_INF_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
