#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TX_DATA_BUF_THLD` writer - NA"]
pub type TX_DATA_BUF_THLD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_DATA_BUF_THLD` writer - NA"]
pub type RX_DATA_BUF_THLD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_STATUS_THLD` writer - NA"]
pub type IBI_STATUS_THLD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_BUF_EMPTY_THLD` writer - NA"]
pub type CMD_BUF_EMPTY_THLD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESP_READY` writer - NA"]
pub type RESP_READY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NXT_CMD_REQ_ERR` writer - NA"]
pub type NXT_CMD_REQ_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANSFER_ERR` writer - NA"]
pub type TRANSFER_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE` writer - NA"]
pub type TRANSFER_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMMAND_DONE` writer - NA"]
pub type COMMAND_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DETECT_START` writer - NA"]
pub type DETECT_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESP_BUF_OVF` writer - NA"]
pub type RESP_BUF_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_DATA_BUF_OVF` writer - NA"]
pub type IBI_DATA_BUF_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_STATUS_BUF_OVF` writer - NA"]
pub type IBI_STATUS_BUF_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_HANDLE_DONE` writer - NA"]
pub type IBI_HANDLE_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IBI_DETECT` writer - NA"]
pub type IBI_DETECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_CCC_MISMATCH` writer - NA"]
pub type CMD_CCC_MISMATCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_buf_thld(&mut self) -> TX_DATA_BUF_THLD_W<INT_CLR_SPEC> {
        TX_DATA_BUF_THLD_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_buf_thld(&mut self) -> RX_DATA_BUF_THLD_W<INT_CLR_SPEC> {
        RX_DATA_BUF_THLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_thld(&mut self) -> IBI_STATUS_THLD_W<INT_CLR_SPEC> {
        IBI_STATUS_THLD_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_buf_empty_thld(&mut self) -> CMD_BUF_EMPTY_THLD_W<INT_CLR_SPEC> {
        CMD_BUF_EMPTY_THLD_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn resp_ready(&mut self) -> RESP_READY_W<INT_CLR_SPEC> {
        RESP_READY_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_cmd_req_err(&mut self) -> NXT_CMD_REQ_ERR_W<INT_CLR_SPEC> {
        NXT_CMD_REQ_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_err(&mut self) -> TRANSFER_ERR_W<INT_CLR_SPEC> {
        TRANSFER_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete(&mut self) -> TRANSFER_COMPLETE_W<INT_CLR_SPEC> {
        TRANSFER_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<INT_CLR_SPEC> {
        COMMAND_DONE_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start(&mut self) -> DETECT_START_W<INT_CLR_SPEC> {
        DETECT_START_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn resp_buf_ovf(&mut self) -> RESP_BUF_OVF_W<INT_CLR_SPEC> {
        RESP_BUF_OVF_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_data_buf_ovf(&mut self) -> IBI_DATA_BUF_OVF_W<INT_CLR_SPEC> {
        IBI_DATA_BUF_OVF_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_buf_ovf(&mut self) -> IBI_STATUS_BUF_OVF_W<INT_CLR_SPEC> {
        IBI_STATUS_BUF_OVF_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_handle_done(&mut self) -> IBI_HANDLE_DONE_W<INT_CLR_SPEC> {
        IBI_HANDLE_DONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_detect(&mut self) -> IBI_DETECT_W<INT_CLR_SPEC> {
        IBI_DETECT_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ccc_mismatch(&mut self) -> CMD_CCC_MISMATCH_W<INT_CLR_SPEC> {
        CMD_CCC_MISMATCH_W::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
