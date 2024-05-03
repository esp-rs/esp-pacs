#[doc = "Register `STATUS2` reader"]
pub type R = crate::R<STATUS2_SPEC>;
#[doc = "Field `SOURCE_PIXEL` reader - source pixels fetched from dma"]
pub type SOURCE_PIXEL_R = crate::FieldReader<u32>;
#[doc = "Field `LAST_BLOCK` reader - indicate the encoding process for the last mcu of the picture"]
pub type LAST_BLOCK_R = crate::BitReader;
#[doc = "Field `LAST_MCU` reader - indicate the encoding process for the last block of the picture"]
pub type LAST_MCU_R = crate::BitReader;
#[doc = "Field `LAST_DC` reader - indicate the encoding process is at the header of the last block of the picture"]
pub type LAST_DC_R = crate::BitReader;
#[doc = "Field `PACKFIFO_READY` reader - the jpeg pack_fifo ready signal, high active"]
pub type PACKFIFO_READY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - source pixels fetched from dma"]
    #[inline(always)]
    pub fn source_pixel(&self) -> SOURCE_PIXEL_R {
        SOURCE_PIXEL_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - indicate the encoding process for the last mcu of the picture"]
    #[inline(always)]
    pub fn last_block(&self) -> LAST_BLOCK_R {
        LAST_BLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - indicate the encoding process for the last block of the picture"]
    #[inline(always)]
    pub fn last_mcu(&self) -> LAST_MCU_R {
        LAST_MCU_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - indicate the encoding process is at the header of the last block of the picture"]
    #[inline(always)]
    pub fn last_dc(&self) -> LAST_DC_R {
        LAST_DC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the jpeg pack_fifo ready signal, high active"]
    #[inline(always)]
    pub fn packfifo_ready(&self) -> PACKFIFO_READY_R {
        PACKFIFO_READY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS2")
            .field("source_pixel", &self.source_pixel().bits())
            .field("last_block", &self.last_block().bit())
            .field("last_mcu", &self.last_mcu().bit())
            .field("last_dc", &self.last_dc().bit())
            .field("packfifo_ready", &self.packfifo_ready().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS2_SPEC;
impl crate::RegisterSpec for STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status2::R`](R) reader structure"]
impl crate::Readable for STATUS2_SPEC {}
#[doc = "`reset()` method sets STATUS2 to value 0x0800_0000"]
impl crate::Resettable for STATUS2_SPEC {
    const RESET_VALUE: u32 = 0x0800_0000;
}
