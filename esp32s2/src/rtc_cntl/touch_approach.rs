#[doc = "Register `TOUCH_APPROACH` reader"]
pub type R = crate::R<TOUCH_APPROACH_SPEC>;
#[doc = "Register `TOUCH_APPROACH` writer"]
pub type W = crate::W<TOUCH_APPROACH_SPEC>;
#[doc = "Field `TOUCH_SLP_CHANNEL_CLR` writer - Clear touch sleep channel."]
pub type TOUCH_SLP_CHANNEL_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEAS_TIME` reader - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
pub type MEAS_TIME_R = crate::FieldReader;
#[doc = "Field `MEAS_TIME` writer - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
pub type MEAS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31 - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
    #[inline(always)]
    pub fn meas_time(&self) -> MEAS_TIME_R {
        MEAS_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_APPROACH")
            .field("meas_time", &self.meas_time())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - Clear touch sleep channel."]
    #[inline(always)]
    pub fn touch_slp_channel_clr(&mut self) -> TOUCH_SLP_CHANNEL_CLR_W<'_, TOUCH_APPROACH_SPEC> {
        TOUCH_SLP_CHANNEL_CLR_W::new(self, 23)
    }
    #[doc = "Bits 24:31 - Set the total measurement times for the pads in proximity mode. Range: 0 – 255."]
    #[inline(always)]
    pub fn meas_time(&mut self) -> MEAS_TIME_W<'_, TOUCH_APPROACH_SPEC> {
        MEAS_TIME_W::new(self, 24)
    }
}
#[doc = "Configure touch approach settings\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_approach::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_approach::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_APPROACH_SPEC;
impl crate::RegisterSpec for TOUCH_APPROACH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_approach::R`](R) reader structure"]
impl crate::Readable for TOUCH_APPROACH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_approach::W`](W) writer structure"]
impl crate::Writable for TOUCH_APPROACH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_APPROACH to value 0x5000_0000"]
impl crate::Resettable for TOUCH_APPROACH_SPEC {
    const RESET_VALUE: u32 = 0x5000_0000;
}
