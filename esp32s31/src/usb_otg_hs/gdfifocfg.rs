#[doc = "Register `GDFIFOCFG` reader"]
pub type R = crate::R<GDFIFOCFG_SPEC>;
#[doc = "Register `GDFIFOCFG` writer"]
pub type W = crate::W<GDFIFOCFG_SPEC>;
#[doc = "Field `GDFIFOCFG` reader - GDFIFOCfg This field is for dynamic programming of the DFIFO Size. This value takes effect only when the application programs a non zero value to this register. The value programmed must conform to the guidelines described in 'FIFO RAM Allocation'. The core does not have any corrective logic if the FIFO sizes are programmed incorrectly."]
pub type GDFIFOCFG_R = crate::FieldReader<u16>;
#[doc = "Field `GDFIFOCFG` writer - GDFIFOCfg This field is for dynamic programming of the DFIFO Size. This value takes effect only when the application programs a non zero value to this register. The value programmed must conform to the guidelines described in 'FIFO RAM Allocation'. The core does not have any corrective logic if the FIFO sizes are programmed incorrectly."]
pub type GDFIFOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EPINFOBASEADDR` reader - This field provides the start address of the EP info controller. The EP info controller manages the stored values in the last few locations of the SPRAM as listed below. - Host Buffer DMA mode: One location per channel is used in SPRAM to store the HCDMAn value. - Host Scatter/Gather DMA mode: Four locations per channel are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer, and the Status Quadlet. - Device Buffer DMA mode: One location per endpoint direction is used in SPRAM to store the DIEPDMA and DOEPDMA value. - Device Scatter/Gather DMA mode: Four locations per endpoint direction are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer and the Status Quadlet."]
pub type EPINFOBASEADDR_R = crate::FieldReader<u16>;
#[doc = "Field `EPINFOBASEADDR` writer - This field provides the start address of the EP info controller. The EP info controller manages the stored values in the last few locations of the SPRAM as listed below. - Host Buffer DMA mode: One location per channel is used in SPRAM to store the HCDMAn value. - Host Scatter/Gather DMA mode: Four locations per channel are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer, and the Status Quadlet. - Device Buffer DMA mode: One location per endpoint direction is used in SPRAM to store the DIEPDMA and DOEPDMA value. - Device Scatter/Gather DMA mode: Four locations per endpoint direction are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer and the Status Quadlet."]
pub type EPINFOBASEADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GDFIFOCfg This field is for dynamic programming of the DFIFO Size. This value takes effect only when the application programs a non zero value to this register. The value programmed must conform to the guidelines described in 'FIFO RAM Allocation'. The core does not have any corrective logic if the FIFO sizes are programmed incorrectly."]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GDFIFOCFG_R {
        GDFIFOCFG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field provides the start address of the EP info controller. The EP info controller manages the stored values in the last few locations of the SPRAM as listed below. - Host Buffer DMA mode: One location per channel is used in SPRAM to store the HCDMAn value. - Host Scatter/Gather DMA mode: Four locations per channel are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer, and the Status Quadlet. - Device Buffer DMA mode: One location per endpoint direction is used in SPRAM to store the DIEPDMA and DOEPDMA value. - Device Scatter/Gather DMA mode: Four locations per endpoint direction are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer and the Status Quadlet."]
    #[inline(always)]
    pub fn epinfobaseaddr(&self) -> EPINFOBASEADDR_R {
        EPINFOBASEADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDFIFOCFG")
            .field("gdfifocfg", &self.gdfifocfg())
            .field("epinfobaseaddr", &self.epinfobaseaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - GDFIFOCfg This field is for dynamic programming of the DFIFO Size. This value takes effect only when the application programs a non zero value to this register. The value programmed must conform to the guidelines described in 'FIFO RAM Allocation'. The core does not have any corrective logic if the FIFO sizes are programmed incorrectly."]
    #[inline(always)]
    pub fn gdfifocfg(&mut self) -> GDFIFOCFG_W<'_, GDFIFOCFG_SPEC> {
        GDFIFOCFG_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field provides the start address of the EP info controller. The EP info controller manages the stored values in the last few locations of the SPRAM as listed below. - Host Buffer DMA mode: One location per channel is used in SPRAM to store the HCDMAn value. - Host Scatter/Gather DMA mode: Four locations per channel are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer, and the Status Quadlet. - Device Buffer DMA mode: One location per endpoint direction is used in SPRAM to store the DIEPDMA and DOEPDMA value. - Device Scatter/Gather DMA mode: Four locations per endpoint direction are used in SPRAM to store the Base Descriptor address, Current Descriptor address, Current Buffer Pointer and the Status Quadlet."]
    #[inline(always)]
    pub fn epinfobaseaddr(&mut self) -> EPINFOBASEADDR_W<'_, GDFIFOCFG_SPEC> {
        EPINFOBASEADDR_W::new(self, 16)
    }
}
#[doc = "Register to configure the DFIFOs for the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`gdfifocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdfifocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDFIFOCFG_SPEC;
impl crate::RegisterSpec for GDFIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdfifocfg::R`](R) reader structure"]
impl crate::Readable for GDFIFOCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdfifocfg::W`](W) writer structure"]
impl crate::Writable for GDFIFOCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GDFIFOCFG to value 0x0380_0400"]
impl crate::Resettable for GDFIFOCFG_SPEC {
    const RESET_VALUE: u32 = 0x0380_0400;
}
