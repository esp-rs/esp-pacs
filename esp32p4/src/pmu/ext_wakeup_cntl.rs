///Register `EXT_WAKEUP_CNTL` reader
pub type R = crate::R<EXT_WAKEUP_CNTL_SPEC>;
///Register `EXT_WAKEUP_CNTL` writer
pub type W = crate::W<EXT_WAKEUP_CNTL_SPEC>;
///Field `EXT_WAKEUP_STATUS_CLR` reader - need_des
pub type EXT_WAKEUP_STATUS_CLR_R = crate::BitReader;
///Field `EXT_WAKEUP_STATUS_CLR` writer - need_des
pub type EXT_WAKEUP_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXT_WAKEUP_FILTER` reader - need_des
pub type EXT_WAKEUP_FILTER_R = crate::BitReader;
///Field `EXT_WAKEUP_FILTER` writer - need_des
pub type EXT_WAKEUP_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn ext_wakeup_status_clr(&self) -> EXT_WAKEUP_STATUS_CLR_R {
        EXT_WAKEUP_STATUS_CLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn ext_wakeup_filter(&self) -> EXT_WAKEUP_FILTER_R {
        EXT_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CNTL")
            .field("ext_wakeup_status_clr", &self.ext_wakeup_status_clr())
            .field("ext_wakeup_filter", &self.ext_wakeup_filter())
            .finish()
    }
}
impl W {
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_status_clr(&mut self) -> EXT_WAKEUP_STATUS_CLR_W<EXT_WAKEUP_CNTL_SPEC> {
        EXT_WAKEUP_STATUS_CLR_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_filter(&mut self) -> EXT_WAKEUP_FILTER_W<EXT_WAKEUP_CNTL_SPEC> {
        EXT_WAKEUP_FILTER_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_WAKEUP_CNTL_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CNTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_wakeup_cntl::R`](R) reader structure
impl crate::Readable for EXT_WAKEUP_CNTL_SPEC {}
///`write(|w| ..)` method takes [`ext_wakeup_cntl::W`](W) writer structure
impl crate::Writable for EXT_WAKEUP_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXT_WAKEUP_CNTL to value 0
impl crate::Resettable for EXT_WAKEUP_CNTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
