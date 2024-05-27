///Register `HOST_SLC1HOST_TOKEN_WDATA` reader
pub type R = crate::R<HOST_SLC1HOST_TOKEN_WDATA_SPEC>;
///Register `HOST_SLC1HOST_TOKEN_WDATA` writer
pub type W = crate::W<HOST_SLC1HOST_TOKEN_WDATA_SPEC>;
///Field `HOST_SLC1HOST_TOKEN0_WD` reader -
pub type HOST_SLC1HOST_TOKEN0_WD_R = crate::FieldReader<u16>;
///Field `HOST_SLC1HOST_TOKEN0_WD` writer -
pub type HOST_SLC1HOST_TOKEN0_WD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HOST_SLC1HOST_TOKEN1_WD` reader -
pub type HOST_SLC1HOST_TOKEN1_WD_R = crate::FieldReader<u16>;
///Field `HOST_SLC1HOST_TOKEN1_WD` writer -
pub type HOST_SLC1HOST_TOKEN1_WD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11
    #[inline(always)]
    pub fn host_slc1host_token0_wd(&self) -> HOST_SLC1HOST_TOKEN0_WD_R {
        HOST_SLC1HOST_TOKEN0_WD_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27
    #[inline(always)]
    pub fn host_slc1host_token1_wd(&self) -> HOST_SLC1HOST_TOKEN1_WD_R {
        HOST_SLC1HOST_TOKEN1_WD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC1HOST_TOKEN_WDATA")
            .field("host_slc1host_token0_wd", &self.host_slc1host_token0_wd())
            .field("host_slc1host_token1_wd", &self.host_slc1host_token1_wd())
            .finish()
    }
}
impl W {
    ///Bits 0:11
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token0_wd(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN0_WD_W<HOST_SLC1HOST_TOKEN_WDATA_SPEC> {
        HOST_SLC1HOST_TOKEN0_WD_W::new(self, 0)
    }
    ///Bits 16:27
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_token1_wd(
        &mut self,
    ) -> HOST_SLC1HOST_TOKEN1_WD_W<HOST_SLC1HOST_TOKEN_WDATA_SPEC> {
        HOST_SLC1HOST_TOKEN1_WD_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_token_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_token_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLC1HOST_TOKEN_WDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_TOKEN_WDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slc1host_token_wdata::R`](R) reader structure
impl crate::Readable for HOST_SLC1HOST_TOKEN_WDATA_SPEC {}
///`write(|w| ..)` method takes [`host_slc1host_token_wdata::W`](W) writer structure
impl crate::Writable for HOST_SLC1HOST_TOKEN_WDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLC1HOST_TOKEN_WDATA to value 0
impl crate::Resettable for HOST_SLC1HOST_TOKEN_WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
