#[doc = "Register `INT_MASK_SET` reader"]
pub type R = crate::R<INT_MASK_SET_SPEC>;
#[doc = "Register `INT_MASK_SET` writer"]
pub type W = crate::W<INT_MASK_SET_SPEC>;
#[doc = "Field `RXI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_RXI_INT."]
pub type RXI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `RXI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_RXI_INT."]
pub type RXI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_TXI_INT."]
pub type TXI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `TXI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_TXI_INT."]
pub type TXI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWLI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_EWLI_INT."]
pub type EWLI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `EWLI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_EWLI_INT."]
pub type EWLI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_DOI_INT."]
pub type DOI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `DOI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_DOI_INT."]
pub type DOI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_FCSI_INT."]
pub type FCSI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `FCSI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_FCSI_INT."]
pub type FCSI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_ALI_INT."]
pub type ALI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `ALI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_ALI_INT."]
pub type ALI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_BEI_INT."]
pub type BEI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `BEI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_BEI_INT."]
pub type BEI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_OFI_INT."]
pub type OFI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `OFI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_OFI_INT."]
pub type OFI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_RXFI_INT."]
pub type RXFI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `RXFI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_RXFI_INT."]
pub type RXFI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_BSI_INT."]
pub type BSI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `BSI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_BSI_INT."]
pub type BSI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_RBNEI_INT."]
pub type RBNEI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `RBNEI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_RBNEI_INT."]
pub type RBNEI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBHCI_INT_MASK_SET` reader - Write 1 to mask TWAIFD_TXBHCI_INT."]
pub type TXBHCI_INT_MASK_SET_R = crate::BitReader;
#[doc = "Field `TXBHCI_INT_MASK_SET` writer - Write 1 to mask TWAIFD_TXBHCI_INT."]
pub type TXBHCI_INT_MASK_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to mask TWAIFD_RXI_INT."]
    #[inline(always)]
    pub fn rxi_int_mask_set(&self) -> RXI_INT_MASK_SET_R {
        RXI_INT_MASK_SET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to mask TWAIFD_TXI_INT."]
    #[inline(always)]
    pub fn txi_int_mask_set(&self) -> TXI_INT_MASK_SET_R {
        TXI_INT_MASK_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to mask TWAIFD_EWLI_INT."]
    #[inline(always)]
    pub fn ewli_int_mask_set(&self) -> EWLI_INT_MASK_SET_R {
        EWLI_INT_MASK_SET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to mask TWAIFD_DOI_INT."]
    #[inline(always)]
    pub fn doi_int_mask_set(&self) -> DOI_INT_MASK_SET_R {
        DOI_INT_MASK_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to mask TWAIFD_FCSI_INT."]
    #[inline(always)]
    pub fn fcsi_int_mask_set(&self) -> FCSI_INT_MASK_SET_R {
        FCSI_INT_MASK_SET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to mask TWAIFD_ALI_INT."]
    #[inline(always)]
    pub fn ali_int_mask_set(&self) -> ALI_INT_MASK_SET_R {
        ALI_INT_MASK_SET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to mask TWAIFD_BEI_INT."]
    #[inline(always)]
    pub fn bei_int_mask_set(&self) -> BEI_INT_MASK_SET_R {
        BEI_INT_MASK_SET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to mask TWAIFD_OFI_INT."]
    #[inline(always)]
    pub fn ofi_int_mask_set(&self) -> OFI_INT_MASK_SET_R {
        OFI_INT_MASK_SET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to mask TWAIFD_RXFI_INT."]
    #[inline(always)]
    pub fn rxfi_int_mask_set(&self) -> RXFI_INT_MASK_SET_R {
        RXFI_INT_MASK_SET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to mask TWAIFD_BSI_INT."]
    #[inline(always)]
    pub fn bsi_int_mask_set(&self) -> BSI_INT_MASK_SET_R {
        BSI_INT_MASK_SET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write 1 to mask TWAIFD_RBNEI_INT."]
    #[inline(always)]
    pub fn rbnei_int_mask_set(&self) -> RBNEI_INT_MASK_SET_R {
        RBNEI_INT_MASK_SET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write 1 to mask TWAIFD_TXBHCI_INT."]
    #[inline(always)]
    pub fn txbhci_int_mask_set(&self) -> TXBHCI_INT_MASK_SET_R {
        TXBHCI_INT_MASK_SET_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MASK_SET")
            .field("rxi_int_mask_set", &self.rxi_int_mask_set())
            .field("txi_int_mask_set", &self.txi_int_mask_set())
            .field("ewli_int_mask_set", &self.ewli_int_mask_set())
            .field("doi_int_mask_set", &self.doi_int_mask_set())
            .field("fcsi_int_mask_set", &self.fcsi_int_mask_set())
            .field("ali_int_mask_set", &self.ali_int_mask_set())
            .field("bei_int_mask_set", &self.bei_int_mask_set())
            .field("ofi_int_mask_set", &self.ofi_int_mask_set())
            .field("rxfi_int_mask_set", &self.rxfi_int_mask_set())
            .field("bsi_int_mask_set", &self.bsi_int_mask_set())
            .field("rbnei_int_mask_set", &self.rbnei_int_mask_set())
            .field("txbhci_int_mask_set", &self.txbhci_int_mask_set())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to mask TWAIFD_RXI_INT."]
    #[inline(always)]
    pub fn rxi_int_mask_set(&mut self) -> RXI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        RXI_INT_MASK_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to mask TWAIFD_TXI_INT."]
    #[inline(always)]
    pub fn txi_int_mask_set(&mut self) -> TXI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        TXI_INT_MASK_SET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to mask TWAIFD_EWLI_INT."]
    #[inline(always)]
    pub fn ewli_int_mask_set(&mut self) -> EWLI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        EWLI_INT_MASK_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to mask TWAIFD_DOI_INT."]
    #[inline(always)]
    pub fn doi_int_mask_set(&mut self) -> DOI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        DOI_INT_MASK_SET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to mask TWAIFD_FCSI_INT."]
    #[inline(always)]
    pub fn fcsi_int_mask_set(&mut self) -> FCSI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        FCSI_INT_MASK_SET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to mask TWAIFD_ALI_INT."]
    #[inline(always)]
    pub fn ali_int_mask_set(&mut self) -> ALI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        ALI_INT_MASK_SET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to mask TWAIFD_BEI_INT."]
    #[inline(always)]
    pub fn bei_int_mask_set(&mut self) -> BEI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        BEI_INT_MASK_SET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to mask TWAIFD_OFI_INT."]
    #[inline(always)]
    pub fn ofi_int_mask_set(&mut self) -> OFI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        OFI_INT_MASK_SET_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to mask TWAIFD_RXFI_INT."]
    #[inline(always)]
    pub fn rxfi_int_mask_set(&mut self) -> RXFI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        RXFI_INT_MASK_SET_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to mask TWAIFD_BSI_INT."]
    #[inline(always)]
    pub fn bsi_int_mask_set(&mut self) -> BSI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        BSI_INT_MASK_SET_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to mask TWAIFD_RBNEI_INT."]
    #[inline(always)]
    pub fn rbnei_int_mask_set(&mut self) -> RBNEI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        RBNEI_INT_MASK_SET_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to mask TWAIFD_TXBHCI_INT."]
    #[inline(always)]
    pub fn txbhci_int_mask_set(&mut self) -> TXBHCI_INT_MASK_SET_W<INT_MASK_SET_SPEC> {
        TXBHCI_INT_MASK_SET_W::new(self, 11)
    }
}
#[doc = "TWAI FD interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_MASK_SET_SPEC;
impl crate::RegisterSpec for INT_MASK_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask_set::R`](R) reader structure"]
impl crate::Readable for INT_MASK_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_mask_set::W`](W) writer structure"]
impl crate::Writable for INT_MASK_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MASK_SET to value 0"]
impl crate::Resettable for INT_MASK_SET_SPEC {}
