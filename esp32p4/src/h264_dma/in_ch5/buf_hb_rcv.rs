#[doc = "Register `BUF_HB_RCV` reader"]
pub type R = crate::R<BUF_HB_RCV_SPEC>;
#[doc = "Field `IN_CMDFIFO_BUF_HB_RCV` reader - only for debug"]
pub type IN_CMDFIFO_BUF_HB_RCV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_buf_hb_rcv(&self) -> IN_CMDFIFO_BUF_HB_RCV_R {
        IN_CMDFIFO_BUF_HB_RCV_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUF_HB_RCV")
            .field(
                "in_cmdfifo_buf_hb_rcv",
                &format_args!("{}", self.in_cmdfifo_buf_hb_rcv().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUF_HB_RCV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rx CH5 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_hb_rcv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_HB_RCV_SPEC;
impl crate::RegisterSpec for BUF_HB_RCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_hb_rcv::R`](R) reader structure"]
impl crate::Readable for BUF_HB_RCV_SPEC {}
#[doc = "`reset()` method sets BUF_HB_RCV to value 0"]
impl crate::Resettable for BUF_HB_RCV_SPEC {
    const RESET_VALUE: u32 = 0;
}
