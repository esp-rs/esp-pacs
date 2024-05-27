///Register `DATE` reader
pub type R = crate::R<DATE_SPEC>;
///Field `DATE` reader - Version control register.
pub type DATE_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:29 - Version control register.
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE").field("date", &self.date()).finish()
    }
}
/**Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`date::R`](R) reader structure
impl crate::Readable for DATE_SPEC {}
///`reset()` method sets DATE to value 0x2019_0514
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x2019_0514;
}
