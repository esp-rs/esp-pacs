#[doc = "Register `CH%s_RX_STATUS` reader"]
pub type R = crate::R<CH_RX_STATUS_SPEC>;
#[doc = "Field `MEM_WADDR_EX_CH` reader - Represents the memory address offset when receiver of channel %s is using the RAM."]
pub type MEM_WADDR_EX_CH_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_RADDR_CH` reader - Represents the memory address offset when reads RAM over APB bus."]
pub type APB_MEM_RADDR_CH_R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH` reader - Represents the FSM status of channel %s."]
pub type STATE_CH_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR_CH` reader - Represents whether the ownership of memory block is wrong. \\\\ 0: The ownership of memory block is correct\\\\ 1: The ownership of memory block is wrong\\\\"]
pub type MEM_OWNER_ERR_CH_R = crate::BitReader;
#[doc = "Field `MEM_FULL_CH` reader - Represents whether the receiver receives more data than the memory can fit. \\\\ 0: The receiver does not receive more data than the memory can fit\\\\ 1: The receiver receives more data than the memory can fit\\\\"]
pub type MEM_FULL_CH_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR_CH` reader - Represents whether the offset address exceeds memory size (overflows) when reads RAM via APB bus. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
pub type APB_MEM_RD_ERR_CH_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Represents the memory address offset when receiver of channel %s is using the RAM."]
    #[inline(always)]
    pub fn mem_waddr_ex_ch(&self) -> MEM_WADDR_EX_CH_R {
        MEM_WADDR_EX_CH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - Represents the memory address offset when reads RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_raddr_ch(&self) -> APB_MEM_RADDR_CH_R {
        APB_MEM_RADDR_CH_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:24 - Represents the FSM status of channel %s."]
    #[inline(always)]
    pub fn state_ch(&self) -> STATE_CH_R {
        STATE_CH_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - Represents whether the ownership of memory block is wrong. \\\\ 0: The ownership of memory block is correct\\\\ 1: The ownership of memory block is wrong\\\\"]
    #[inline(always)]
    pub fn mem_owner_err_ch(&self) -> MEM_OWNER_ERR_CH_R {
        MEM_OWNER_ERR_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents whether the receiver receives more data than the memory can fit. \\\\ 0: The receiver does not receive more data than the memory can fit\\\\ 1: The receiver receives more data than the memory can fit\\\\"]
    #[inline(always)]
    pub fn mem_full_ch(&self) -> MEM_FULL_CH_R {
        MEM_FULL_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents whether the offset address exceeds memory size (overflows) when reads RAM via APB bus. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch(&self) -> APB_MEM_RD_ERR_CH_R {
        APB_MEM_RD_ERR_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_STATUS")
            .field("mem_waddr_ex_ch", &self.mem_waddr_ex_ch())
            .field("apb_mem_raddr_ch", &self.apb_mem_raddr_ch())
            .field("state_ch", &self.state_ch())
            .field("mem_owner_err_ch", &self.mem_owner_err_ch())
            .field("mem_full_ch", &self.mem_full_ch())
            .field("apb_mem_rd_err_ch", &self.apb_mem_rd_err_ch())
            .finish()
    }
}
#[doc = "Channel %s status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_rx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_STATUS_SPEC;
impl crate::RegisterSpec for CH_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_status::R`](R) reader structure"]
impl crate::Readable for CH_RX_STATUS_SPEC {}
#[doc = "`reset()` method sets CH%s_RX_STATUS to value 0"]
impl crate::Resettable for CH_RX_STATUS_SPEC {}
