///Register `FIFO_BCNT` reader
pub type R = crate::R<FIFO_BCNT_SPEC>;
///Field `OUT_CMDFIFO_OUTFIFO_BCNT` reader - only for debug
pub type OUT_CMDFIFO_OUTFIFO_BCNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - only for debug
    #[inline(always)]
    pub fn out_cmdfifo_outfifo_bcnt(&self) -> OUT_CMDFIFO_OUTFIFO_BCNT_R {
        OUT_CMDFIFO_OUTFIFO_BCNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_BCNT")
            .field("out_cmdfifo_outfifo_bcnt", &self.out_cmdfifo_outfifo_bcnt())
            .finish()
    }
}
/**TX CHx fifo byte cnt register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_bcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_BCNT_SPEC;
impl crate::RegisterSpec for FIFO_BCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo_bcnt::R`](R) reader structure
impl crate::Readable for FIFO_BCNT_SPEC {}
///`reset()` method sets FIFO_BCNT to value 0
impl crate::Resettable for FIFO_BCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
