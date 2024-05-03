#[doc = "Register `RX_CH%sSTATUS` reader"]
pub type R = crate::R<RX_CHSTATUS_SPEC>;
#[doc = "Field `MEM_WADDR_EX_CH4` reader - This register records the memory address offset when receiver of CHANNEL%s is using the RAM."]
pub type MEM_WADDR_EX_CH4_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_RADDR_CH4` reader - This register records the memory address offset when reads RAM over APB bus."]
pub type APB_MEM_RADDR_CH4_R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH4` reader - This register records the FSM status of CHANNEL%s."]
pub type STATE_CH4_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR_CH4` reader - This status bit will be set when the ownership of memory block is wrong."]
pub type MEM_OWNER_ERR_CH4_R = crate::BitReader;
#[doc = "Field `MEM_FULL_CH4` reader - This status bit will be set if the receiver receives more data than the memory size."]
pub type MEM_FULL_CH4_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR_CH4` reader - This status bit will be set if the offset address out of memory size when reads via APB bus."]
pub type APB_MEM_RD_ERR_CH4_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - This register records the memory address offset when receiver of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&self) -> MEM_WADDR_EX_CH4_R {
        MEM_WADDR_EX_CH4_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - This register records the memory address offset when reads RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_raddr_ch4(&self) -> APB_MEM_RADDR_CH4_R {
        APB_MEM_RADDR_CH4_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:24 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state_ch4(&self) -> STATE_CH4_R {
        STATE_CH4_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - This status bit will be set when the ownership of memory block is wrong."]
    #[inline(always)]
    pub fn mem_owner_err_ch4(&self) -> MEM_OWNER_ERR_CH4_R {
        MEM_OWNER_ERR_CH4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This status bit will be set if the receiver receives more data than the memory size."]
    #[inline(always)]
    pub fn mem_full_ch4(&self) -> MEM_FULL_CH4_R {
        MEM_FULL_CH4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This status bit will be set if the offset address out of memory size when reads via APB bus."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&self) -> APB_MEM_RD_ERR_CH4_R {
        APB_MEM_RD_ERR_CH4_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHSTATUS")
            .field("mem_waddr_ex_ch4", &self.mem_waddr_ex_ch4().bits())
            .field("apb_mem_raddr_ch4", &self.apb_mem_raddr_ch4().bits())
            .field("state_ch4", &self.state_ch4().bits())
            .field("mem_owner_err_ch4", &self.mem_owner_err_ch4().bit())
            .field("mem_full_ch4", &self.mem_full_ch4().bit())
            .field("apb_mem_rd_err_ch4", &self.apb_mem_rd_err_ch4().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CHSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Channel %s status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_chstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CHSTATUS_SPEC;
impl crate::RegisterSpec for RX_CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_chstatus::R`](R) reader structure"]
impl crate::Readable for RX_CHSTATUS_SPEC {}
#[doc = "`reset()` method sets RX_CH%sSTATUS to value 0x0006_00c0"]
impl crate::Resettable for RX_CHSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x0006_00c0;
}
