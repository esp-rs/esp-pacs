#[doc = "Register `CH%sSTATUS` reader"]
pub type R = crate::R<CHSTATUS_SPEC>;
#[doc = "Field `MEM_RADDR_EX_CH` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
pub type MEM_RADDR_EX_CH_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_WADDR_CH` reader - This register records the memory address offset when writes RAM over APB bus."]
pub type APB_MEM_WADDR_CH_R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH` reader - This register records the FSM status of CHANNEL%s."]
pub type STATE_CH_R = crate::FieldReader;
#[doc = "Field `MEM_EMPTY_CH` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
pub type MEM_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR_CH` reader - This status bit will be set if the offset address out of memory size when writes via APB bus."]
pub type APB_MEM_WR_ERR_CH_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch(&self) -> MEM_RADDR_EX_CH_R {
        MEM_RADDR_EX_CH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - This register records the memory address offset when writes RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_waddr_ch(&self) -> APB_MEM_WADDR_CH_R {
        APB_MEM_WADDR_CH_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:24 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state_ch(&self) -> STATE_CH_R {
        STATE_CH_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
    #[inline(always)]
    pub fn mem_empty_ch(&self) -> MEM_EMPTY_CH_R {
        MEM_EMPTY_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This status bit will be set if the offset address out of memory size when writes via APB bus."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch(&self) -> APB_MEM_WR_ERR_CH_R {
        APB_MEM_WR_ERR_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSTATUS")
            .field("mem_raddr_ex_ch", &self.mem_raddr_ex_ch())
            .field("apb_mem_waddr_ch", &self.apb_mem_waddr_ch())
            .field("state_ch", &self.state_ch())
            .field("mem_empty_ch", &self.mem_empty_ch())
            .field("apb_mem_wr_err_ch", &self.apb_mem_wr_err_ch())
            .finish()
    }
}
#[doc = "Channel %s status register\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {}
#[doc = "`reset()` method sets CH%sSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {}
