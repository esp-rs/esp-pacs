///Register `HOST_SLCHOST_FUNC2_0` reader
pub type R = crate::R<HOST_SLCHOST_FUNC2_0_SPEC>;
///Register `HOST_SLCHOST_FUNC2_0` writer
pub type W = crate::W<HOST_SLCHOST_FUNC2_0_SPEC>;
///Field `HOST_SLC_FUNC2_INT` reader -
pub type HOST_SLC_FUNC2_INT_R = crate::BitReader;
///Field `HOST_SLC_FUNC2_INT` writer -
pub type HOST_SLC_FUNC2_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24
    #[inline(always)]
    pub fn host_slc_func2_int(&self) -> HOST_SLC_FUNC2_INT_R {
        HOST_SLC_FUNC2_INT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_FUNC2_0")
            .field("host_slc_func2_int", &self.host_slc_func2_int())
            .finish()
    }
}
impl W {
    ///Bit 24
    #[inline(always)]
    #[must_use]
    pub fn host_slc_func2_int(&mut self) -> HOST_SLC_FUNC2_INT_W<HOST_SLCHOST_FUNC2_0_SPEC> {
        HOST_SLC_FUNC2_INT_W::new(self, 24)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_func2_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_func2_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_FUNC2_0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_FUNC2_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_func2_0::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_FUNC2_0_SPEC {}
///`write(|w| ..)` method takes [`host_slchost_func2_0::W`](W) writer structure
impl crate::Writable for HOST_SLCHOST_FUNC2_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLCHOST_FUNC2_0 to value 0
impl crate::Resettable for HOST_SLCHOST_FUNC2_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
