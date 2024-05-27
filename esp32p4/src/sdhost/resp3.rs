///Register `RESP3` reader
pub type R = crate::R<RESP3_SPEC>;
///Field `RESPONSE3` reader - Bit\[127:96\] of long response.
pub type RESPONSE3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bit\[127:96\] of long response.
    #[inline(always)]
    pub fn response3(&self) -> RESPONSE3_R {
        RESPONSE3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP3")
            .field("response3", &self.response3())
            .finish()
    }
}
/**Long response data register

You can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`resp3::R`](R) reader structure
impl crate::Readable for RESP3_SPEC {}
///`reset()` method sets RESP3 to value 0
impl crate::Resettable for RESP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
