///Register `DATE` reader
pub type R = crate::R<DATE_SPEC>;
///Register `DATE` writer
pub type W = crate::W<DATE_SPEC>;
///Field `IO_DATE` reader - date
pub type IO_DATE_R = crate::FieldReader<u32>;
///Field `IO_DATE` writer - date
pub type IO_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - date
    #[inline(always)]
    pub fn io_date(&self) -> IO_DATE_R {
        IO_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("io_date", &self.io_date())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - date
    #[inline(always)]
    #[must_use]
    pub fn io_date(&mut self) -> IO_DATE_W<DATE_SPEC> {
        IO_DATE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`date::R`](R) reader structure
impl crate::Readable for DATE_SPEC {}
///`write(|w| ..)` method takes [`date::W`](W) writer structure
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATE to value 0x0160_3160
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0160_3160;
}
