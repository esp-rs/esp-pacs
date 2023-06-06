#[doc = "Register `CH5STATUS` reader"]
pub struct R(crate::R<CH5STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - The status for channel5"]
pub type STATUS_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_WADDR_EX` reader - The current memory read address of channel5."]
pub type MEM_WADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_RADDR_EX` reader - The current memory write address of channel5."]
pub type MEM_RADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `STATE` reader - The channel5 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR` reader - When channel5 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
pub type MEM_OWNER_ERR_R = crate::BitReader;
#[doc = "Field `MEM_FULL` reader - The memory full status bit for channel5 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
pub type MEM_FULL_R = crate::BitReader;
#[doc = "Field `MEM_EMPTY` reader - The memory empty status bit for channel5. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
pub type MEM_EMPTY_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR` reader - The apb write memory status bit for channel5 turns to high level when the apb write address exceeds the configuration range."]
pub type APB_MEM_WR_ERR_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR` reader - The apb read memory status bit for channel5 turns to high level when the apb read address exceeds the configuration range."]
pub type APB_MEM_RD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:31 - The status for channel5"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
    #[doc = "Bits 0:9 - The current memory read address of channel5."]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - The current memory write address of channel5."]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - The channel5 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - When channel5 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The memory full status bit for channel5 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The memory empty status bit for channel5. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel5 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel5 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH5STATUS")
            .field("status", &format_args!("{}", self.status().bits()))
            .field(
                "mem_waddr_ex",
                &format_args!("{}", self.mem_waddr_ex().bits()),
            )
            .field(
                "mem_raddr_ex",
                &format_args!("{}", self.mem_raddr_ex().bits()),
            )
            .field("state", &format_args!("{}", self.state().bits()))
            .field(
                "mem_owner_err",
                &format_args!("{}", self.mem_owner_err().bit()),
            )
            .field("mem_full", &format_args!("{}", self.mem_full().bit()))
            .field("mem_empty", &format_args!("{}", self.mem_empty().bit()))
            .field(
                "apb_mem_wr_err",
                &format_args!("{}", self.apb_mem_wr_err().bit()),
            )
            .field(
                "apb_mem_rd_err",
                &format_args!("{}", self.apb_mem_rd_err().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH5STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5status](index.html) module"]
pub struct CH5STATUS_SPEC;
impl crate::RegisterSpec for CH5STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5status::R](R) reader structure"]
impl crate::Readable for CH5STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH5STATUS to value 0"]
impl crate::Resettable for CH5STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
