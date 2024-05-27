///Register `TOUCH_APPROACH` reader
pub type R = crate::R<TOUCH_APPROACH_SPEC>;
///Register `TOUCH_APPROACH` writer
pub type W = crate::W<TOUCH_APPROACH_SPEC>;
///Field `TOUCH_SLP_CHANNEL_CLR` writer - clear touch slp channel
pub type TOUCH_SLP_CHANNEL_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_APPROACH_MEAS_TIME` reader - approach pads total meas times
pub type TOUCH_APPROACH_MEAS_TIME_R = crate::FieldReader;
///Field `TOUCH_APPROACH_MEAS_TIME` writer - approach pads total meas times
pub type TOUCH_APPROACH_MEAS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 24:31 - approach pads total meas times
    #[inline(always)]
    pub fn touch_approach_meas_time(&self) -> TOUCH_APPROACH_MEAS_TIME_R {
        TOUCH_APPROACH_MEAS_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_APPROACH")
            .field("touch_approach_meas_time", &self.touch_approach_meas_time())
            .finish()
    }
}
impl W {
    ///Bit 23 - clear touch slp channel
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_channel_clr(&mut self) -> TOUCH_SLP_CHANNEL_CLR_W<TOUCH_APPROACH_SPEC> {
        TOUCH_SLP_CHANNEL_CLR_W::new(self, 23)
    }
    ///Bits 24:31 - approach pads total meas times
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_meas_time(&mut self) -> TOUCH_APPROACH_MEAS_TIME_W<TOUCH_APPROACH_SPEC> {
        TOUCH_APPROACH_MEAS_TIME_W::new(self, 24)
    }
}
/**configure touch controller

You can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_APPROACH_SPEC;
impl crate::RegisterSpec for TOUCH_APPROACH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_approach::R`](R) reader structure
impl crate::Readable for TOUCH_APPROACH_SPEC {}
///`write(|w| ..)` method takes [`touch_approach::W`](W) writer structure
impl crate::Writable for TOUCH_APPROACH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_APPROACH to value 0x5000_0000
impl crate::Resettable for TOUCH_APPROACH_SPEC {
    const RESET_VALUE: u32 = 0x5000_0000;
}
