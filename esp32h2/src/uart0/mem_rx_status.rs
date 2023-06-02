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
pub type RX_SRAM_RADDR_R = crate::FieldReader;
#[doc = "Field `RX_SRAM_WADDR` reader - This register stores the offset write address in Rx-SRAM."]
pub type RX_SRAM_WADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This register stores the offset read address in RX-SRAM."]
    #[inline(always)]
    pub fn rx_sram_raddr(&self) -> RX_SRAM_RADDR_R {
        RX_SRAM_RADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - This register stores the offset write address in Rx-SRAM."]
    #[inline(always)]
    pub fn rx_sram_waddr(&self) -> RX_SRAM_WADDR_R {
        RX_SRAM_WADDR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RX_STATUS")
            .field(
                "rx_sram_raddr",
                &format_args!("{}", self.rx_sram_raddr().bits()),
            )
            .field(
                "rx_sram_waddr",
                &format_args!("{}", self.rx_sram_waddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_RX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
