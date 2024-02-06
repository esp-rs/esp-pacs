#[doc = "Register `IN_POP_DATA_CNT_CH2` reader"]
pub type R = crate::R<IN_POP_DATA_CNT_CH2_SPEC>;
#[doc = "Field `IN_CMDFIFO_POP_DATA_CNT_CH2` reader - only for debug"]
pub type IN_CMDFIFO_POP_DATA_CNT_CH2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_pop_data_cnt_ch2(&self) -> IN_CMDFIFO_POP_DATA_CNT_CH2_R {
        IN_CMDFIFO_POP_DATA_CNT_CH2_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_POP_DATA_CNT_CH2")
            .field(
                "in_cmdfifo_pop_data_cnt_ch2",
                &format_args!("{}", self.in_cmdfifo_pop_data_cnt_ch2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_POP_DATA_CNT_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rx CH2 pop data cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_data_cnt_ch2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_POP_DATA_CNT_CH2_SPEC;
impl crate::RegisterSpec for IN_POP_DATA_CNT_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pop_data_cnt_ch2::R`](R) reader structure"]
impl crate::Readable for IN_POP_DATA_CNT_CH2_SPEC {}
#[doc = "`reset()` method sets IN_POP_DATA_CNT_CH2 to value 0x07"]
impl crate::Resettable for IN_POP_DATA_CNT_CH2_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
