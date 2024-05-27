///Register `IN` reader
pub type R = crate::R<IN_SPEC>;
///Field `NEXT` reader - GPIO0~17 input value
pub type NEXT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 14:31 - GPIO0~17 input value
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN").field("next", &self.next()).finish()
    }
}
/**

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
