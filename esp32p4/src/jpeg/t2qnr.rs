///Register `T2QNR` reader
pub type R = crate::R<T2QNR_SPEC>;
///Field `T2_QNR_VAL` reader - write this reg to configure 64 quantization coefficient in t2 table
pub type T2_QNR_VAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - write this reg to configure 64 quantization coefficient in t2 table
    #[inline(always)]
    pub fn t2_qnr_val(&self) -> T2_QNR_VAL_R {
        T2_QNR_VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T2QNR")
            .field("t2_qnr_val", &self.t2_qnr_val())
            .finish()
    }
}
/**Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`t2qnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct T2QNR_SPEC;
impl crate::RegisterSpec for T2QNR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`t2qnr::R`](R) reader structure
impl crate::Readable for T2QNR_SPEC {}
///`reset()` method sets T2QNR to value 0
impl crate::Resettable for T2QNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
