#[doc = "Register `CH%s_TX_STATUS` reader"]
pub type R = crate::R<CH_TX_STATUS_SPEC>;
#[doc = "Field `MEM_RADDR_EX_CH` reader - Represents the memory address offset when transmitter of channel %s is using the RAM."]
pub type MEM_RADDR_EX_CH_R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH` reader - Represents the FSM status of channel %s."]
pub type STATE_CH_R = crate::FieldReader;
#[doc = "Field `APB_MEM_WADDR_CH` reader - Represents the memory address offset when writes RAM over APB bus."]
pub type APB_MEM_WADDR_CH_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_RD_ERR_CH` reader - Represents whether the offset address exceeds memory size when reading via APB bus. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
pub type APB_MEM_RD_ERR_CH_R = crate::BitReader;
#[doc = "Field `MEM_EMPTY_CH` reader - Represents whether the TX data size exceeds the memory size and the wrap TX mode is disabled. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
pub type MEM_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR_CH` reader - Represents whether the offset address exceeds memory size (overflows) when writes via APB bus. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
pub type APB_MEM_WR_ERR_CH_R = crate::BitReader;
#[doc = "Field `APB_MEM_RADDR_CH` reader - Represents the memory address offset when reading RAM over APB bus."]
pub type APB_MEM_RADDR_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Represents the memory address offset when transmitter of channel %s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch(&self) -> MEM_RADDR_EX_CH_R {
        MEM_RADDR_EX_CH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - Represents the FSM status of channel %s."]
    #[inline(always)]
    pub fn state_ch(&self) -> STATE_CH_R {
        STATE_CH_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:20 - Represents the memory address offset when writes RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_waddr_ch(&self) -> APB_MEM_WADDR_CH_R {
        APB_MEM_WADDR_CH_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - Represents whether the offset address exceeds memory size when reading via APB bus. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch(&self) -> APB_MEM_RD_ERR_CH_R {
        APB_MEM_RD_ERR_CH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents whether the TX data size exceeds the memory size and the wrap TX mode is disabled. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
    #[inline(always)]
    pub fn mem_empty_ch(&self) -> MEM_EMPTY_CH_R {
        MEM_EMPTY_CH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents whether the offset address exceeds memory size (overflows) when writes via APB bus. \\\\ 0: Not exceed\\\\ 1: Exceed\\\\"]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch(&self) -> APB_MEM_WR_ERR_CH_R {
        APB_MEM_WR_ERR_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Represents the memory address offset when reading RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_raddr_ch(&self) -> APB_MEM_RADDR_CH_R {
        APB_MEM_RADDR_CH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_STATUS")
            .field("mem_raddr_ex_ch", &self.mem_raddr_ex_ch())
            .field("state_ch", &self.state_ch())
            .field("apb_mem_waddr_ch", &self.apb_mem_waddr_ch())
            .field("apb_mem_rd_err_ch", &self.apb_mem_rd_err_ch())
            .field("mem_empty_ch", &self.mem_empty_ch())
            .field("apb_mem_wr_err_ch", &self.apb_mem_wr_err_ch())
            .field("apb_mem_raddr_ch", &self.apb_mem_raddr_ch())
            .finish()
    }
}
#[doc = "Channel %s status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_tx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TX_STATUS_SPEC;
impl crate::RegisterSpec for CH_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_tx_status::R`](R) reader structure"]
impl crate::Readable for CH_TX_STATUS_SPEC {}
#[doc = "`reset()` method sets CH%s_TX_STATUS to value 0"]
impl crate::Resettable for CH_TX_STATUS_SPEC {}
