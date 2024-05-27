///Register `RTC_IO_DATE` reader
pub type R = crate::R<RTC_IO_DATE_SPEC>;
///Register `RTC_IO_DATE` writer
pub type W = crate::W<RTC_IO_DATE_SPEC>;
///Field `IO_DATE` reader - Version control register
pub type IO_DATE_R = crate::FieldReader<u32>;
///Field `IO_DATE` writer - Version control register
pub type IO_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - Version control register
    #[inline(always)]
    pub fn io_date(&self) -> IO_DATE_R {
        IO_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO_DATE")
            .field("io_date", &self.io_date())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - Version control register
    #[inline(always)]
    #[must_use]
    pub fn io_date(&mut self) -> IO_DATE_W<RTC_IO_DATE_SPEC> {
        IO_DATE_W::new(self, 0)
    }
}
/**Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_io_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_io_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTC_IO_DATE_SPEC;
impl crate::RegisterSpec for RTC_IO_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rtc_io_date::R`](R) reader structure
impl crate::Readable for RTC_IO_DATE_SPEC {}
///`write(|w| ..)` method takes [`rtc_io_date::W`](W) writer structure
impl crate::Writable for RTC_IO_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_IO_DATE to value 0x0190_3170
impl crate::Resettable for RTC_IO_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0190_3170;
}
