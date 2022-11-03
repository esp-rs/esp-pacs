#[doc = "Register `MEM_TX_STATUS` reader"]
pub struct R(crate::R<MEM_TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_SRAM_WADDR` reader - This register stores the offset write address in Tx-SRAM."]
pub type TX_SRAM_WADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SRAM_RADDR` reader - This register stores the offset read address in Tx-SRAM."]
pub type TX_SRAM_RADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - This register stores the offset write address in Tx-SRAM."]
    #[inline(always)]
    pub fn tx_sram_waddr(&self) -> TX_SRAM_WADDR_R {
        TX_SRAM_WADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - This register stores the offset read address in Tx-SRAM."]
    #[inline(always)]
    pub fn tx_sram_raddr(&self) -> TX_SRAM_RADDR_R {
        TX_SRAM_RADDR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[doc = "Tx-SRAM write and read offset address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_tx_status](index.html) module"]
pub struct MEM_TX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_tx_status::R](R) reader structure"]
impl crate::Readable for MEM_TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_TX_STATUS to value 0"]
impl crate::Resettable for MEM_TX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
