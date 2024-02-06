#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TX_DATA_BUF_THLD_INT_ST` reader - This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value."]
pub type TX_DATA_BUF_THLD_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD_INT_ST` reader - This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value."]
pub type RX_DATA_BUF_THLD_INT_ST_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD_INT_ST` reader - Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value."]
pub type IBI_STATUS_THLD_INT_ST_R = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD_INT_ST` reader - This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value."]
pub type CMD_BUF_EMPTY_THLD_INT_ST_R = crate::BitReader;
#[doc = "Field `RESP_READY_INT_ST` reader - This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value."]
pub type RESP_READY_INT_ST_R = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR_INT_ST` reader - This interrupt is generated if toc is 0(master will restart next command), but command buf is empty."]
pub type NXT_CMD_REQ_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANSFER_ERR_INT_ST` reader - This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1."]
pub type TRANSFER_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE_INT_ST` reader - NA"]
pub type TRANSFER_COMPLETE_INT_ST_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE_INT_ST` reader - NA"]
pub type COMMAND_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `DETECT_START_INT_ST` reader - NA"]
pub type DETECT_START_INT_ST_R = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF_INT_ST` reader - NA"]
pub type RESP_BUF_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF_INT_ST` reader - NA"]
pub type IBI_DATA_BUF_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF_INT_ST` reader - NA"]
pub type IBI_STATUS_BUF_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE_INT_ST` reader - NA"]
pub type IBI_HANDLE_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `IBI_DETECT_INT_ST` reader - NA"]
pub type IBI_DETECT_INT_ST_R = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH_INT_ST` reader - NA"]
pub type CMD_CCC_MISMATCH_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value."]
    #[inline(always)]
    pub fn tx_data_buf_thld_int_st(&self) -> TX_DATA_BUF_THLD_INT_ST_R {
        TX_DATA_BUF_THLD_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value."]
    #[inline(always)]
    pub fn rx_data_buf_thld_int_st(&self) -> RX_DATA_BUF_THLD_INT_ST_R {
        RX_DATA_BUF_THLD_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value."]
    #[inline(always)]
    pub fn ibi_status_thld_int_st(&self) -> IBI_STATUS_THLD_INT_ST_R {
        IBI_STATUS_THLD_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value."]
    #[inline(always)]
    pub fn cmd_buf_empty_thld_int_st(&self) -> CMD_BUF_EMPTY_THLD_INT_ST_R {
        CMD_BUF_EMPTY_THLD_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value."]
    #[inline(always)]
    pub fn resp_ready_int_st(&self) -> RESP_READY_INT_ST_R {
        RESP_READY_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is generated if toc is 0(master will restart next command), but command buf is empty."]
    #[inline(always)]
    pub fn nxt_cmd_req_err_int_st(&self) -> NXT_CMD_REQ_ERR_INT_ST_R {
        NXT_CMD_REQ_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1."]
    #[inline(always)]
    pub fn transfer_err_int_st(&self) -> TRANSFER_ERR_INT_ST_R {
        TRANSFER_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete_int_st(&self) -> TRANSFER_COMPLETE_INT_ST_R {
        TRANSFER_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done_int_st(&self) -> COMMAND_DONE_INT_ST_R {
        COMMAND_DONE_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start_int_st(&self) -> DETECT_START_INT_ST_R {
        DETECT_START_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf_int_st(&self) -> RESP_BUF_OVF_INT_ST_R {
        RESP_BUF_OVF_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf_int_st(&self) -> IBI_DATA_BUF_OVF_INT_ST_R {
        IBI_DATA_BUF_OVF_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf_int_st(&self) -> IBI_STATUS_BUF_OVF_INT_ST_R {
        IBI_STATUS_BUF_OVF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done_int_st(&self) -> IBI_HANDLE_DONE_INT_ST_R {
        IBI_HANDLE_DONE_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect_int_st(&self) -> IBI_DETECT_INT_ST_R {
        IBI_DETECT_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch_int_st(&self) -> CMD_CCC_MISMATCH_INT_ST_R {
        CMD_CCC_MISMATCH_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "tx_data_buf_thld_int_st",
                &format_args!("{}", self.tx_data_buf_thld_int_st().bit()),
            )
            .field(
                "rx_data_buf_thld_int_st",
                &format_args!("{}", self.rx_data_buf_thld_int_st().bit()),
            )
            .field(
                "ibi_status_thld_int_st",
                &format_args!("{}", self.ibi_status_thld_int_st().bit()),
            )
            .field(
                "cmd_buf_empty_thld_int_st",
                &format_args!("{}", self.cmd_buf_empty_thld_int_st().bit()),
            )
            .field(
                "resp_ready_int_st",
                &format_args!("{}", self.resp_ready_int_st().bit()),
            )
            .field(
                "nxt_cmd_req_err_int_st",
                &format_args!("{}", self.nxt_cmd_req_err_int_st().bit()),
            )
            .field(
                "transfer_err_int_st",
                &format_args!("{}", self.transfer_err_int_st().bit()),
            )
            .field(
                "transfer_complete_int_st",
                &format_args!("{}", self.transfer_complete_int_st().bit()),
            )
            .field(
                "command_done_int_st",
                &format_args!("{}", self.command_done_int_st().bit()),
            )
            .field(
                "detect_start_int_st",
                &format_args!("{}", self.detect_start_int_st().bit()),
            )
            .field(
                "resp_buf_ovf_int_st",
                &format_args!("{}", self.resp_buf_ovf_int_st().bit()),
            )
            .field(
                "ibi_data_buf_ovf_int_st",
                &format_args!("{}", self.ibi_data_buf_ovf_int_st().bit()),
            )
            .field(
                "ibi_status_buf_ovf_int_st",
                &format_args!("{}", self.ibi_status_buf_ovf_int_st().bit()),
            )
            .field(
                "ibi_handle_done_int_st",
                &format_args!("{}", self.ibi_handle_done_int_st().bit()),
            )
            .field(
                "ibi_detect_int_st",
                &format_args!("{}", self.ibi_detect_int_st().bit()),
            )
            .field(
                "cmd_ccc_mismatch_int_st",
                &format_args!("{}", self.cmd_ccc_mismatch_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
