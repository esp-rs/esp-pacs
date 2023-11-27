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
            .field("eotp_tx_en", &format_args!("{}", self.eotp_tx_en().bit()))
            .field("eotp_rx_en", &format_args!("{}", self.eotp_rx_en().bit()))
            .field("bta_en", &format_args!("{}", self.bta_en().bit()))
            .field("ecc_rx_en", &format_args!("{}", self.ecc_rx_en().bit()))
            .field("crc_rx_en", &format_args!("{}", self.crc_rx_en().bit()))
            .field(
                "eotp_tx_lp_en",
                &format_args!("{}", self.eotp_tx_lp_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PCKHDL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn eotp_tx_en(&mut self) -> EOTP_TX_EN_W<PCKHDL_CFG_SPEC> {
        EOTP_TX_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn eotp_rx_en(&mut self) -> EOTP_RX_EN_W<PCKHDL_CFG_SPEC> {
        EOTP_RX_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn bta_en(&mut self) -> BTA_EN_W<PCKHDL_CFG_SPEC> {
        BTA_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_rx_en(&mut self) -> ECC_RX_EN_W<PCKHDL_CFG_SPEC> {
        ECC_RX_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn crc_rx_en(&mut self) -> CRC_RX_EN_W<PCKHDL_CFG_SPEC> {
        CRC_RX_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn eotp_tx_lp_en(&mut self) -> EOTP_TX_LP_EN_W<PCKHDL_CFG_SPEC> {
        EOTP_TX_LP_EN_W::new(self, 5)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckhdl_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckhdl_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCKHDL_CFG_SPEC;
impl crate::RegisterSpec for PCKHDL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pckhdl_cfg::R`](R) reader structure"]
impl crate::Readable for PCKHDL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pckhdl_cfg::W`](W) writer structure"]
impl crate::Writable for PCKHDL_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCKHDL_CFG to value 0"]
impl crate::Resettable for PCKHDL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
