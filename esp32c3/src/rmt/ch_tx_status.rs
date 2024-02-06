#[doc = "Register `CH%s_TX_STATUS` reader"]
pub type R = crate::R<CH_TX_STATUS_SPEC>;
#[doc = "Field `MEM_RADDR_EX` reader - reg_mem_raddr_ex_ch0."]
pub type MEM_RADDR_EX_R = crate::FieldReader<u16>;
#[doc = "Field `STATE` reader - reg_state_ch0."]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `APB_MEM_WADDR` reader - reg_apb_mem_waddr_ch0."]
pub type APB_MEM_WADDR_R = crate::FieldReader<u16>;
#[doc = "Field `APB_MEM_RD_ERR` reader - reg_apb_mem_rd_err_ch0."]
pub type APB_MEM_RD_ERR_R = crate::BitReader;
#[doc = "Field `MEM_EMPTY` reader - reg_mem_empty_ch0."]
pub type MEM_EMPTY_R = crate::BitReader;
#[doc = "Field `APB_MEM_WR_ERR` reader - reg_apb_mem_wr_err_ch0."]
pub type APB_MEM_WR_ERR_R = crate::BitReader;
#[doc = "Field `APB_MEM_RADDR` reader - reg_apb_mem_raddr_ch0."]
pub type APB_MEM_RADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - reg_mem_raddr_ex_ch0."]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - reg_state_ch0."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:20 - reg_apb_mem_waddr_ch0."]
    #[inline(always)]
    pub fn apb_mem_waddr(&self) -> APB_MEM_WADDR_R {
        APB_MEM_WADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - reg_apb_mem_rd_err_ch0."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reg_mem_empty_ch0."]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_apb_mem_wr_err_ch0."]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - reg_apb_mem_raddr_ch0."]
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_STATUS")
            .field(
                "mem_raddr_ex",
                &format_args!("{}", self.mem_raddr_ex().bits()),
            )
            .field("state", &format_args!("{}", self.state().bits()))
            .field(
                "apb_mem_waddr",
                &format_args!("{}", self.apb_mem_waddr().bits()),
            )
            .field(
                "apb_mem_rd_err",
                &format_args!("{}", self.apb_mem_rd_err().bit()),
            )
            .field("mem_empty", &format_args!("{}", self.mem_empty().bit()))
            .field(
                "apb_mem_wr_err",
                &format_args!("{}", self.apb_mem_wr_err().bit()),
            )
            .field(
                "apb_mem_raddr",
                &format_args!("{}", self.apb_mem_raddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_TX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RMT_CH%sSTATUS_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TX_STATUS_SPEC;
impl crate::RegisterSpec for CH_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_tx_status::R`](R) reader structure"]
impl crate::Readable for CH_TX_STATUS_SPEC {}
#[doc = "`reset()` method sets CH%s_TX_STATUS to value 0"]
impl crate::Resettable for CH_TX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
