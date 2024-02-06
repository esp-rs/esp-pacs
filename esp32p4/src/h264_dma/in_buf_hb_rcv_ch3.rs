#[doc = "Register `IN_BUF_HB_RCV_CH3` reader"]
pub type R = crate::R<IN_BUF_HB_RCV_CH3_SPEC>;
#[doc = "Field `IN_CMDFIFO_BUF_HB_RCV_CH3` reader - only for debug"]
pub type IN_CMDFIFO_BUF_HB_RCV_CH3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_buf_hb_rcv_ch3(&self) -> IN_CMDFIFO_BUF_HB_RCV_CH3_R {
        IN_CMDFIFO_BUF_HB_RCV_CH3_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_BUF_HB_RCV_CH3")
            .field(
                "in_cmdfifo_buf_hb_rcv_ch3",
                &format_args!("{}", self.in_cmdfifo_buf_hb_rcv_ch3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_BUF_HB_RCV_CH3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rx CH3 buf len hb rcv register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_buf_hb_rcv_ch3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_BUF_HB_RCV_CH3_SPEC;
impl crate::RegisterSpec for IN_BUF_HB_RCV_CH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_buf_hb_rcv_ch3::R`](R) reader structure"]
impl crate::Readable for IN_BUF_HB_RCV_CH3_SPEC {}
#[doc = "`reset()` method sets IN_BUF_HB_RCV_CH3 to value 0"]
impl crate::Resettable for IN_BUF_HB_RCV_CH3_SPEC {
    const RESET_VALUE: u32 = 0;
}
