#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FIFO_CONF_SPEC>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FIFO_CONF_SPEC>;
#[doc = "Field `RXFIFO_FULL_THRHD` reader - "]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - "]
pub type RXFIFO_FULL_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - Config txfifo empty threhd value when using apb fifo access"]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - Config txfifo empty threhd value when using apb fifo access"]
pub type TXFIFO_EMPTY_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NONFIFO_EN` reader - Set this bit to enble apb nonfifo access."]
pub type NONFIFO_EN_R = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - Set this bit to enble apb nonfifo access."]
pub type NONFIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
pub type FIFO_ADDR_CFG_EN_R = crate::BitReader;
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
pub type FIFO_ADDR_CFG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` reader - Set this bit to reset rx fifo when using apb fifo access."]
pub type RX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Set this bit to reset rx fifo when using apb fifo access."]
pub type RX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RST` reader - Set this bit to reset tx fifo when using apb fifo access."]
pub type TX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Set this bit to reset tx fifo when using apb fifo access."]
pub type TX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONFIFO_RX_THRES` reader - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
pub type NONFIFO_RX_THRES_R = crate::FieldReader;
#[doc = "Field `NONFIFO_RX_THRES` writer - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
pub type NONFIFO_RX_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NONFIFO_TX_THRES` reader - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
pub type NONFIFO_TX_THRES_R = crate::FieldReader;
#[doc = "Field `NONFIFO_TX_THRES` writer - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
pub type NONFIFO_TX_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
            .field("rxfifo_full_thrhd", &self.rxfifo_full_thrhd())
            .field("txfifo_empty_thrhd", &self.txfifo_empty_thrhd())
            .field("nonfifo_en", &self.nonfifo_en())
            .field("fifo_addr_cfg_en", &self.fifo_addr_cfg_en())
            .field("rx_fifo_rst", &self.rx_fifo_rst())
            .field("tx_fifo_rst", &self.tx_fifo_rst())
            .field("nonfifo_rx_thres", &self.nonfifo_rx_thres())
            .field("nonfifo_tx_thres", &self.nonfifo_tx_thres())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<FIFO_CONF_SPEC> {
        RXFIFO_FULL_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Config txfifo empty threhd value when using apb fifo access"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<FIFO_CONF_SPEC> {
        TXFIFO_EMPTY_THRHD_W::new(self, 5)
    }
    #[doc = "Bit 10 - Set this bit to enble apb nonfifo access."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<FIFO_CONF_SPEC> {
        NONFIFO_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - When this bit is set to 1 then the byte after address represent the offset address of I2C Slave's ram."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W<FIFO_CONF_SPEC> {
        FIFO_ADDR_CFG_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to reset rx fifo when using apb fifo access."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<FIFO_CONF_SPEC> {
        RX_FIFO_RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to reset tx fifo when using apb fifo access."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<FIFO_CONF_SPEC> {
        TX_FIFO_RST_W::new(self, 13)
    }
    #[doc = "Bits 14:19 - when I2C receives more than nonfifo_rx_thres data it will produce rx_send_full_int_raw interrupt and update the current offset address of the receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_rx_thres(&mut self) -> NONFIFO_RX_THRES_W<FIFO_CONF_SPEC> {
        NONFIFO_RX_THRES_W::new(self, 14)
    }
    #[doc = "Bits 20:25 - when I2C sends more than nonfifo_tx_thres data it will produce tx_send_empty_int_raw interrupt and update the current offset address of the sending data."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_tx_thres(&mut self) -> NONFIFO_TX_THRES_W<FIFO_CONF_SPEC> {
        NONFIFO_TX_THRES_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_conf::R`](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x0155_408b"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0155_408b;
}
