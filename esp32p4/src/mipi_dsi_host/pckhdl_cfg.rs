#[doc = "Register `PCKHDL_CFG` reader"]
pub type R = crate::R<PCKHDL_CFG_SPEC>;
#[doc = "Register `PCKHDL_CFG` writer"]
pub type W = crate::W<PCKHDL_CFG_SPEC>;
#[doc = "Field `EOTP_TX_EN` reader - NA"]
pub type EOTP_TX_EN_R = crate::BitReader;
#[doc = "Field `EOTP_TX_EN` writer - NA"]
pub type EOTP_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTP_RX_EN` reader - NA"]
pub type EOTP_RX_EN_R = crate::BitReader;
#[doc = "Field `EOTP_RX_EN` writer - NA"]
pub type EOTP_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTA_EN` reader - NA"]
pub type BTA_EN_R = crate::BitReader;
#[doc = "Field `BTA_EN` writer - NA"]
pub type BTA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_RX_EN` reader - NA"]
pub type ECC_RX_EN_R = crate::BitReader;
#[doc = "Field `ECC_RX_EN` writer - NA"]
pub type ECC_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_RX_EN` reader - NA"]
pub type CRC_RX_EN_R = crate::BitReader;
#[doc = "Field `CRC_RX_EN` writer - NA"]
pub type CRC_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTP_TX_LP_EN` reader - NA"]
pub type EOTP_TX_LP_EN_R = crate::BitReader;
#[doc = "Field `EOTP_TX_LP_EN` writer - NA"]
pub type EOTP_TX_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn eotp_tx_en(&self) -> EOTP_TX_EN_R {
        EOTP_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn eotp_rx_en(&self) -> EOTP_RX_EN_R {
        EOTP_RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn bta_en(&self) -> BTA_EN_R {
        BTA_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ecc_rx_en(&self) -> ECC_RX_EN_R {
        ECC_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn crc_rx_en(&self) -> CRC_RX_EN_R {
        CRC_RX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn eotp_tx_lp_en(&self) -> EOTP_TX_LP_EN_R {
        EOTP_TX_LP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCKHDL_CFG")
            .field("eotp_tx_en", &self.eotp_tx_en())
            .field("eotp_rx_en", &self.eotp_rx_en())
            .field("bta_en", &self.bta_en())
            .field("ecc_rx_en", &self.ecc_rx_en())
            .field("crc_rx_en", &self.crc_rx_en())
            .field("eotp_tx_lp_en", &self.eotp_tx_lp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn eotp_tx_en(&mut self) -> EOTP_TX_EN_W<'_, PCKHDL_CFG_SPEC> {
        EOTP_TX_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn eotp_rx_en(&mut self) -> EOTP_RX_EN_W<'_, PCKHDL_CFG_SPEC> {
        EOTP_RX_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn bta_en(&mut self) -> BTA_EN_W<'_, PCKHDL_CFG_SPEC> {
        BTA_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ecc_rx_en(&mut self) -> ECC_RX_EN_W<'_, PCKHDL_CFG_SPEC> {
        ECC_RX_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn crc_rx_en(&mut self) -> CRC_RX_EN_W<'_, PCKHDL_CFG_SPEC> {
        CRC_RX_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn eotp_tx_lp_en(&mut self) -> EOTP_TX_LP_EN_W<'_, PCKHDL_CFG_SPEC> {
        EOTP_TX_LP_EN_W::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pckhdl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckhdl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCKHDL_CFG_SPEC;
impl crate::RegisterSpec for PCKHDL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pckhdl_cfg::R`](R) reader structure"]
impl crate::Readable for PCKHDL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pckhdl_cfg::W`](W) writer structure"]
impl crate::Writable for PCKHDL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCKHDL_CFG to value 0"]
impl crate::Resettable for PCKHDL_CFG_SPEC {}
