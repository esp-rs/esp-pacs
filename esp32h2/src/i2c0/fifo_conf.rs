#[doc = "Register `FIFO_CONF` reader"]
pub struct R(crate::R<FIFO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CONF` writer"]
pub struct W(crate::W<FIFO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FIFO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_WM_THRHD` reader - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[4:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
pub type RXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_WM_THRHD` writer - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[4:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
pub type RXFIFO_WM_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 5, O>;
#[doc = "Field `TXFIFO_WM_THRHD` reader - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[4:0\\], reg_txfifo_wm_int_raw bit will be valid."]
pub type TXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_WM_THRHD` writer - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[4:0\\], reg_txfifo_wm_int_raw bit will be valid."]
pub type TXFIFO_WM_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 5, O>;
#[doc = "Field `NONFIFO_EN` reader - Set this bit to enable APB nonfifo access."]
pub type NONFIFO_EN_R = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - Set this bit to enable APB nonfifo access."]
pub type NONFIFO_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
pub type FIFO_ADDR_CFG_EN_R = crate::BitReader;
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
pub type FIFO_ADDR_CFG_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `RX_FIFO_RST` reader - Set this bit to reset rx-fifo."]
pub type RX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Set this bit to reset rx-fifo."]
pub type RX_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `TX_FIFO_RST` reader - Set this bit to reset tx-fifo."]
pub type TX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Set this bit to reset tx-fifo."]
pub type TX_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `FIFO_PRT_EN` reader - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
pub type FIFO_PRT_EN_R = crate::BitReader;
#[doc = "Field `FIFO_PRT_EN` writer - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
pub type FIFO_PRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[4:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> RXFIFO_WM_THRHD_R {
        RXFIFO_WM_THRHD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[4:0\\], reg_txfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> TXFIFO_WM_THRHD_R {
        TXFIFO_WM_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Set this bit to enable APB nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset rx-fifo."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to reset tx-fifo."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> FIFO_PRT_EN_R {
        FIFO_PRT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field(
                "rxfifo_wm_thrhd",
                &format_args!("{}", self.rxfifo_wm_thrhd().bits()),
            )
            .field(
                "txfifo_wm_thrhd",
                &format_args!("{}", self.txfifo_wm_thrhd().bits()),
            )
            .field("nonfifo_en", &format_args!("{}", self.nonfifo_en().bit()))
            .field(
                "fifo_addr_cfg_en",
                &format_args!("{}", self.fifo_addr_cfg_en().bit()),
            )
            .field("rx_fifo_rst", &format_args!("{}", self.rx_fifo_rst().bit()))
            .field("tx_fifo_rst", &format_args!("{}", self.tx_fifo_rst().bit()))
            .field("fifo_prt_en", &format_args!("{}", self.fifo_prt_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[4:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_thrhd(&mut self) -> RXFIFO_WM_THRHD_W<0> {
        RXFIFO_WM_THRHD_W::new(self)
    }
    #[doc = "Bits 5:9 - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[4:0\\], reg_txfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_thrhd(&mut self) -> TXFIFO_WM_THRHD_W<5> {
        TXFIFO_WM_THRHD_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to enable APB nonfifo access."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<10> {
        NONFIFO_EN_W::new(self)
    }
    #[doc = "Bit 11 - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W<11> {
        FIFO_ADDR_CFG_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to reset rx-fifo."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<12> {
        RX_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to reset tx-fifo."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<13> {
        TX_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 14 - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_prt_en(&mut self) -> FIFO_PRT_EN_W<14> {
        FIFO_PRT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf](index.html) module"]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_conf::R](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x408b"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x408b;
}
