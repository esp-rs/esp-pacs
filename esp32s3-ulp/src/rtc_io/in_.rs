///Register `IN` reader
pub type R = crate::R<IN_SPEC>;
///Field `NEXT` reader - RTC GPIO input data
pub type NEXT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 10:31 - RTC GPIO input data
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN").field("next", &self.next()).finish()
    }
}
/**RTC GPIO input data

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`in_::R`](R) reader structure
impl crate::Readable for IN_SPEC {}
///`reset()` method sets IN to value 0
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
