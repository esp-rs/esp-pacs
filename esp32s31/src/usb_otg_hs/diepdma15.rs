#[doc = "Register `DIEPDMA15` reader"]
pub type R = crate::R<DIEPDMA15_SPEC>;
#[doc = "Register `DIEPDMA15` writer"]
pub type W = crate::W<DIEPDMA15_SPEC>;
#[doc = "Field `DMAADDR` reader - Holds the start address of the external memory for storing or fetching endpoint data. Note: For control endpoints, this field stores control OUT data packets as well as SETUP transaction data packets. When more than three SETUP packets are received back-to-back, the SETUP data packet in the memory is overwritten. This register is incremented on every AHB transaction. The application can give only a DWORD-aligned address. - When Scatter/Gather DMA mode is not enabled, the application programs the start address value in this field. - When Scatter/Gather DMA mode is enabled, this field indicates the base pointer for the descriptor list."]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - Holds the start address of the external memory for storing or fetching endpoint data. Note: For control endpoints, this field stores control OUT data packets as well as SETUP transaction data packets. When more than three SETUP packets are received back-to-back, the SETUP data packet in the memory is overwritten. This register is incremented on every AHB transaction. The application can give only a DWORD-aligned address. - When Scatter/Gather DMA mode is not enabled, the application programs the start address value in this field. - When Scatter/Gather DMA mode is enabled, this field indicates the base pointer for the descriptor list."]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Holds the start address of the external memory for storing or fetching endpoint data. Note: For control endpoints, this field stores control OUT data packets as well as SETUP transaction data packets. When more than three SETUP packets are received back-to-back, the SETUP data packet in the memory is overwritten. This register is incremented on every AHB transaction. The application can give only a DWORD-aligned address. - When Scatter/Gather DMA mode is not enabled, the application programs the start address value in this field. - When Scatter/Gather DMA mode is enabled, this field indicates the base pointer for the descriptor list."]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMA15")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Holds the start address of the external memory for storing or fetching endpoint data. Note: For control endpoints, this field stores control OUT data packets as well as SETUP transaction data packets. When more than three SETUP packets are received back-to-back, the SETUP data packet in the memory is overwritten. This register is incremented on every AHB transaction. The application can give only a DWORD-aligned address. - When Scatter/Gather DMA mode is not enabled, the application programs the start address value in this field. - When Scatter/Gather DMA mode is enabled, this field indicates the base pointer for the descriptor list."]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, DIEPDMA15_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "This register contains the DMA Address for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA15_SPEC;
impl crate::RegisterSpec for DIEPDMA15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma15::R`](R) reader structure"]
impl crate::Readable for DIEPDMA15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma15::W`](W) writer structure"]
impl crate::Writable for DIEPDMA15_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA15 to value 0"]
impl crate::Resettable for DIEPDMA15_SPEC {}
