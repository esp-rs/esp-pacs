#[doc = "Register `RX_CH0_COUNTER` reader"]
pub type R = crate::R<RX_CH0_COUNTER_SPEC>;
#[doc = "Field `RX_CH0_CNT` reader - rx ch0 counter register"]
pub type RX_CH0_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:22 - rx ch0 counter register"]
    #[inline(always)]
    pub fn rx_ch0_cnt(&self) -> RX_CH0_CNT_R {
        RX_CH0_CNT_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH0_COUNTER")
            .field("rx_ch0_cnt", &self.rx_ch0_cnt())
            .finish()
    }
}
#[doc = "rx ch0 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch0_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CH0_COUNTER_SPEC;
impl crate::RegisterSpec for RX_CH0_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch0_counter::R`](R) reader structure"]
impl crate::Readable for RX_CH0_COUNTER_SPEC {}
#[doc = "`reset()` method sets RX_CH0_COUNTER to value 0"]
impl crate::Resettable for RX_CH0_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
