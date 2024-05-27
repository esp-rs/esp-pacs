///Register `DLL_CONF` reader
pub type R = crate::R<DLL_CONF_SPEC>;
///Register `DLL_CONF` writer
pub type W = crate::W<DLL_CONF_SPEC>;
///Field `DLL_CAL_STOP` reader - Set 1 to stop calibration.
pub type DLL_CAL_STOP_R = crate::BitReader;
///Field `DLL_CAL_STOP` writer - Set 1 to stop calibration.
pub type DLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL_CAL_END` reader - 1 means calibration finished.
pub type DLL_CAL_END_R = crate::BitReader;
impl R {
    ///Bit 0 - Set 1 to stop calibration.
    #[inline(always)]
    pub fn dll_cal_stop(&self) -> DLL_CAL_STOP_R {
        DLL_CAL_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1 means calibration finished.
    #[inline(always)]
    pub fn dll_cal_end(&self) -> DLL_CAL_END_R {
        DLL_CAL_END_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_CONF")
            .field("dll_cal_stop", &self.dll_cal_stop())
            .field("dll_cal_end", &self.dll_cal_end())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to stop calibration.
    #[inline(always)]
    #[must_use]
    pub fn dll_cal_stop(&mut self) -> DLL_CAL_STOP_W<DLL_CONF_SPEC> {
        DLL_CAL_STOP_W::new(self, 0)
    }
}
/**SDIO DLL configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`dll_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DLL_CONF_SPEC;
impl crate::RegisterSpec for DLL_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dll_conf::R`](R) reader structure
impl crate::Readable for DLL_CONF_SPEC {}
///`write(|w| ..)` method takes [`dll_conf::W`](W) writer structure
impl crate::Writable for DLL_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DLL_CONF to value 0
impl crate::Resettable for DLL_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
