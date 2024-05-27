///Register `BUF_HB_RCV` reader
pub type R = crate::R<BUF_HB_RCV_SPEC>;
///Field `IN_CMDFIFO_BUF_HB_RCV` reader - only for debug
pub type IN_CMDFIFO_BUF_HB_RCV_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:28 - only for debug
    #[inline(always)]
    pub fn in_cmdfifo_buf_hb_rcv(&self) -> IN_CMDFIFO_BUF_HB_RCV_R {
        IN_CMDFIFO_BUF_HB_RCV_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUF_HB_RCV")
            .field("in_cmdfifo_buf_hb_rcv", &self.in_cmdfifo_buf_hb_rcv())
            .finish()
    }
}
/**RX CH0 buf len hb rcv register

You can [`read`](crate::generic::Reg::read) this register and get [`buf_hb_rcv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUF_HB_RCV_SPEC;
impl crate::RegisterSpec for BUF_HB_RCV_SPEC {
    type Ux = u32;
}
///`read()` method returns [`buf_hb_rcv::R`](R) reader structure
impl crate::Readable for BUF_HB_RCV_SPEC {}
///`reset()` method sets BUF_HB_RCV to value 0
impl crate::Resettable for BUF_HB_RCV_SPEC {
    const RESET_VALUE: u32 = 0;
}
