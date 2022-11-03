#[doc = "Register `MEM_RX_STATUS` reader"]
pub struct R(crate::R<MEM_RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_SRAM_RADDR` reader - This register stores the offset read address in RX-SRAM."]
pub type RX_SRAM_RADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_SRAM_WADDR` reader - This register stores the offset write address in Rx-SRAM."]
pub type RX_SRAM_WADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 3:7 - This register stores the offset read address in RX-SRAM."]
    #[inline(always)]
    pub fn rx_sram_raddr(&self) -> RX_SRAM_RADDR_R {
        RX_SRAM_RADDR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - This register stores the offset write address in Rx-SRAM."]
    #[inline(always)]
    pub fn rx_sram_waddr(&self) -> RX_SRAM_WADDR_R {
        RX_SRAM_WADDR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
#[doc = "Rx-SRAM write and read offset address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_rx_status](index.html) module"]
pub struct MEM_RX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_rx_status::R](R) reader structure"]
impl crate::Readable for MEM_RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0x0001_0080"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0080;
}
