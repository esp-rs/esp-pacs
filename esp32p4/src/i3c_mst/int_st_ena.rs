#[doc = "Register `INT_ST_ENA` reader"]
pub type R = crate::R<INT_ST_ENA_SPEC>;
#[doc = "Register `INT_ST_ENA` writer"]
pub type W = crate::W<INT_ST_ENA_SPEC>;
#[doc = "Field `TX_DATA_BUF_THLD_INT_ENA` reader - Transmit Buffer threshold status enable."]
pub type TX_DATA_BUF_THLD_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_DATA_BUF_THLD_INT_ENA` writer - Transmit Buffer threshold status enable."]
pub type TX_DATA_BUF_THLD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_BUF_THLD_INT_ENA` reader - Receive Buffer threshold status enable."]
pub type RX_DATA_BUF_THLD_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD_INT_ENA` writer - Receive Buffer threshold status enable."]
pub type RX_DATA_BUF_THLD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_THLD_INT_ENA` reader - Only used in master mode. IBI Buffer threshold status enable."]
pub type IBI_STATUS_THLD_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD_INT_ENA` writer - Only used in master mode. IBI Buffer threshold status enable."]
pub type IBI_STATUS_THLD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_BUF_EMPTY_THLD_INT_ENA` reader - Command buffer ready status enable."]
pub type CMD_BUF_EMPTY_THLD_INT_ENA_R = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD_INT_ENA` writer - Command buffer ready status enable."]
pub type CMD_BUF_EMPTY_THLD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_READY_INT_ENA` reader - Response buffer ready status enable."]
pub type RESP_READY_INT_ENA_R = crate::BitReader;
#[doc = "Field `RESP_READY_INT_ENA` writer - Response buffer ready status enable."]
pub type RESP_READY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NXT_CMD_REQ_ERR_INT_ENA` reader - next command request error status enable"]
pub type NXT_CMD_REQ_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR_INT_ENA` writer - next command request error status enable"]
pub type NXT_CMD_REQ_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_ERR_INT_ENA` reader - Transfer error status enable"]
pub type TRANSFER_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANSFER_ERR_INT_ENA` writer - Transfer error status enable"]
pub type TRANSFER_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE_INT_ENA` reader - NA"]
pub type TRANSFER_COMPLETE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE_INT_ENA` writer - NA"]
pub type TRANSFER_COMPLETE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_DONE_INT_ENA` reader - NA"]
pub type COMMAND_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE_INT_ENA` writer - NA"]
pub type COMMAND_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETECT_START_INT_ENA` reader - NA"]
pub type DETECT_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `DETECT_START_INT_ENA` writer - NA"]
pub type DETECT_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_BUF_OVF_INT_ENA` reader - NA"]
pub type RESP_BUF_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF_INT_ENA` writer - NA"]
pub type RESP_BUF_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DATA_BUF_OVF_INT_ENA` reader - NA"]
pub type IBI_DATA_BUF_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF_INT_ENA` writer - NA"]
pub type IBI_DATA_BUF_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_BUF_OVF_INT_ENA` reader - NA"]
pub type IBI_STATUS_BUF_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF_INT_ENA` writer - NA"]
pub type IBI_STATUS_BUF_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_HANDLE_DONE_INT_ENA` reader - NA"]
pub type IBI_HANDLE_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE_INT_ENA` writer - NA"]
pub type IBI_HANDLE_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DETECT_INT_ENA` reader - NA"]
pub type IBI_DETECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `IBI_DETECT_INT_ENA` writer - NA"]
pub type IBI_DETECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CCC_MISMATCH_INT_ENA` reader - NA"]
pub type CMD_CCC_MISMATCH_INT_ENA_R = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH_INT_ENA` writer - NA"]
pub type CMD_CCC_MISMATCH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Buffer threshold status enable."]
    #[inline(always)]
    pub fn tx_data_buf_thld_int_ena(&self) -> TX_DATA_BUF_THLD_INT_ENA_R {
        TX_DATA_BUF_THLD_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Buffer threshold status enable."]
    #[inline(always)]
    pub fn rx_data_buf_thld_int_ena(&self) -> RX_DATA_BUF_THLD_INT_ENA_R {
        RX_DATA_BUF_THLD_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only used in master mode. IBI Buffer threshold status enable."]
    #[inline(always)]
    pub fn ibi_status_thld_int_ena(&self) -> IBI_STATUS_THLD_INT_ENA_R {
        IBI_STATUS_THLD_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command buffer ready status enable."]
    #[inline(always)]
    pub fn cmd_buf_empty_thld_int_ena(&self) -> CMD_BUF_EMPTY_THLD_INT_ENA_R {
        CMD_BUF_EMPTY_THLD_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Response buffer ready status enable."]
    #[inline(always)]
    pub fn resp_ready_int_ena(&self) -> RESP_READY_INT_ENA_R {
        RESP_READY_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - next command request error status enable"]
    #[inline(always)]
    pub fn nxt_cmd_req_err_int_ena(&self) -> NXT_CMD_REQ_ERR_INT_ENA_R {
        NXT_CMD_REQ_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer error status enable"]
    #[inline(always)]
    pub fn transfer_err_int_ena(&self) -> TRANSFER_ERR_INT_ENA_R {
        TRANSFER_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete_int_ena(&self) -> TRANSFER_COMPLETE_INT_ENA_R {
        TRANSFER_COMPLETE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done_int_ena(&self) -> COMMAND_DONE_INT_ENA_R {
        COMMAND_DONE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start_int_ena(&self) -> DETECT_START_INT_ENA_R {
        DETECT_START_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf_int_ena(&self) -> RESP_BUF_OVF_INT_ENA_R {
        RESP_BUF_OVF_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf_int_ena(&self) -> IBI_DATA_BUF_OVF_INT_ENA_R {
        IBI_DATA_BUF_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf_int_ena(&self) -> IBI_STATUS_BUF_OVF_INT_ENA_R {
        IBI_STATUS_BUF_OVF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done_int_ena(&self) -> IBI_HANDLE_DONE_INT_ENA_R {
        IBI_HANDLE_DONE_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect_int_ena(&self) -> IBI_DETECT_INT_ENA_R {
        IBI_DETECT_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch_int_ena(&self) -> CMD_CCC_MISMATCH_INT_ENA_R {
        CMD_CCC_MISMATCH_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_ENA")
            .field(
                "tx_data_buf_thld_int_ena",
                &format_args!("{}", self.tx_data_buf_thld_int_ena().bit()),
            )
            .field(
                "rx_data_buf_thld_int_ena",
                &format_args!("{}", self.rx_data_buf_thld_int_ena().bit()),
            )
            .field(
                "ibi_status_thld_int_ena",
                &format_args!("{}", self.ibi_status_thld_int_ena().bit()),
            )
            .field(
                "cmd_buf_empty_thld_int_ena",
                &format_args!("{}", self.cmd_buf_empty_thld_int_ena().bit()),
            )
            .field(
                "resp_ready_int_ena",
                &format_args!("{}", self.resp_ready_int_ena().bit()),
            )
            .field(
                "nxt_cmd_req_err_int_ena",
                &format_args!("{}", self.nxt_cmd_req_err_int_ena().bit()),
            )
            .field(
                "transfer_err_int_ena",
                &format_args!("{}", self.transfer_err_int_ena().bit()),
            )
            .field(
                "transfer_complete_int_ena",
                &format_args!("{}", self.transfer_complete_int_ena().bit()),
            )
            .field(
                "command_done_int_ena",
                &format_args!("{}", self.command_done_int_ena().bit()),
            )
            .field(
                "detect_start_int_ena",
                &format_args!("{}", self.detect_start_int_ena().bit()),
            )
            .field(
                "resp_buf_ovf_int_ena",
                &format_args!("{}", self.resp_buf_ovf_int_ena().bit()),
            )
            .field(
                "ibi_data_buf_ovf_int_ena",
                &format_args!("{}", self.ibi_data_buf_ovf_int_ena().bit()),
            )
            .field(
                "ibi_status_buf_ovf_int_ena",
                &format_args!("{}", self.ibi_status_buf_ovf_int_ena().bit()),
            )
            .field(
                "ibi_handle_done_int_ena",
                &format_args!("{}", self.ibi_handle_done_int_ena().bit()),
            )
            .field(
                "ibi_detect_int_ena",
                &format_args!("{}", self.ibi_detect_int_ena().bit()),
            )
            .field(
                "cmd_ccc_mismatch_int_ena",
                &format_args!("{}", self.cmd_ccc_mismatch_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Buffer threshold status enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_buf_thld_int_ena(&mut self) -> TX_DATA_BUF_THLD_INT_ENA_W<INT_ST_ENA_SPEC> {
        TX_DATA_BUF_THLD_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Buffer threshold status enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_buf_thld_int_ena(&mut self) -> RX_DATA_BUF_THLD_INT_ENA_W<INT_ST_ENA_SPEC> {
        RX_DATA_BUF_THLD_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Only used in master mode. IBI Buffer threshold status enable."]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_thld_int_ena(&mut self) -> IBI_STATUS_THLD_INT_ENA_W<INT_ST_ENA_SPEC> {
        IBI_STATUS_THLD_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Command buffer ready status enable."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_buf_empty_thld_int_ena(&mut self) -> CMD_BUF_EMPTY_THLD_INT_ENA_W<INT_ST_ENA_SPEC> {
        CMD_BUF_EMPTY_THLD_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Response buffer ready status enable."]
    #[inline(always)]
    #[must_use]
    pub fn resp_ready_int_ena(&mut self) -> RESP_READY_INT_ENA_W<INT_ST_ENA_SPEC> {
        RESP_READY_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - next command request error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_cmd_req_err_int_ena(&mut self) -> NXT_CMD_REQ_ERR_INT_ENA_W<INT_ST_ENA_SPEC> {
        NXT_CMD_REQ_ERR_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_err_int_ena(&mut self) -> TRANSFER_ERR_INT_ENA_W<INT_ST_ENA_SPEC> {
        TRANSFER_ERR_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete_int_ena(&mut self) -> TRANSFER_COMPLETE_INT_ENA_W<INT_ST_ENA_SPEC> {
        TRANSFER_COMPLETE_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn command_done_int_ena(&mut self) -> COMMAND_DONE_INT_ENA_W<INT_ST_ENA_SPEC> {
        COMMAND_DONE_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start_int_ena(&mut self) -> DETECT_START_INT_ENA_W<INT_ST_ENA_SPEC> {
        DETECT_START_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn resp_buf_ovf_int_ena(&mut self) -> RESP_BUF_OVF_INT_ENA_W<INT_ST_ENA_SPEC> {
        RESP_BUF_OVF_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_data_buf_ovf_int_ena(&mut self) -> IBI_DATA_BUF_OVF_INT_ENA_W<INT_ST_ENA_SPEC> {
        IBI_DATA_BUF_OVF_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_buf_ovf_int_ena(&mut self) -> IBI_STATUS_BUF_OVF_INT_ENA_W<INT_ST_ENA_SPEC> {
        IBI_STATUS_BUF_OVF_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_handle_done_int_ena(&mut self) -> IBI_HANDLE_DONE_INT_ENA_W<INT_ST_ENA_SPEC> {
        IBI_HANDLE_DONE_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_detect_int_ena(&mut self) -> IBI_DETECT_INT_ENA_W<INT_ST_ENA_SPEC> {
        IBI_DETECT_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ccc_mismatch_int_ena(&mut self) -> CMD_CCC_MISMATCH_INT_ENA_W<INT_ST_ENA_SPEC> {
        CMD_CCC_MISMATCH_INT_ENA_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_st_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_ENA_SPEC;
impl crate::RegisterSpec for INT_ST_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_ena::R`](R) reader structure"]
impl crate::Readable for INT_ST_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_st_ena::W`](W) writer structure"]
impl crate::Writable for INT_ST_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ST_ENA to value 0"]
impl crate::Resettable for INT_ST_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
