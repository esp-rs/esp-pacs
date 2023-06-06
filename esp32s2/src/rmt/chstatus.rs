#[doc = "Register `CH%sSTATUS` reader"]
pub struct R(crate::R<CHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_WADDR_EX` reader - This register records the memory address offset when receiver of CHANNEL%s is using the RAM."]
pub type MEM_WADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_RADDR_EX` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
pub type MEM_RADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `STATE` reader - This register records the FSM status of CHANNEL%s."]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR` reader - This status bit will be set when the ownership of memory block is wrong."]
pub type MEM_OWNER_ERR_R = crate::BitReader;
#[doc = "Field `MEM_FULL` reader - This status bit will be set if the receiver receives more data than the memory size."]
pub type MEM_FULL_R = crate::BitReader;
#[doc = "Field `MEM_EMPTY` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
pub type MEM_EMPTY_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR` reader - This status bit will be set if the offset address out of memory size when writes via APB bus."]
pub type APB_MEM_WR_ERR_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR` reader - This status bit will be set if the offset address out of memory size when reads via APB bus."]
pub type APB_MEM_RD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - This register records the memory address offset when receiver of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:22 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - This status bit will be set when the ownership of memory block is wrong."]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This status bit will be set if the receiver receives more data than the memory size."]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This status bit will be set if the offset address out of memory size when writes via APB bus."]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This status bit will be set if the offset address out of memory size when reads via APB bus."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSTATUS")
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
impl core::fmt::Debug for crate::generic::Reg<CHSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Channel %s status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](index.html) module"]
pub struct CHSTATUS_SPEC;
impl crate::RegisterSpec for CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chstatus::R](R) reader structure"]
impl crate::Readable for CHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%sSTATUS to value 0"]
impl crate::Resettable for CHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
