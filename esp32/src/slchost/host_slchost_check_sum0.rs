///Register `HOST_SLCHOST_CHECK_SUM0` reader
pub type R = crate::R<HOST_SLCHOST_CHECK_SUM0_SPEC>;
///Field `HOST_SLCHOST_CHECK_SUM0` reader -
pub type HOST_SLCHOST_CHECK_SUM0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn host_slchost_check_sum0(&self) -> HOST_SLCHOST_CHECK_SUM0_R {
        HOST_SLCHOST_CHECK_SUM0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CHECK_SUM0")
            .field("host_slchost_check_sum0", &self.host_slchost_check_sum0())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_check_sum0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_CHECK_SUM0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CHECK_SUM0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_check_sum0::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_CHECK_SUM0_SPEC {}
///`reset()` method sets HOST_SLCHOST_CHECK_SUM0 to value 0
impl crate::Resettable for HOST_SLCHOST_CHECK_SUM0_SPEC {
    const RESET_VALUE: u32 = 0;
}
