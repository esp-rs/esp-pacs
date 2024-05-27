///Register `FIFO_CONF` reader
pub type R = crate::R<FIFO_CONF_SPEC>;
///Register `FIFO_CONF` writer
pub type W = crate::W<FIFO_CONF_SPEC>;
///Field `RXFIFO_WM_THRHD` reader - reg_rxfifo_wm_thrhd
pub type RXFIFO_WM_THRHD_R = crate::FieldReader;
///Field `RXFIFO_WM_THRHD` writer - reg_rxfifo_wm_thrhd
pub type RXFIFO_WM_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TXFIFO_WM_THRHD` reader - reg_txfifo_wm_thrhd
pub type TXFIFO_WM_THRHD_R = crate::FieldReader;
///Field `TXFIFO_WM_THRHD` writer - reg_txfifo_wm_thrhd
pub type TXFIFO_WM_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `NONFIFO_EN` reader - reg_nonfifo_en
pub type NONFIFO_EN_R = crate::BitReader;
///Field `NONFIFO_EN` writer - reg_nonfifo_en
pub type NONFIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_ADDR_CFG_EN` reader - reg_fifo_addr_cfg_en
pub type FIFO_ADDR_CFG_EN_R = crate::BitReader;
///Field `FIFO_ADDR_CFG_EN` writer - reg_fifo_addr_cfg_en
pub type FIFO_ADDR_CFG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_FIFO_RST` reader - reg_rx_fifo_rst
pub type RX_FIFO_RST_R = crate::BitReader;
///Field `RX_FIFO_RST` writer - reg_rx_fifo_rst
pub type RX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_FIFO_RST` reader - reg_tx_fifo_rst
pub type TX_FIFO_RST_R = crate::BitReader;
///Field `TX_FIFO_RST` writer - reg_tx_fifo_rst
pub type TX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFO_PRT_EN` reader - reg_fifo_prt_en
pub type FIFO_PRT_EN_R = crate::BitReader;
///Field `FIFO_PRT_EN` writer - reg_fifo_prt_en
pub type FIFO_PRT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - reg_rxfifo_wm_thrhd
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> RXFIFO_WM_THRHD_R {
        RXFIFO_WM_THRHD_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - reg_txfifo_wm_thrhd
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> TXFIFO_WM_THRHD_R {
        TXFIFO_WM_THRHD_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bit 10 - reg_nonfifo_en
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - reg_fifo_addr_cfg_en
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FIFO_ADDR_CFG_EN_R {
        FIFO_ADDR_CFG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - reg_rx_fifo_rst
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - reg_tx_fifo_rst
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - reg_fifo_prt_en
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> FIFO_PRT_EN_R {
        FIFO_PRT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field("rxfifo_wm_thrhd", &self.rxfifo_wm_thrhd())
            .field("txfifo_wm_thrhd", &self.txfifo_wm_thrhd())
            .field("nonfifo_en", &self.nonfifo_en())
            .field("fifo_addr_cfg_en", &self.fifo_addr_cfg_en())
            .field("rx_fifo_rst", &self.rx_fifo_rst())
            .field("tx_fifo_rst", &self.tx_fifo_rst())
            .field("fifo_prt_en", &self.fifo_prt_en())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - reg_rxfifo_wm_thrhd
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_thrhd(&mut self) -> RXFIFO_WM_THRHD_W<FIFO_CONF_SPEC> {
        RXFIFO_WM_THRHD_W::new(self, 0)
    }
    ///Bits 5:9 - reg_txfifo_wm_thrhd
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_thrhd(&mut self) -> TXFIFO_WM_THRHD_W<FIFO_CONF_SPEC> {
        TXFIFO_WM_THRHD_W::new(self, 5)
    }
    ///Bit 10 - reg_nonfifo_en
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<FIFO_CONF_SPEC> {
        NONFIFO_EN_W::new(self, 10)
    }
    ///Bit 11 - reg_fifo_addr_cfg_en
    #[inline(always)]
    #[must_use]
    pub fn fifo_addr_cfg_en(&mut self) -> FIFO_ADDR_CFG_EN_W<FIFO_CONF_SPEC> {
        FIFO_ADDR_CFG_EN_W::new(self, 11)
    }
    ///Bit 12 - reg_rx_fifo_rst
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<FIFO_CONF_SPEC> {
        RX_FIFO_RST_W::new(self, 12)
    }
    ///Bit 13 - reg_tx_fifo_rst
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<FIFO_CONF_SPEC> {
        TX_FIFO_RST_W::new(self, 13)
    }
    ///Bit 14 - reg_fifo_prt_en
    #[inline(always)]
    #[must_use]
    pub fn fifo_prt_en(&mut self) -> FIFO_PRT_EN_W<FIFO_CONF_SPEC> {
        FIFO_PRT_EN_W::new(self, 14)
    }
}
/**I2C_FIFO_CONF_REG

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo_conf::R`](R) reader structure
impl crate::Readable for FIFO_CONF_SPEC {}
///`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure
impl crate::Writable for FIFO_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_CONF to value 0x408b
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x408b;
}
