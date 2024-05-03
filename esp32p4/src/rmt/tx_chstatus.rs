#[doc = "Register `TX_CH%sSTATUS` reader"]
pub type R = crate::R<TX_CHSTATUS_SPEC>;
#[doc = "Field `MEM_RADDR_EX_CH0` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
pub type MEM_RADDR_EX_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_WADDR_CH0` reader - This register records the memory address offset when writes RAM over APB bus."]
pub type APB_MEM_WADDR_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH0` reader - This register records the FSM status of CHANNEL%s."]
pub type STATE_CH0_R = crate::FieldReader;
#[doc = "Field `MEM_EMPTY_CH0` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
pub type MEM_EMPTY_CH0_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR_CH0` reader - This status bit will be set if the offset address out of memory size when writes via APB bus."]
pub type APB_MEM_WR_ERR_CH0_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> MEM_RADDR_EX_CH0_R {
        MEM_RADDR_EX_CH0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - This register records the memory address offset when writes RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> APB_MEM_WADDR_CH0_R {
        APB_MEM_WADDR_CH0_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:24 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state_ch0(&self) -> STATE_CH0_R {
        STATE_CH0_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> MEM_EMPTY_CH0_R {
        MEM_EMPTY_CH0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This status bit will be set if the offset address out of memory size when writes via APB bus."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> APB_MEM_WR_ERR_CH0_R {
        APB_MEM_WR_ERR_CH0_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CHSTATUS")
            .field("mem_raddr_ex_ch0", &self.mem_raddr_ex_ch0().bits())
            .field("apb_mem_waddr_ch0", &self.apb_mem_waddr_ch0().bits())
            .field("state_ch0", &self.state_ch0().bits())
            .field("mem_empty_ch0", &self.mem_empty_ch0().bit())
            .field("apb_mem_wr_err_ch0", &self.apb_mem_wr_err_ch0().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CHSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Channel %s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_chstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CHSTATUS_SPEC;
impl crate::RegisterSpec for TX_CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_chstatus::R`](R) reader structure"]
impl crate::Readable for TX_CHSTATUS_SPEC {}
#[doc = "`reset()` method sets TX_CH%sSTATUS to value 0"]
impl crate::Resettable for TX_CHSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
