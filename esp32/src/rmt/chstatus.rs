#[doc = "Register `CH%sSTATUS` reader"]
pub type R = crate::R<CHSTATUS_SPEC>;
#[doc = "Field `STATUS` reader - The status for channel0"]
pub type STATUS_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_WADDR_EX` reader - The current memory read address of channel0."]
pub type MEM_WADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_RADDR_EX` reader - The current memory write address of channel0."]
pub type MEM_RADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `STATE` reader - The channel0 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR` reader - When channel0 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
pub type MEM_OWNER_ERR_R = crate::BitReader;
#[doc = "Field `MEM_FULL` reader - The memory full status bit for channel0 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
pub type MEM_FULL_R = crate::BitReader;
#[doc = "Field `MEM_EMPTY` reader - The memory empty status bit for channel0. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
pub type MEM_EMPTY_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR` reader - The apb write memory status bit for channel0 turns to high level when the apb write address exceeds the configuration range."]
pub type APB_MEM_WR_ERR_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR` reader - The apb read memory status bit for channel0 turns to high level when the apb read address exceeds the configuration range."]
pub type APB_MEM_RD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:31 - The status for channel0"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
    #[doc = "Bits 0:9 - The current memory read address of channel0."]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - The current memory write address of channel0."]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - The channel0 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - When channel0 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The memory full status bit for channel0 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The memory empty status bit for channel0. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel0 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel0 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSTATUS")
            .field("status", &self.status())
            .field("mem_waddr_ex", &self.mem_waddr_ex())
            .field("mem_raddr_ex", &self.mem_raddr_ex())
            .field("state", &self.state())
            .field("mem_owner_err", &self.mem_owner_err())
            .field("mem_full", &self.mem_full())
            .field("mem_empty", &self.mem_empty())
            .field("apb_mem_wr_err", &self.apb_mem_wr_err())
            .field("apb_mem_rd_err", &self.apb_mem_rd_err())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {}
#[doc = "`reset()` method sets CH%sSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
