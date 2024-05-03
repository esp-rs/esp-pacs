#[doc = "Register `CLUT_CNT` reader"]
pub type R = crate::R<CLUT_CNT_SPEC>;
#[doc = "Field `BLEND0_CLUT_CNT` reader - The write data counter of BLEND0 CLUT in fifo mode."]
pub type BLEND0_CLUT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `BLEND1_CLUT_CNT` reader - The write data counter of BLEND1 CLUT in fifo mode."]
pub type BLEND1_CLUT_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - The write data counter of BLEND0 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend0_clut_cnt(&self) -> BLEND0_CLUT_CNT_R {
        BLEND0_CLUT_CNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - The write data counter of BLEND1 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend1_clut_cnt(&self) -> BLEND1_CLUT_CNT_R {
        BLEND1_CLUT_CNT_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLUT_CNT")
            .field("blend0_clut_cnt", &self.blend0_clut_cnt().bits())
            .field("blend1_clut_cnt", &self.blend1_clut_cnt().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLUT_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "BLEND CLUT write counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clut_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLUT_CNT_SPEC;
impl crate::RegisterSpec for CLUT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clut_cnt::R`](R) reader structure"]
impl crate::Readable for CLUT_CNT_SPEC {}
#[doc = "`reset()` method sets CLUT_CNT to value 0"]
impl crate::Resettable for CLUT_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
