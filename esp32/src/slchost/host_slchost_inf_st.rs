///Register `HOST_SLCHOST_INF_ST` reader
pub type R = crate::R<HOST_SLCHOST_INF_ST_SPEC>;
///Field `HOST_SDIO20_MODE` reader -
pub type HOST_SDIO20_MODE_R = crate::FieldReader;
///Field `HOST_SDIO_NEG_SAMP` reader -
pub type HOST_SDIO_NEG_SAMP_R = crate::FieldReader;
///Field `HOST_SDIO_QUICK_IN` reader -
pub type HOST_SDIO_QUICK_IN_R = crate::FieldReader;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn host_sdio20_mode(&self) -> HOST_SDIO20_MODE_R {
        HOST_SDIO20_MODE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9
    #[inline(always)]
    pub fn host_sdio_neg_samp(&self) -> HOST_SDIO_NEG_SAMP_R {
        HOST_SDIO_NEG_SAMP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14
    #[inline(always)]
    pub fn host_sdio_quick_in(&self) -> HOST_SDIO_QUICK_IN_R {
        HOST_SDIO_QUICK_IN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_INF_ST")
            .field("host_sdio20_mode", &self.host_sdio20_mode())
            .field("host_sdio_neg_samp", &self.host_sdio_neg_samp())
            .field("host_sdio_quick_in", &self.host_sdio_quick_in())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_inf_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_INF_ST_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_INF_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_inf_st::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_INF_ST_SPEC {}
///`reset()` method sets HOST_SLCHOST_INF_ST to value 0
impl crate::Resettable for HOST_SLCHOST_INF_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
