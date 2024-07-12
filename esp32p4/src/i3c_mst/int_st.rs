#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `TX_DATA_BUF_THLD` reader - This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value."]
pub type TX_DATA_BUF_THLD_R = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD` reader - This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value."]
pub type RX_DATA_BUF_THLD_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD` reader - Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value."]
pub type IBI_STATUS_THLD_R = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD` reader - This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value."]
pub type CMD_BUF_EMPTY_THLD_R = crate::BitReader;
#[doc = "Field `RESP_READY` reader - This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value."]
pub type RESP_READY_R = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR` reader - This interrupt is generated if toc is 0(master will restart next command), but command buf is empty."]
pub type NXT_CMD_REQ_ERR_R = crate::BitReader;
#[doc = "Field `TRANSFER_ERR` reader - This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1."]
pub type TRANSFER_ERR_R = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE` reader - NA"]
pub type TRANSFER_COMPLETE_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE` reader - NA"]
pub type COMMAND_DONE_R = crate::BitReader;
#[doc = "Field `DETECT_START` reader - NA"]
pub type DETECT_START_R = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF` reader - NA"]
pub type RESP_BUF_OVF_R = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF` reader - NA"]
pub type IBI_DATA_BUF_OVF_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF` reader - NA"]
pub type IBI_STATUS_BUF_OVF_R = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE` reader - NA"]
pub type IBI_HANDLE_DONE_R = crate::BitReader;
#[doc = "Field `IBI_DETECT` reader - NA"]
pub type IBI_DETECT_R = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH` reader - NA"]
pub type CMD_CCC_MISMATCH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value."]
    #[inline(always)]
    pub fn tx_data_buf_thld(&self) -> TX_DATA_BUF_THLD_R {
        TX_DATA_BUF_THLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value."]
    #[inline(always)]
    pub fn rx_data_buf_thld(&self) -> RX_DATA_BUF_THLD_R {
        RX_DATA_BUF_THLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value."]
    #[inline(always)]
    pub fn ibi_status_thld(&self) -> IBI_STATUS_THLD_R {
        IBI_STATUS_THLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value."]
    #[inline(always)]
    pub fn cmd_buf_empty_thld(&self) -> CMD_BUF_EMPTY_THLD_R {
        CMD_BUF_EMPTY_THLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value."]
    #[inline(always)]
    pub fn resp_ready(&self) -> RESP_READY_R {
        RESP_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is generated if toc is 0(master will restart next command), but command buf is empty."]
    #[inline(always)]
    pub fn nxt_cmd_req_err(&self) -> NXT_CMD_REQ_ERR_R {
        NXT_CMD_REQ_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1."]
    #[inline(always)]
    pub fn transfer_err(&self) -> TRANSFER_ERR_R {
        TRANSFER_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete(&self) -> TRANSFER_COMPLETE_R {
        TRANSFER_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start(&self) -> DETECT_START_R {
        DETECT_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf(&self) -> RESP_BUF_OVF_R {
        RESP_BUF_OVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf(&self) -> IBI_DATA_BUF_OVF_R {
        IBI_DATA_BUF_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf(&self) -> IBI_STATUS_BUF_OVF_R {
        IBI_STATUS_BUF_OVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done(&self) -> IBI_HANDLE_DONE_R {
        IBI_HANDLE_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect(&self) -> IBI_DETECT_R {
        IBI_DETECT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch(&self) -> CMD_CCC_MISMATCH_R {
        CMD_CCC_MISMATCH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("tx_data_buf_thld", &self.tx_data_buf_thld())
            .field("rx_data_buf_thld", &self.rx_data_buf_thld())
            .field("ibi_status_thld", &self.ibi_status_thld())
            .field("cmd_buf_empty_thld", &self.cmd_buf_empty_thld())
            .field("resp_ready", &self.resp_ready())
            .field("nxt_cmd_req_err", &self.nxt_cmd_req_err())
            .field("transfer_err", &self.transfer_err())
            .field("transfer_complete", &self.transfer_complete())
            .field("command_done", &self.command_done())
            .field("detect_start", &self.detect_start())
            .field("resp_buf_ovf", &self.resp_buf_ovf())
            .field("ibi_data_buf_ovf", &self.ibi_data_buf_ovf())
            .field("ibi_status_buf_ovf", &self.ibi_status_buf_ovf())
            .field("ibi_handle_done", &self.ibi_handle_done())
            .field("ibi_detect", &self.ibi_detect())
            .field("cmd_ccc_mismatch", &self.cmd_ccc_mismatch())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
