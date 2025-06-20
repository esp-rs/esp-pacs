#[doc = "Register `INT_MASK_CLR` writer"]
pub type W = crate::W<INT_MASK_CLR_SPEC>;
#[doc = "Field `RXI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_RXI_INT_MASK_CLR ."]
pub type RXI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_TXI_INT_MASK_CLR ."]
pub type TXI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWLI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_EWLI_INT_MASK_CLR ."]
pub type EWLI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_DOI_INT_MASK_CLR ."]
pub type DOI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_FCSI_INT_MASK_CLR ."]
pub type FCSI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_ALI_INT_MASK_CLR ."]
pub type ALI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_BEI_INT_MASK_CLR ."]
pub type BEI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_OFI_INT_MASK_CLR ."]
pub type OFI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_RXFI_INT_MASK_CLR ."]
pub type RXFI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_BSI_INT_MASK_CLR ."]
pub type BSI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_RBNEI_INT_MASK_CLR ."]
pub type RBNEI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBHCI_INT_MASK_CLR` writer - Write 1 to clear TWAIFD_TXBHCI_INT_MASK_CLR ."]
pub type TXBHCI_INT_MASK_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_MASK_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear TWAIFD_RXI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn rxi_int_mask_clr(&mut self) -> RXI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        RXI_INT_MASK_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear TWAIFD_TXI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn txi_int_mask_clr(&mut self) -> TXI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        TXI_INT_MASK_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear TWAIFD_EWLI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn ewli_int_mask_clr(&mut self) -> EWLI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        EWLI_INT_MASK_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear TWAIFD_DOI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn doi_int_mask_clr(&mut self) -> DOI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        DOI_INT_MASK_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear TWAIFD_FCSI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn fcsi_int_mask_clr(&mut self) -> FCSI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        FCSI_INT_MASK_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear TWAIFD_ALI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn ali_int_mask_clr(&mut self) -> ALI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        ALI_INT_MASK_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear TWAIFD_BEI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn bei_int_mask_clr(&mut self) -> BEI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        BEI_INT_MASK_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear TWAIFD_OFI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn ofi_int_mask_clr(&mut self) -> OFI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        OFI_INT_MASK_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear TWAIFD_RXFI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn rxfi_int_mask_clr(&mut self) -> RXFI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        RXFI_INT_MASK_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to clear TWAIFD_BSI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn bsi_int_mask_clr(&mut self) -> BSI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        BSI_INT_MASK_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to clear TWAIFD_RBNEI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn rbnei_int_mask_clr(&mut self) -> RBNEI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        RBNEI_INT_MASK_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to clear TWAIFD_TXBHCI_INT_MASK_CLR ."]
    #[inline(always)]
    pub fn txbhci_int_mask_clr(&mut self) -> TXBHCI_INT_MASK_CLR_W<INT_MASK_CLR_SPEC> {
        TXBHCI_INT_MASK_CLR_W::new(self, 11)
    }
}
#[doc = "TWAI FD interrupt mask clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_MASK_CLR_SPEC;
impl crate::RegisterSpec for INT_MASK_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_mask_clr::W`](W) writer structure"]
impl crate::Writable for INT_MASK_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_MASK_CLR to value 0"]
impl crate::Resettable for INT_MASK_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
