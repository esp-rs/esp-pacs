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
#[doc = "Field `RXFIFO_FULL_THRHD` reader - "]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - "]
pub type RXFIFO_FULL_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 5, O>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - Config txfifo empty threhd value when using apb fifo access"]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - Config txfifo empty threhd value when using apb fifo access"]
pub type TXFIFO_EMPTY_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 5, O>;
#[doc = "Field `NONFIFO_EN` reader - Set this bit to enble apb nonfifo access."]
pub type NONFIFO_EN_R = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - Set this bit to enble apb nonfifo access."]
pub type NONFIFO_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
pub type FIFO_ADDR_CFG_EN_R = crate::BitReader;
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
pub type FIFO_ADDR_CFG_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `RX_FIFO_RST` reader - Set this bit to reset rx fifo when using apb fifo access."]
pub type RX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Set this bit to reset rx fifo when using apb fifo access."]
pub type RX_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `TX_FIFO_RST` reader - Set this bit to reset tx fifo when using apb fifo access."]
pub type TX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Set this bit to reset tx fifo when using apb fifo access."]
pub type TX_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `NONFIFO_RX_THRES` reader - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
pub type NONFIFO_RX_THRES_R = crate::FieldReader;
#[doc = "Field `NONFIFO_RX_THRES` writer - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
pub type NONFIFO_RX_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 6, O>;
#[doc = "Field `NONFIFO_TX_THRES` reader - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
pub type NONFIFO_TX_THRES_R = crate::FieldReader;
#[doc = "Field `NONFIFO_TX_THRES` writer - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
pub type NONFIFO_TX_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    pub fn nonfifo_rx_thres(&self) -> NONFIFO_RX_THRES_R {
        NONFIFO_RX_THRES_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    pub fn nonfifo_tx_thres(&self) -> NONFIFO_TX_THRES_R {
        NONFIFO_TX_THRES_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field(
                "rxfifo_full_thrhd",
                &format_args!("{}", self.rxfifo_full_thrhd().bits()),
            )
            .field(
                "txfifo_empty_thrhd",
                &format_args!("{}", self.txfifo_empty_thrhd().bits()),
            )
            .field("nonfifo_en", &format_args!("{}", self.nonfifo_en().bit()))
            .field(
                "fifo_addr_cfg_en",
                &format_args!("{}", self.fifo_addr_cfg_en().bit()),
            )
            .field("rx_fifo_rst", &format_args!("{}", self.rx_fifo_rst().bit()))
            .field("tx_fifo_rst", &format_args!("{}", self.tx_fifo_rst().bit()))
            .field(
                "nonfifo_rx_thres",
                &format_args!("{}", self.nonfifo_rx_thres().bits()),
            )
            .field(
                "nonfifo_tx_thres",
                &format_args!("{}", self.nonfifo_tx_thres().bits()),
            )
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
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<0> {
        RXFIFO_FULL_THRHD_W::new(self)
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<5> {
        TXFIFO_EMPTY_THRHD_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<10> {
        NONFIFO_EN_W::new(self)
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W<11> {
        FIFO_ADDR_CFG_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<12> {
        RX_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<13> {
        TX_FIFO_RST_W::new(self)
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_rx_thres(&mut self) -> NONFIFO_RX_THRES_W<14> {
        NONFIFO_RX_THRES_W::new(self)
    }
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_tx_thres(&mut self) -> NONFIFO_TX_THRES_W<20> {
        NONFIFO_TX_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf](index.html) module"]
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
#[doc = "`reset()` method sets FIFO_CONF to value 0x0155_408b"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0155_408b;
}
