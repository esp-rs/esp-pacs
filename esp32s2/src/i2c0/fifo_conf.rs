#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FIFO_CONF_SPEC>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FIFO_CONF_SPEC>;
#[doc = "Field `RXFIFO_WM_THRHD` reader - The water mark threshold of RX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid."]
pub type RXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_WM_THRHD` writer - The water mark threshold of RX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid."]
pub type RXFIFO_WM_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TXFIFO_WM_THRHD` reader - The water mark threshold of TX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and TX FIFO counter is smaller than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
pub type TXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_WM_THRHD` writer - The water mark threshold of TX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and TX FIFO counter is smaller than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
pub type TXFIFO_WM_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NONFIFO_EN` reader - Set this bit to enable APB non-FIFO mode."]
pub type NONFIFO_EN_R = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - Set this bit to enable APB non-FIFO mode."]
pub type NONFIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
pub type FIFO_ADDR_CFG_EN_R = crate::BitReader;
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
pub type FIFO_ADDR_CFG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` reader - Set this bit to reset RX FIFO."]
pub type RX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Set this bit to reset RX FIFO."]
pub type RX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RST` reader - Set this bit to reset TX FIFO."]
pub type TX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Set this bit to reset TX FIFO."]
pub type TX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONFIFO_RX_THRES` reader - When I2C receives more than I2C_NONFIFO_RX_THRES bytes of data, it will generate an I2C_RXFIFO_UDF_INT interrupt and update the current offset address of the received data."]
pub type NONFIFO_RX_THRES_R = crate::FieldReader;
#[doc = "Field `NONFIFO_RX_THRES` writer - When I2C receives more than I2C_NONFIFO_RX_THRES bytes of data, it will generate an I2C_RXFIFO_UDF_INT interrupt and update the current offset address of the received data."]
pub type NONFIFO_RX_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NONFIFO_TX_THRES` reader - When I2C sends more than I2C_NONFIFO_TX_THRES bytes of data, it will generate an I2C_TXFIFO_OVF_INT interrupt and update the current offset address of the sent data."]
pub type NONFIFO_TX_THRES_R = crate::FieldReader;
#[doc = "Field `NONFIFO_TX_THRES` writer - When I2C sends more than I2C_NONFIFO_TX_THRES bytes of data, it will generate an I2C_TXFIFO_OVF_INT interrupt and update the current offset address of the sent data."]
pub type NONFIFO_TX_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FIFO_PRT_EN` reader - The control enable bit of FIFO pointer in non-FIFO mode. This bit controls the valid bits and the interrupts of TX/RX FIFO overflow, underflow, full and empty."]
pub type FIFO_PRT_EN_R = crate::BitReader;
#[doc = "Field `FIFO_PRT_EN` writer - The control enable bit of FIFO pointer in non-FIFO mode. This bit controls the valid bits and the interrupts of TX/RX FIFO overflow, underflow, full and empty."]
pub type FIFO_PRT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - The water mark threshold of RX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid."]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> RXFIFO_WM_THRHD_R {
        RXFIFO_WM_THRHD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The water mark threshold of TX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and TX FIFO counter is smaller than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> TXFIFO_WM_THRHD_R {
        TXFIFO_WM_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Set this bit to enable APB non-FIFO mode."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to reset TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:19 - When I2C receives more than I2C_NONFIFO_RX_THRES bytes of data, it will generate an I2C_RXFIFO_UDF_INT interrupt and update the current offset address of the received data."]
    #[inline(always)]
    pub fn nonfifo_rx_thres(&self) -> NONFIFO_RX_THRES_R {
        NONFIFO_RX_THRES_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - When I2C sends more than I2C_NONFIFO_TX_THRES bytes of data, it will generate an I2C_TXFIFO_OVF_INT interrupt and update the current offset address of the sent data."]
    #[inline(always)]
    pub fn nonfifo_tx_thres(&self) -> NONFIFO_TX_THRES_R {
        NONFIFO_TX_THRES_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - The control enable bit of FIFO pointer in non-FIFO mode. This bit controls the valid bits and the interrupts of TX/RX FIFO overflow, underflow, full and empty."]
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> FIFO_PRT_EN_R {
        FIFO_PRT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field("rxfifo_wm_thrhd", &self.rxfifo_wm_thrhd().bits())
            .field("txfifo_wm_thrhd", &self.txfifo_wm_thrhd().bits())
            .field("nonfifo_en", &self.nonfifo_en().bit())
            .field("fifo_addr_cfg_en", &self.fifo_addr_cfg_en().bit())
            .field("rx_fifo_rst", &self.rx_fifo_rst().bit())
            .field("tx_fifo_rst", &self.tx_fifo_rst().bit())
            .field("nonfifo_rx_thres", &self.nonfifo_rx_thres().bits())
            .field("nonfifo_tx_thres", &self.nonfifo_tx_thres().bits())
            .field("fifo_prt_en", &self.fifo_prt_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - The water mark threshold of RX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_thrhd(&mut self) -> RXFIFO_WM_THRHD_W<FIFO_CONF_SPEC> {
        RXFIFO_WM_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - The water mark threshold of TX FIFO in non-FIFO mode. When I2C_FIFO_PRT_EN is 1 and TX FIFO counter is smaller than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_thrhd(&mut self) -> TXFIFO_WM_THRHD_W<FIFO_CONF_SPEC> {
        TXFIFO_WM_THRHD_W::new(self, 5)
    }
    #[doc = "Bit 10 - Set this bit to enable APB non-FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<FIFO_CONF_SPEC> {
        NONFIFO_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - When this bit is set to 1, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W<FIFO_CONF_SPEC> {
        FIFO_ADDR_CFG_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to reset RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<FIFO_CONF_SPEC> {
        RX_FIFO_RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to reset TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<FIFO_CONF_SPEC> {
        TX_FIFO_RST_W::new(self, 13)
    }
    #[doc = "Bits 14:19 - When I2C receives more than I2C_NONFIFO_RX_THRES bytes of data, it will generate an I2C_RXFIFO_UDF_INT interrupt and update the current offset address of the received data."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_rx_thres(&mut self) -> NONFIFO_RX_THRES_W<FIFO_CONF_SPEC> {
        NONFIFO_RX_THRES_W::new(self, 14)
    }
    #[doc = "Bits 20:25 - When I2C sends more than I2C_NONFIFO_TX_THRES bytes of data, it will generate an I2C_TXFIFO_OVF_INT interrupt and update the current offset address of the sent data."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_tx_thres(&mut self) -> NONFIFO_TX_THRES_W<FIFO_CONF_SPEC> {
        NONFIFO_TX_THRES_W::new(self, 20)
    }
    #[doc = "Bit 26 - The control enable bit of FIFO pointer in non-FIFO mode. This bit controls the valid bits and the interrupts of TX/RX FIFO overflow, underflow, full and empty."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_prt_en(&mut self) -> FIFO_PRT_EN_W<FIFO_CONF_SPEC> {
        FIFO_PRT_EN_W::new(self, 26)
    }
}
#[doc = "FIFO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets FIFO_CONF to value 0x0555_408b"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0555_408b;
}
