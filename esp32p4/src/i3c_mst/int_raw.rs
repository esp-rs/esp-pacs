#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `TX_DATA_BUF_THLD_INT_RAW` reader - NA"]
pub type TX_DATA_BUF_THLD_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_DATA_BUF_THLD_INT_RAW` writer - NA"]
pub type TX_DATA_BUF_THLD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_BUF_THLD_INT_RAW` reader - NA"]
pub type RX_DATA_BUF_THLD_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD_INT_RAW` writer - NA"]
pub type RX_DATA_BUF_THLD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_THLD_INT_RAW` reader - NA"]
pub type IBI_STATUS_THLD_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD_INT_RAW` writer - NA"]
pub type IBI_STATUS_THLD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_BUF_EMPTY_THLD_INT_RAW` reader - NA"]
pub type CMD_BUF_EMPTY_THLD_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD_INT_RAW` writer - NA"]
pub type CMD_BUF_EMPTY_THLD_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_READY_INT_RAW` reader - NA"]
pub type RESP_READY_INT_RAW_R = crate::BitReader;
#[doc = "Field `RESP_READY_INT_RAW` writer - NA"]
pub type RESP_READY_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NXT_CMD_REQ_ERR_INT_RAW` reader - NA"]
pub type NXT_CMD_REQ_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR_INT_RAW` writer - NA"]
pub type NXT_CMD_REQ_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_ERR_INT_RAW` reader - NA"]
pub type TRANSFER_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `TRANSFER_ERR_INT_RAW` writer - NA"]
pub type TRANSFER_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE_INT_RAW` reader - NA"]
pub type TRANSFER_COMPLETE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE_INT_RAW` writer - NA"]
pub type TRANSFER_COMPLETE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_DONE_INT_RAW` reader - NA"]
pub type COMMAND_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE_INT_RAW` writer - NA"]
pub type COMMAND_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETECT_START_INT_RAW` reader - NA"]
pub type DETECT_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `DETECT_START_INT_RAW` writer - NA"]
pub type DETECT_START_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_BUF_OVF_INT_RAW` reader - NA"]
pub type RESP_BUF_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF_INT_RAW` writer - NA"]
pub type RESP_BUF_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DATA_BUF_OVF_INT_RAW` reader - NA"]
pub type IBI_DATA_BUF_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF_INT_RAW` writer - NA"]
pub type IBI_DATA_BUF_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_BUF_OVF_INT_RAW` reader - NA"]
pub type IBI_STATUS_BUF_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF_INT_RAW` writer - NA"]
pub type IBI_STATUS_BUF_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_HANDLE_DONE_INT_RAW` reader - NA"]
pub type IBI_HANDLE_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE_INT_RAW` writer - NA"]
pub type IBI_HANDLE_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DETECT_INT_RAW` reader - NA"]
pub type IBI_DETECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `IBI_DETECT_INT_RAW` writer - NA"]
pub type IBI_DETECT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CCC_MISMATCH_INT_RAW` reader - NA"]
pub type CMD_CCC_MISMATCH_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH_INT_RAW` writer - NA"]
pub type CMD_CCC_MISMATCH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tx_data_buf_thld_int_raw(&self) -> TX_DATA_BUF_THLD_INT_RAW_R {
        TX_DATA_BUF_THLD_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn rx_data_buf_thld_int_raw(&self) -> RX_DATA_BUF_THLD_INT_RAW_R {
        RX_DATA_BUF_THLD_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ibi_status_thld_int_raw(&self) -> IBI_STATUS_THLD_INT_RAW_R {
        IBI_STATUS_THLD_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn cmd_buf_empty_thld_int_raw(&self) -> CMD_BUF_EMPTY_THLD_INT_RAW_R {
        CMD_BUF_EMPTY_THLD_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn resp_ready_int_raw(&self) -> RESP_READY_INT_RAW_R {
        RESP_READY_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn nxt_cmd_req_err_int_raw(&self) -> NXT_CMD_REQ_ERR_INT_RAW_R {
        NXT_CMD_REQ_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn transfer_err_int_raw(&self) -> TRANSFER_ERR_INT_RAW_R {
        TRANSFER_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete_int_raw(&self) -> TRANSFER_COMPLETE_INT_RAW_R {
        TRANSFER_COMPLETE_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done_int_raw(&self) -> COMMAND_DONE_INT_RAW_R {
        COMMAND_DONE_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start_int_raw(&self) -> DETECT_START_INT_RAW_R {
        DETECT_START_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf_int_raw(&self) -> RESP_BUF_OVF_INT_RAW_R {
        RESP_BUF_OVF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf_int_raw(&self) -> IBI_DATA_BUF_OVF_INT_RAW_R {
        IBI_DATA_BUF_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf_int_raw(&self) -> IBI_STATUS_BUF_OVF_INT_RAW_R {
        IBI_STATUS_BUF_OVF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done_int_raw(&self) -> IBI_HANDLE_DONE_INT_RAW_R {
        IBI_HANDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect_int_raw(&self) -> IBI_DETECT_INT_RAW_R {
        IBI_DETECT_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch_int_raw(&self) -> CMD_CCC_MISMATCH_INT_RAW_R {
        CMD_CCC_MISMATCH_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "tx_data_buf_thld_int_raw",
                &format_args!("{}", self.tx_data_buf_thld_int_raw().bit()),
            )
            .field(
                "rx_data_buf_thld_int_raw",
                &format_args!("{}", self.rx_data_buf_thld_int_raw().bit()),
            )
            .field(
                "ibi_status_thld_int_raw",
                &format_args!("{}", self.ibi_status_thld_int_raw().bit()),
            )
            .field(
                "cmd_buf_empty_thld_int_raw",
                &format_args!("{}", self.cmd_buf_empty_thld_int_raw().bit()),
            )
            .field(
                "resp_ready_int_raw",
                &format_args!("{}", self.resp_ready_int_raw().bit()),
            )
            .field(
                "nxt_cmd_req_err_int_raw",
                &format_args!("{}", self.nxt_cmd_req_err_int_raw().bit()),
            )
            .field(
                "transfer_err_int_raw",
                &format_args!("{}", self.transfer_err_int_raw().bit()),
            )
            .field(
                "transfer_complete_int_raw",
                &format_args!("{}", self.transfer_complete_int_raw().bit()),
            )
            .field(
                "command_done_int_raw",
                &format_args!("{}", self.command_done_int_raw().bit()),
            )
            .field(
                "detect_start_int_raw",
                &format_args!("{}", self.detect_start_int_raw().bit()),
            )
            .field(
                "resp_buf_ovf_int_raw",
                &format_args!("{}", self.resp_buf_ovf_int_raw().bit()),
            )
            .field(
                "ibi_data_buf_ovf_int_raw",
                &format_args!("{}", self.ibi_data_buf_ovf_int_raw().bit()),
            )
            .field(
                "ibi_status_buf_ovf_int_raw",
                &format_args!("{}", self.ibi_status_buf_ovf_int_raw().bit()),
            )
            .field(
                "ibi_handle_done_int_raw",
                &format_args!("{}", self.ibi_handle_done_int_raw().bit()),
            )
            .field(
                "ibi_detect_int_raw",
                &format_args!("{}", self.ibi_detect_int_raw().bit()),
            )
            .field(
                "cmd_ccc_mismatch_int_raw",
                &format_args!("{}", self.cmd_ccc_mismatch_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_buf_thld_int_raw(&mut self) -> TX_DATA_BUF_THLD_INT_RAW_W<INT_RAW_SPEC> {
        TX_DATA_BUF_THLD_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_buf_thld_int_raw(&mut self) -> RX_DATA_BUF_THLD_INT_RAW_W<INT_RAW_SPEC> {
        RX_DATA_BUF_THLD_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_thld_int_raw(&mut self) -> IBI_STATUS_THLD_INT_RAW_W<INT_RAW_SPEC> {
        IBI_STATUS_THLD_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_buf_empty_thld_int_raw(&mut self) -> CMD_BUF_EMPTY_THLD_INT_RAW_W<INT_RAW_SPEC> {
        CMD_BUF_EMPTY_THLD_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn resp_ready_int_raw(&mut self) -> RESP_READY_INT_RAW_W<INT_RAW_SPEC> {
        RESP_READY_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_cmd_req_err_int_raw(&mut self) -> NXT_CMD_REQ_ERR_INT_RAW_W<INT_RAW_SPEC> {
        NXT_CMD_REQ_ERR_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_err_int_raw(&mut self) -> TRANSFER_ERR_INT_RAW_W<INT_RAW_SPEC> {
        TRANSFER_ERR_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete_int_raw(&mut self) -> TRANSFER_COMPLETE_INT_RAW_W<INT_RAW_SPEC> {
        TRANSFER_COMPLETE_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn command_done_int_raw(&mut self) -> COMMAND_DONE_INT_RAW_W<INT_RAW_SPEC> {
        COMMAND_DONE_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start_int_raw(&mut self) -> DETECT_START_INT_RAW_W<INT_RAW_SPEC> {
        DETECT_START_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn resp_buf_ovf_int_raw(&mut self) -> RESP_BUF_OVF_INT_RAW_W<INT_RAW_SPEC> {
        RESP_BUF_OVF_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_data_buf_ovf_int_raw(&mut self) -> IBI_DATA_BUF_OVF_INT_RAW_W<INT_RAW_SPEC> {
        IBI_DATA_BUF_OVF_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_buf_ovf_int_raw(&mut self) -> IBI_STATUS_BUF_OVF_INT_RAW_W<INT_RAW_SPEC> {
        IBI_STATUS_BUF_OVF_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_handle_done_int_raw(&mut self) -> IBI_HANDLE_DONE_INT_RAW_W<INT_RAW_SPEC> {
        IBI_HANDLE_DONE_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_detect_int_raw(&mut self) -> IBI_DETECT_INT_RAW_W<INT_RAW_SPEC> {
        IBI_DETECT_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ccc_mismatch_int_raw(&mut self) -> CMD_CCC_MISMATCH_INT_RAW_W<INT_RAW_SPEC> {
        CMD_CCC_MISMATCH_INT_RAW_W::new(self, 15)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0x08"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
