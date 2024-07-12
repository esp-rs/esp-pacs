#[doc = "Register `RX_CH2_COUNTER` reader"]
pub type R = crate::R<RX_CH2_COUNTER_SPEC>;
#[doc = "Field `RX_CH2_CNT` reader - rx ch2 counter register"]
pub type RX_CH2_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - rx ch2 counter register"]
    #[inline(always)]
    pub fn rx_ch2_cnt(&self) -> RX_CH2_CNT_R {
        RX_CH2_CNT_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH2_COUNTER")
            .field("rx_ch2_cnt", &self.rx_ch2_cnt())
            .finish()
    }
}
#[doc = "rx ch2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch2_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CH2_COUNTER_SPEC;
impl crate::RegisterSpec for RX_CH2_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch2_counter::R`](R) reader structure"]
impl crate::Readable for RX_CH2_COUNTER_SPEC {}
#[doc = "`reset()` method sets RX_CH2_COUNTER to value 0"]
impl crate::Resettable for RX_CH2_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
