#[doc = "Register `TX_CH%sSTATUS` reader"]
pub struct R(crate::R<TX_CHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_RADDR_EX_CH0` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
pub type MEM_RADDR_EX_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH0` reader - This register records the FSM status of CHANNEL%s."]
pub type STATE_CH0_R = crate::FieldReader;
#[doc = "Field `APB_MEM_WADDR_CH0` reader - This register records the memory address offset when writes RAM over APB bus."]
pub type APB_MEM_WADDR_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_RD_ERR_CH0` reader - This status bit will be set if the offset address out of memory size when reading via APB bus."]
pub type APB_MEM_RD_ERR_CH0_R = crate::BitReader;
#[doc = "Field `MEM_EMPTY_CH0` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
pub type MEM_EMPTY_CH0_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR_CH0` reader - This status bit will be set if the offset address out of memory size when writes via APB bus."]
pub type APB_MEM_WR_ERR_CH0_R = crate::BitReader;
#[doc = "Field `APB_MEM_RADDR_CH0` reader - This register records the memory address offset when reading RAM over APB bus."]
pub type APB_MEM_RADDR_CH0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> MEM_RADDR_EX_CH0_R {
        MEM_RADDR_EX_CH0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - This register records the FSM status of CHANNEL%s."]
    #[inline(always)]
    pub fn state_ch0(&self) -> STATE_CH0_R {
        STATE_CH0_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:20 - This register records the memory address offset when writes RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> APB_MEM_WADDR_CH0_R {
        APB_MEM_WADDR_CH0_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - This status bit will be set if the offset address out of memory size when reading via APB bus."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch0(&self) -> APB_MEM_RD_ERR_CH0_R {
        APB_MEM_RD_ERR_CH0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled."]
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> MEM_EMPTY_CH0_R {
        MEM_EMPTY_CH0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This status bit will be set if the offset address out of memory size when writes via APB bus."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> APB_MEM_WR_ERR_CH0_R {
        APB_MEM_WR_ERR_CH0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - This register records the memory address offset when reading RAM over APB bus."]
    #[inline(always)]
    pub fn apb_mem_raddr_ch0(&self) -> APB_MEM_RADDR_CH0_R {
        APB_MEM_RADDR_CH0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CHSTATUS")
            .field(
                "mem_raddr_ex_ch0",
                &format_args!("{}", self.mem_raddr_ex_ch0().bits()),
            )
            .field("state_ch0", &format_args!("{}", self.state_ch0().bits()))
            .field(
                "apb_mem_waddr_ch0",
                &format_args!("{}", self.apb_mem_waddr_ch0().bits()),
            )
            .field(
                "apb_mem_rd_err_ch0",
                &format_args!("{}", self.apb_mem_rd_err_ch0().bit()),
            )
            .field(
                "mem_empty_ch0",
                &format_args!("{}", self.mem_empty_ch0().bit()),
            )
            .field(
                "apb_mem_wr_err_ch0",
                &format_args!("{}", self.apb_mem_wr_err_ch0().bit()),
            )
            .field(
                "apb_mem_raddr_ch0",
                &format_args!("{}", self.apb_mem_raddr_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CHSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Channel %s status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_chstatus](index.html) module"]
pub struct TX_CHSTATUS_SPEC;
impl crate::RegisterSpec for TX_CHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_chstatus::R](R) reader structure"]
impl crate::Readable for TX_CHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_CH%sSTATUS to value 0"]
impl crate::Resettable for TX_CHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
