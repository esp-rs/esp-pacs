#[doc = "Register `MEM_RX_STATUS` reader"]
pub type R = crate::R<MEM_RX_STATUS_SPEC>;
#[doc = "Field `MEM_RX_STATUS` reader - This register stores the current uart rx mem read address and rx mem write address"]
pub type MEM_RX_STATUS_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_RX_RD_ADDR` reader - This register stores the rx mem read address"]
pub type MEM_RX_RD_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_RX_WR_ADDR` reader - This register stores the rx mem write address"]
pub type MEM_RX_WR_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:23 - This register stores the current uart rx mem read address and rx mem write address"]
    #[inline(always)]
    pub fn mem_rx_status(&self) -> MEM_RX_STATUS_R {
        MEM_RX_STATUS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 2:12 - This register stores the rx mem read address"]
    #[inline(always)]
    pub fn mem_rx_rd_addr(&self) -> MEM_RX_RD_ADDR_R {
        MEM_RX_RD_ADDR_R::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 13:23 - This register stores the rx mem write address"]
    #[inline(always)]
    pub fn mem_rx_wr_addr(&self) -> MEM_RX_WR_ADDR_R {
        MEM_RX_WR_ADDR_R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RX_STATUS")
            .field("mem_rx_status", &self.mem_rx_status())
            .field("mem_rx_rd_addr", &self.mem_rx_rd_addr())
            .field("mem_rx_wr_addr", &self.mem_rx_wr_addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_RX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rx_status::R`](R) reader structure"]
impl crate::Readable for MEM_RX_STATUS_SPEC {}
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
