#[doc = "Register `CH4STATUS` reader"]
pub struct R(crate::R<CH4STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - The status for channel4"]
pub type STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEM_WADDR_EX` reader - The current memory read address of channel4."]
pub type MEM_WADDR_EX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEM_RADDR_EX` reader - The current memory write address of channel4."]
pub type MEM_RADDR_EX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STATE` reader - The channel4 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
pub type STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_OWNER_ERR` reader - When channel4 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
pub type MEM_OWNER_ERR_R = crate::BitReader<bool>;
#[doc = "Field `MEM_FULL` reader - The memory full status bit for channel4 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
pub type MEM_FULL_R = crate::BitReader<bool>;
#[doc = "Field `MEM_EMPTY` reader - The memory empty status bit for channel4. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
pub type MEM_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `APB_MEM_WR_ERR` reader - The apb write memory status bit for channel4 turns to high level when the apb write address exceeds the configuration range."]
pub type APB_MEM_WR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `APB_MEM_RD_ERR` reader - The apb read memory status bit for channel4 turns to high level when the apb read address exceeds the configuration range."]
pub type APB_MEM_RD_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:31 - The status for channel4"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
    #[doc = "Bits 0:9 - The current memory read address of channel4."]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - The current memory write address of channel4."]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - The channel4 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - When channel4 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The memory full status bit for channel4 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The memory empty status bit for channel4. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel4 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel4 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4status](index.html) module"]
pub struct CH4STATUS_SPEC;
impl crate::RegisterSpec for CH4STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4status::R](R) reader structure"]
impl crate::Readable for CH4STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH4STATUS to value 0"]
impl crate::Resettable for CH4STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
