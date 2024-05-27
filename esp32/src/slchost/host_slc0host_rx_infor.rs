///Register `HOST_SLC0HOST_RX_INFOR` reader
pub type R = crate::R<HOST_SLC0HOST_RX_INFOR_SPEC>;
///Register `HOST_SLC0HOST_RX_INFOR` writer
pub type W = crate::W<HOST_SLC0HOST_RX_INFOR_SPEC>;
///Field `HOST_SLC0HOST_RX_INFOR` reader -
pub type HOST_SLC0HOST_RX_INFOR_R = crate::FieldReader<u32>;
///Field `HOST_SLC0HOST_RX_INFOR` writer -
pub type HOST_SLC0HOST_RX_INFOR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn host_slc0host_rx_infor(&self) -> HOST_SLC0HOST_RX_INFOR_R {
        HOST_SLC0HOST_RX_INFOR_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC0HOST_RX_INFOR")
            .field("host_slc0host_rx_infor", &self.host_slc0host_rx_infor())
            .finish()
    }
}
impl W {
    ///Bits 0:19
    #[inline(always)]
    #[must_use]
    pub fn host_slc0host_rx_infor(
        &mut self,
    ) -> HOST_SLC0HOST_RX_INFOR_W<HOST_SLC0HOST_RX_INFOR_SPEC> {
        HOST_SLC0HOST_RX_INFOR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_rx_infor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_rx_infor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLC0HOST_RX_INFOR_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_RX_INFOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slc0host_rx_infor::R`](R) reader structure
impl crate::Readable for HOST_SLC0HOST_RX_INFOR_SPEC {}
///`write(|w| ..)` method takes [`host_slc0host_rx_infor::W`](W) writer structure
impl crate::Writable for HOST_SLC0HOST_RX_INFOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLC0HOST_RX_INFOR to value 0
impl crate::Resettable for HOST_SLC0HOST_RX_INFOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
