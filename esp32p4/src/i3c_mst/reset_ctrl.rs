#[doc = "Register `RESET_CTRL` reader"]
pub type R = crate::R<RESET_CTRL_SPEC>;
#[doc = "Register `RESET_CTRL` writer"]
pub type W = crate::W<RESET_CTRL_SPEC>;
#[doc = "Field `REG_CORE_SOFT_RST` writer - NA"]
pub type REG_CORE_SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CMD_BUF_RST` reader - NA"]
pub type REG_CMD_BUF_RST_R = crate::BitReader;
#[doc = "Field `REG_CMD_BUF_RST` writer - NA"]
pub type REG_CMD_BUF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RESP_BUF_RST` reader - NA"]
pub type REG_RESP_BUF_RST_R = crate::BitReader;
#[doc = "Field `REG_RESP_BUF_RST` writer - NA"]
pub type REG_RESP_BUF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TX_DATA_BUF_BUF_RST` reader - NA"]
pub type REG_TX_DATA_BUF_BUF_RST_R = crate::BitReader;
#[doc = "Field `REG_TX_DATA_BUF_BUF_RST` writer - NA"]
pub type REG_TX_DATA_BUF_BUF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RX_DATA_BUF_RST` reader - NA"]
pub type REG_RX_DATA_BUF_RST_R = crate::BitReader;
#[doc = "Field `REG_RX_DATA_BUF_RST` writer - NA"]
pub type REG_RX_DATA_BUF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IBI_DATA_BUF_RST` reader - NA"]
pub type REG_IBI_DATA_BUF_RST_R = crate::BitReader;
#[doc = "Field `REG_IBI_DATA_BUF_RST` writer - NA"]
pub type REG_IBI_DATA_BUF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IBI_STATUS_BUF_RST` reader - NA"]
pub type REG_IBI_STATUS_BUF_RST_R = crate::BitReader;
#[doc = "Field `REG_IBI_STATUS_BUF_RST` writer - NA"]
pub type REG_IBI_STATUS_BUF_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_cmd_buf_rst(&self) -> REG_CMD_BUF_RST_R {
        REG_CMD_BUF_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_resp_buf_rst(&self) -> REG_RESP_BUF_RST_R {
        REG_RESP_BUF_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_tx_data_buf_buf_rst(&self) -> REG_TX_DATA_BUF_BUF_RST_R {
        REG_TX_DATA_BUF_BUF_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_rx_data_buf_rst(&self) -> REG_RX_DATA_BUF_RST_R {
        REG_RX_DATA_BUF_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_ibi_data_buf_rst(&self) -> REG_IBI_DATA_BUF_RST_R {
        REG_IBI_DATA_BUF_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn reg_ibi_status_buf_rst(&self) -> REG_IBI_STATUS_BUF_RST_R {
        REG_IBI_STATUS_BUF_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_CTRL")
            .field(
                "reg_cmd_buf_rst",
                &format_args!("{}", self.reg_cmd_buf_rst().bit()),
            )
            .field(
                "reg_resp_buf_rst",
                &format_args!("{}", self.reg_resp_buf_rst().bit()),
            )
            .field(
                "reg_tx_data_buf_buf_rst",
                &format_args!("{}", self.reg_tx_data_buf_buf_rst().bit()),
            )
            .field(
                "reg_rx_data_buf_rst",
                &format_args!("{}", self.reg_rx_data_buf_rst().bit()),
            )
            .field(
                "reg_ibi_data_buf_rst",
                &format_args!("{}", self.reg_ibi_data_buf_rst().bit()),
            )
            .field(
                "reg_ibi_status_buf_rst",
                &format_args!("{}", self.reg_ibi_status_buf_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_core_soft_rst(&mut self) -> REG_CORE_SOFT_RST_W<RESET_CTRL_SPEC> {
        REG_CORE_SOFT_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cmd_buf_rst(&mut self) -> REG_CMD_BUF_RST_W<RESET_CTRL_SPEC> {
        REG_CMD_BUF_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_resp_buf_rst(&mut self) -> REG_RESP_BUF_RST_W<RESET_CTRL_SPEC> {
        REG_RESP_BUF_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tx_data_buf_buf_rst(&mut self) -> REG_TX_DATA_BUF_BUF_RST_W<RESET_CTRL_SPEC> {
        REG_TX_DATA_BUF_BUF_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_data_buf_rst(&mut self) -> REG_RX_DATA_BUF_RST_W<RESET_CTRL_SPEC> {
        REG_RX_DATA_BUF_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_data_buf_rst(&mut self) -> REG_IBI_DATA_BUF_RST_W<RESET_CTRL_SPEC> {
        REG_IBI_DATA_BUF_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_status_buf_rst(&mut self) -> REG_IBI_STATUS_BUF_RST_W<RESET_CTRL_SPEC> {
        REG_IBI_STATUS_BUF_RST_W::new(self, 6)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_CTRL_SPEC;
impl crate::RegisterSpec for RESET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_ctrl::R`](R) reader structure"]
impl crate::Readable for RESET_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_ctrl::W`](W) writer structure"]
impl crate::Writable for RESET_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_CTRL to value 0"]
impl crate::Resettable for RESET_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
