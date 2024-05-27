///Register `XADDR` reader
pub type R = crate::R<XADDR_SPEC>;
///Field `OUT_CMDFIFO_XADDR` reader - only for debug
pub type OUT_CMDFIFO_XADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - only for debug
    #[inline(always)]
    pub fn out_cmdfifo_xaddr(&self) -> OUT_CMDFIFO_XADDR_R {
        OUT_CMDFIFO_XADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XADDR")
            .field("out_cmdfifo_xaddr", &self.out_cmdfifo_xaddr())
            .finish()
    }
}
/**TX CHx xaddr register

You can [`read`](crate::generic::Reg::read) this register and get [`xaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XADDR_SPEC;
impl crate::RegisterSpec for XADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`xaddr::R`](R) reader structure
impl crate::Readable for XADDR_SPEC {}
///`reset()` method sets XADDR to value 0
impl crate::Resettable for XADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
