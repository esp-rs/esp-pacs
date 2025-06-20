#[doc = "Register `INT_STAT` reader"]
pub type R = crate::R<INT_STAT_SPEC>;
#[doc = "Register `INT_STAT` writer"]
pub type W = crate::W<INT_STAT_SPEC>;
#[doc = "Field `RXI_INT_ST` reader - The masked interrupt status of TWAIFD_RXI_INT. Frame received interrupt."]
pub type RXI_INT_ST_R = crate::BitReader;
#[doc = "Field `RXI_INT_ST` writer - The masked interrupt status of TWAIFD_RXI_INT. Frame received interrupt."]
pub type RXI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXI_INT_ST` reader - The masked interrupt status of TWAIFD_TXI_INT. Frame transmitted interrupt."]
pub type TXI_INT_ST_R = crate::BitReader;
#[doc = "Field `TXI_INT_ST` writer - The masked interrupt status of TWAIFD_TXI_INT. Frame transmitted interrupt."]
pub type TXI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWLI_INT_ST` reader - The masked interrupt status of TWAIFD_EWLI_INT. Error warning limit interrupt. When both TEC and REC are lower than EWL and one of the becomes equal to or higher than EWL, or when both TEC and REC become less than EWL, this interrupt is generated. When Interrupt is cleared and REC, or TEC is still equal to or higher than EWL, Interrupt is not generated again."]
pub type EWLI_INT_ST_R = crate::BitReader;
#[doc = "Field `EWLI_INT_ST` writer - The masked interrupt status of TWAIFD_EWLI_INT. Error warning limit interrupt. When both TEC and REC are lower than EWL and one of the becomes equal to or higher than EWL, or when both TEC and REC become less than EWL, this interrupt is generated. When Interrupt is cleared and REC, or TEC is still equal to or higher than EWL, Interrupt is not generated again."]
pub type EWLI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOI_INT_ST` reader - The masked interrupt status of TWAIFD_DOI_INT. Data overrun interrupt. Before this Interrupt is cleared , STATUS\\[DOR\\] must be cleared to avoid setting of this Interrupt again."]
pub type DOI_INT_ST_R = crate::BitReader;
#[doc = "Field `DOI_INT_ST` writer - The masked interrupt status of TWAIFD_DOI_INT. Data overrun interrupt. Before this Interrupt is cleared , STATUS\\[DOR\\] must be cleared to avoid setting of this Interrupt again."]
pub type DOI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSI_INT_ST` reader - The masked interrupt status of TWAIFD_FCSI_INT. Fault confinement state changed interrupt. Interrupt is set when node turns error-passive (from error-active), bus-off (from error-passive) or error-active (from bus-off after reintegration or from error-passive)."]
pub type FCSI_INT_ST_R = crate::BitReader;
#[doc = "Field `FCSI_INT_ST` writer - The masked interrupt status of TWAIFD_FCSI_INT. Fault confinement state changed interrupt. Interrupt is set when node turns error-passive (from error-active), bus-off (from error-passive) or error-active (from bus-off after reintegration or from error-passive)."]
pub type FCSI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALI_INT_ST` reader - The masked interrupt status of TWAIFD_ALI_INT. Arbitration lost interrupt."]
pub type ALI_INT_ST_R = crate::BitReader;
#[doc = "Field `ALI_INT_ST` writer - The masked interrupt status of TWAIFD_ALI_INT. Arbitration lost interrupt."]
pub type ALI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEI_INT_ST` reader - The masked interrupt status of TWAIFD_BEI_INT. Bus error interrupt."]
pub type BEI_INT_ST_R = crate::BitReader;
#[doc = "Field `BEI_INT_ST` writer - The masked interrupt status of TWAIFD_BEI_INT. Bus error interrupt."]
pub type BEI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFI_INT_ST` reader - The masked interrupt status of TWAIFD_OFI_INT. Overload frame interrupt."]
pub type OFI_INT_ST_R = crate::BitReader;
#[doc = "Field `OFI_INT_ST` writer - The masked interrupt status of TWAIFD_OFI_INT. Overload frame interrupt."]
pub type OFI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFI_INT_ST` reader - The masked interrupt status of TWAIFD_RXFI_INT. RX buffer full interrupt."]
pub type RXFI_INT_ST_R = crate::BitReader;
#[doc = "Field `RXFI_INT_ST` writer - The masked interrupt status of TWAIFD_RXFI_INT. RX buffer full interrupt."]
pub type RXFI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSI_INT_ST` reader - The masked interrupt status of TWAIFD_BSI_INT. Bit rate shifted interrupt."]
pub type BSI_INT_ST_R = crate::BitReader;
#[doc = "Field `BSI_INT_ST` writer - The masked interrupt status of TWAIFD_BSI_INT. Bit rate shifted interrupt."]
pub type BSI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEI_INT_ST` reader - The masked interrupt status of TWAIFD_RBNEI_INT. RX buffer not empty interrupt. Clearing this interrupt and not reading out content of RX Buffer via RX_DATA will re-activate the interrupt."]
pub type RBNEI_INT_ST_R = crate::BitReader;
#[doc = "Field `RBNEI_INT_ST` writer - The masked interrupt status of TWAIFD_RBNEI_INT. RX buffer not empty interrupt. Clearing this interrupt and not reading out content of RX Buffer via RX_DATA will re-activate the interrupt."]
pub type RBNEI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBHCI_INT_ST` reader - The masked interrupt status of TWAIFD_TXBHCI_INT. TXT buffer HW command interrupt. Anytime TXT buffer receives HW command from CAN Core which changes TXT buffer state to \"TX OK\", \"Error\" or \"Aborted\", this interrupt will be generated."]
pub type TXBHCI_INT_ST_R = crate::BitReader;
#[doc = "Field `TXBHCI_INT_ST` writer - The masked interrupt status of TWAIFD_TXBHCI_INT. TXT buffer HW command interrupt. Anytime TXT buffer receives HW command from CAN Core which changes TXT buffer state to \"TX OK\", \"Error\" or \"Aborted\", this interrupt will be generated."]
pub type TXBHCI_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of TWAIFD_RXI_INT. Frame received interrupt."]
    #[inline(always)]
    pub fn rxi_int_st(&self) -> RXI_INT_ST_R {
        RXI_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of TWAIFD_TXI_INT. Frame transmitted interrupt."]
    #[inline(always)]
    pub fn txi_int_st(&self) -> TXI_INT_ST_R {
        TXI_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of TWAIFD_EWLI_INT. Error warning limit interrupt. When both TEC and REC are lower than EWL and one of the becomes equal to or higher than EWL, or when both TEC and REC become less than EWL, this interrupt is generated. When Interrupt is cleared and REC, or TEC is still equal to or higher than EWL, Interrupt is not generated again."]
    #[inline(always)]
    pub fn ewli_int_st(&self) -> EWLI_INT_ST_R {
        EWLI_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of TWAIFD_DOI_INT. Data overrun interrupt. Before this Interrupt is cleared , STATUS\\[DOR\\] must be cleared to avoid setting of this Interrupt again."]
    #[inline(always)]
    pub fn doi_int_st(&self) -> DOI_INT_ST_R {
        DOI_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of TWAIFD_FCSI_INT. Fault confinement state changed interrupt. Interrupt is set when node turns error-passive (from error-active), bus-off (from error-passive) or error-active (from bus-off after reintegration or from error-passive)."]
    #[inline(always)]
    pub fn fcsi_int_st(&self) -> FCSI_INT_ST_R {
        FCSI_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of TWAIFD_ALI_INT. Arbitration lost interrupt."]
    #[inline(always)]
    pub fn ali_int_st(&self) -> ALI_INT_ST_R {
        ALI_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of TWAIFD_BEI_INT. Bus error interrupt."]
    #[inline(always)]
    pub fn bei_int_st(&self) -> BEI_INT_ST_R {
        BEI_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status of TWAIFD_OFI_INT. Overload frame interrupt."]
    #[inline(always)]
    pub fn ofi_int_st(&self) -> OFI_INT_ST_R {
        OFI_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status of TWAIFD_RXFI_INT. RX buffer full interrupt."]
    #[inline(always)]
    pub fn rxfi_int_st(&self) -> RXFI_INT_ST_R {
        RXFI_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status of TWAIFD_BSI_INT. Bit rate shifted interrupt."]
    #[inline(always)]
    pub fn bsi_int_st(&self) -> BSI_INT_ST_R {
        BSI_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status of TWAIFD_RBNEI_INT. RX buffer not empty interrupt. Clearing this interrupt and not reading out content of RX Buffer via RX_DATA will re-activate the interrupt."]
    #[inline(always)]
    pub fn rbnei_int_st(&self) -> RBNEI_INT_ST_R {
        RBNEI_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status of TWAIFD_TXBHCI_INT. TXT buffer HW command interrupt. Anytime TXT buffer receives HW command from CAN Core which changes TXT buffer state to \"TX OK\", \"Error\" or \"Aborted\", this interrupt will be generated."]
    #[inline(always)]
    pub fn txbhci_int_st(&self) -> TXBHCI_INT_ST_R {
        TXBHCI_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STAT")
            .field("rxi_int_st", &self.rxi_int_st())
            .field("txi_int_st", &self.txi_int_st())
            .field("ewli_int_st", &self.ewli_int_st())
            .field("doi_int_st", &self.doi_int_st())
            .field("fcsi_int_st", &self.fcsi_int_st())
            .field("ali_int_st", &self.ali_int_st())
            .field("bei_int_st", &self.bei_int_st())
            .field("ofi_int_st", &self.ofi_int_st())
            .field("rxfi_int_st", &self.rxfi_int_st())
            .field("bsi_int_st", &self.bsi_int_st())
            .field("rbnei_int_st", &self.rbnei_int_st())
            .field("txbhci_int_st", &self.txbhci_int_st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The masked interrupt status of TWAIFD_RXI_INT. Frame received interrupt."]
    #[inline(always)]
    pub fn rxi_int_st(&mut self) -> RXI_INT_ST_W<INT_STAT_SPEC> {
        RXI_INT_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of TWAIFD_TXI_INT. Frame transmitted interrupt."]
    #[inline(always)]
    pub fn txi_int_st(&mut self) -> TXI_INT_ST_W<INT_STAT_SPEC> {
        TXI_INT_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - The masked interrupt status of TWAIFD_EWLI_INT. Error warning limit interrupt. When both TEC and REC are lower than EWL and one of the becomes equal to or higher than EWL, or when both TEC and REC become less than EWL, this interrupt is generated. When Interrupt is cleared and REC, or TEC is still equal to or higher than EWL, Interrupt is not generated again."]
    #[inline(always)]
    pub fn ewli_int_st(&mut self) -> EWLI_INT_ST_W<INT_STAT_SPEC> {
        EWLI_INT_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - The masked interrupt status of TWAIFD_DOI_INT. Data overrun interrupt. Before this Interrupt is cleared , STATUS\\[DOR\\] must be cleared to avoid setting of this Interrupt again."]
    #[inline(always)]
    pub fn doi_int_st(&mut self) -> DOI_INT_ST_W<INT_STAT_SPEC> {
        DOI_INT_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - The masked interrupt status of TWAIFD_FCSI_INT. Fault confinement state changed interrupt. Interrupt is set when node turns error-passive (from error-active), bus-off (from error-passive) or error-active (from bus-off after reintegration or from error-passive)."]
    #[inline(always)]
    pub fn fcsi_int_st(&mut self) -> FCSI_INT_ST_W<INT_STAT_SPEC> {
        FCSI_INT_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - The masked interrupt status of TWAIFD_ALI_INT. Arbitration lost interrupt."]
    #[inline(always)]
    pub fn ali_int_st(&mut self) -> ALI_INT_ST_W<INT_STAT_SPEC> {
        ALI_INT_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - The masked interrupt status of TWAIFD_BEI_INT. Bus error interrupt."]
    #[inline(always)]
    pub fn bei_int_st(&mut self) -> BEI_INT_ST_W<INT_STAT_SPEC> {
        BEI_INT_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - The masked interrupt status of TWAIFD_OFI_INT. Overload frame interrupt."]
    #[inline(always)]
    pub fn ofi_int_st(&mut self) -> OFI_INT_ST_W<INT_STAT_SPEC> {
        OFI_INT_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - The masked interrupt status of TWAIFD_RXFI_INT. RX buffer full interrupt."]
    #[inline(always)]
    pub fn rxfi_int_st(&mut self) -> RXFI_INT_ST_W<INT_STAT_SPEC> {
        RXFI_INT_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - The masked interrupt status of TWAIFD_BSI_INT. Bit rate shifted interrupt."]
    #[inline(always)]
    pub fn bsi_int_st(&mut self) -> BSI_INT_ST_W<INT_STAT_SPEC> {
        BSI_INT_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - The masked interrupt status of TWAIFD_RBNEI_INT. RX buffer not empty interrupt. Clearing this interrupt and not reading out content of RX Buffer via RX_DATA will re-activate the interrupt."]
    #[inline(always)]
    pub fn rbnei_int_st(&mut self) -> RBNEI_INT_ST_W<INT_STAT_SPEC> {
        RBNEI_INT_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - The masked interrupt status of TWAIFD_TXBHCI_INT. TXT buffer HW command interrupt. Anytime TXT buffer receives HW command from CAN Core which changes TXT buffer state to \"TX OK\", \"Error\" or \"Aborted\", this interrupt will be generated."]
    #[inline(always)]
    pub fn txbhci_int_st(&mut self) -> TXBHCI_INT_ST_W<INT_STAT_SPEC> {
        TXBHCI_INT_ST_W::new(self, 11)
    }
}
#[doc = "TWAI FD command register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STAT_SPEC;
impl crate::RegisterSpec for INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_stat::R`](R) reader structure"]
impl crate::Readable for INT_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_stat::W`](W) writer structure"]
impl crate::Writable for INT_STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_STAT to value 0"]
impl crate::Resettable for INT_STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
