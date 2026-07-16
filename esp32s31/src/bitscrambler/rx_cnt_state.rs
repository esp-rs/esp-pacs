#[doc = "Register `RX_CNT_STATE` reader"]
pub type R = crate::R<RX_CNT_STATE_SPEC>;
#[doc = "Field `TX_CNT_A_VALUE` reader - read the rx counter a value"]
pub type TX_CNT_A_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TX_CNT_B_VALUE` reader - read the rx counter b value"]
pub type TX_CNT_B_VALUE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - read the rx counter a value"]
    #[inline(always)]
    pub fn tx_cnt_a_value(&self) -> TX_CNT_A_VALUE_R {
        TX_CNT_A_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - read the rx counter b value"]
    #[inline(always)]
    pub fn tx_cnt_b_value(&self) -> TX_CNT_B_VALUE_R {
        TX_CNT_B_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CNT_STATE")
            .field("tx_cnt_a_value", &self.tx_cnt_a_value())
            .field("tx_cnt_b_value", &self.tx_cnt_b_value())
            .finish()
    }
}
#[doc = "RX Counter state registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cnt_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CNT_STATE_SPEC;
impl crate::RegisterSpec for RX_CNT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_cnt_state::R`](R) reader structure"]
impl crate::Readable for RX_CNT_STATE_SPEC {}
#[doc = "`reset()` method sets RX_CNT_STATE to value 0"]
impl crate::Resettable for RX_CNT_STATE_SPEC {}
