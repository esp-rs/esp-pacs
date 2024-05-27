///Register `EXT_WAKEUP1` reader
pub type R = crate::R<EXT_WAKEUP1_SPEC>;
///Register `EXT_WAKEUP1` writer
pub type W = crate::W<EXT_WAKEUP1_SPEC>;
///Field `EXT_WAKEUP1_SEL` reader - Bitmap to select RTC pads for ext wakeup1
pub type EXT_WAKEUP1_SEL_R = crate::FieldReader<u32>;
///Field `EXT_WAKEUP1_SEL` writer - Bitmap to select RTC pads for ext wakeup1
pub type EXT_WAKEUP1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
///Field `EXT_WAKEUP1_STATUS_CLR` writer - clear ext wakeup1 status
pub type EXT_WAKEUP1_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:21 - Bitmap to select RTC pads for ext wakeup1
    #[inline(always)]
    pub fn ext_wakeup1_sel(&self) -> EXT_WAKEUP1_SEL_R {
        EXT_WAKEUP1_SEL_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP1")
            .field("ext_wakeup1_sel", &self.ext_wakeup1_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - Bitmap to select RTC pads for ext wakeup1
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_sel(&mut self) -> EXT_WAKEUP1_SEL_W<EXT_WAKEUP1_SPEC> {
        EXT_WAKEUP1_SEL_W::new(self, 0)
    }
    ///Bit 22 - clear ext wakeup1 status
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_status_clr(&mut self) -> EXT_WAKEUP1_STATUS_CLR_W<EXT_WAKEUP1_SPEC> {
        EXT_WAKEUP1_STATUS_CLR_W::new(self, 22)
    }
}
/**configure ext1 wakeup

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_WAKEUP1_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_wakeup1::R`](R) reader structure
impl crate::Readable for EXT_WAKEUP1_SPEC {}
///`write(|w| ..)` method takes [`ext_wakeup1::W`](W) writer structure
impl crate::Writable for EXT_WAKEUP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXT_WAKEUP1 to value 0
impl crate::Resettable for EXT_WAKEUP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
