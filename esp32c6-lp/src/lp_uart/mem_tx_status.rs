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
pub type TX_SRAM_WADDR_R = crate::FieldReader;
#[doc = "Field `TX_SRAM_RADDR` reader - This register stores the offset read address in Tx-SRAM."]
pub type TX_SRAM_RADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 3:7 - This register stores the offset write address in Tx-SRAM."]
    #[inline(always)]
    pub fn tx_sram_waddr(&self) -> TX_SRAM_WADDR_R {
        TX_SRAM_WADDR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - This register stores the offset read address in Tx-SRAM."]
    #[inline(always)]
    pub fn tx_sram_raddr(&self) -> TX_SRAM_RADDR_R {
        TX_SRAM_RADDR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_TX_STATUS")
            .field(
                "tx_sram_waddr",
                &format_args!("{}", self.tx_sram_waddr().bits()),
            )
            .field(
                "tx_sram_raddr",
                &format_args!("{}", self.tx_sram_raddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_TX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
