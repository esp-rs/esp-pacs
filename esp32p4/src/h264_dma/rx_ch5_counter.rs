#[doc = "Register `RX_CH5_COUNTER` reader"]
pub type R = crate::R<RX_CH5_COUNTER_SPEC>;
#[doc = "Field `RX_CH5_CNT` reader - rx ch5 counter register"]
pub type RX_CH5_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - rx ch5 counter register"]
    #[inline(always)]
    pub fn rx_ch5_cnt(&self) -> RX_CH5_CNT_R {
        RX_CH5_CNT_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH5_COUNTER")
            .field("rx_ch5_cnt", &self.rx_ch5_cnt().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CH5_COUNTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rx ch5 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ch5_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CH5_COUNTER_SPEC;
impl crate::RegisterSpec for RX_CH5_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch5_counter::R`](R) reader structure"]
impl crate::Readable for RX_CH5_COUNTER_SPEC {}
#[doc = "`reset()` method sets RX_CH5_COUNTER to value 0"]
impl crate::Resettable for RX_CH5_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
