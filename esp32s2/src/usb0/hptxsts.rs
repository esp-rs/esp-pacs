#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HPTXSTS_SPEC>;
#[doc = "Field `PTXFSPCAVAIL` reader - "]
pub type PTXFSPCAVAIL_R = crate::FieldReader<u16>;
#[doc = "Field `PTXQSPCAVAIL` reader - "]
pub type PTXQSPCAVAIL_R = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - "]
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfspcavail", &self.ptxfspcavail().bits())
            .field("ptxqspcavail", &self.ptxqspcavail().bits())
            .field("ptxqtop", &self.ptxqtop().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HPTXSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: u32 = 0x0008_0100;
}
