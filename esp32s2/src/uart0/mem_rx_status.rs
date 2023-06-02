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
#[doc = "Field `APB_RX_RADDR` reader - This register stores the offset address in RX_FIFO when software reads data from RX FIFO via APB."]
pub type APB_RX_RADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_WADDR` reader - This register stores the offset address in RX FIFO when Rx_FIFO_Ctrl writes RX FIFO."]
pub type RX_WADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - This register stores the offset address in RX_FIFO when software reads data from RX FIFO via APB."]
    #[inline(always)]
    pub fn apb_rx_raddr(&self) -> APB_RX_RADDR_R {
        APB_RX_RADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - This register stores the offset address in RX FIFO when Rx_FIFO_Ctrl writes RX FIFO."]
    #[inline(always)]
    pub fn rx_waddr(&self) -> RX_WADDR_R {
        RX_WADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RX_STATUS")
            .field(
                "apb_rx_raddr",
                &format_args!("{}", self.apb_rx_raddr().bits()),
            )
            .field("rx_waddr", &format_args!("{}", self.rx_waddr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_RX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RX FIFO write and read offset address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_rx_status](index.html) module"]
pub struct MEM_RX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_rx_status::R](R) reader structure"]
impl crate::Readable for MEM_RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
