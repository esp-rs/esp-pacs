#[doc = "Register `TX_CNT_STATE` reader"]
pub type R = crate::R<TX_CNT_STATE_SPEC>;
#[doc = "Field `RX_CNT_A_VALUE` reader - read the tx counter a value"]
pub type RX_CNT_A_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `RX_CNT_B_VALUE` reader - read the tx counter b value"]
pub type RX_CNT_B_VALUE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - read the tx counter a value"]
    #[inline(always)]
    pub fn rx_cnt_a_value(&self) -> RX_CNT_A_VALUE_R {
        RX_CNT_A_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - read the tx counter b value"]
    #[inline(always)]
    pub fn rx_cnt_b_value(&self) -> RX_CNT_B_VALUE_R {
        RX_CNT_B_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CNT_STATE")
            .field("rx_cnt_a_value", &self.rx_cnt_a_value())
            .field("rx_cnt_b_value", &self.rx_cnt_b_value())
            .finish()
    }
}
#[doc = "TX Counter state registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cnt_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CNT_STATE_SPEC;
impl crate::RegisterSpec for TX_CNT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cnt_state::R`](R) reader structure"]
impl crate::Readable for TX_CNT_STATE_SPEC {}
#[doc = "`reset()` method sets TX_CNT_STATE to value 0"]
impl crate::Resettable for TX_CNT_STATE_SPEC {}
