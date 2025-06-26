#[doc = "Register `MEM_CNT_STATUS` reader"]
pub type R = crate::R<MEM_CNT_STATUS_SPEC>;
#[doc = "Field `RX_MEM_CNT` reader - Refer to the rxfifo_cnt's description."]
pub type RX_MEM_CNT_R = crate::FieldReader;
#[doc = "Field `TX_MEM_CNT` reader - Refer to the txfifo_cnt's description."]
pub type TX_MEM_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Refer to the rxfifo_cnt's description."]
    #[inline(always)]
    pub fn rx_mem_cnt(&self) -> RX_MEM_CNT_R {
        RX_MEM_CNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Refer to the txfifo_cnt's description."]
    #[inline(always)]
    pub fn tx_mem_cnt(&self) -> TX_MEM_CNT_R {
        TX_MEM_CNT_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CNT_STATUS")
            .field("rx_mem_cnt", &self.rx_mem_cnt())
            .field("tx_mem_cnt", &self.tx_mem_cnt())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cnt_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CNT_STATUS_SPEC;
impl crate::RegisterSpec for MEM_CNT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_cnt_status::R`](R) reader structure"]
impl crate::Readable for MEM_CNT_STATUS_SPEC {}
#[doc = "`reset()` method sets MEM_CNT_STATUS to value 0"]
impl crate::Resettable for MEM_CNT_STATUS_SPEC {}
