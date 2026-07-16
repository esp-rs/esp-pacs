#[doc = "Register `PDMA_INT_RAW` reader"]
pub type R = crate::R<PDMA_INT_RAW_SPEC>;
#[doc = "Register `PDMA_INT_RAW` writer"]
pub type W = crate::W<PDMA_INT_RAW_SPEC>;
#[doc = "Field `PDMA_EMPTY_INT_RAW` reader - need_des"]
pub type PDMA_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `PDMA_EMPTY_INT_RAW` writer - need_des"]
pub type PDMA_EMPTY_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn pdma_empty_int_raw(&self) -> PDMA_EMPTY_INT_RAW_R {
        PDMA_EMPTY_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMA_INT_RAW")
            .field("pdma_empty_int_raw", &self.pdma_empty_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn pdma_empty_int_raw(&mut self) -> PDMA_EMPTY_INT_RAW_W<'_, PDMA_INT_RAW_SPEC> {
        PDMA_EMPTY_INT_RAW_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pdma_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdma_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDMA_INT_RAW_SPEC;
impl crate::RegisterSpec for PDMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdma_int_raw::R`](R) reader structure"]
impl crate::Readable for PDMA_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdma_int_raw::W`](W) writer structure"]
impl crate::Writable for PDMA_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDMA_INT_RAW to value 0"]
impl crate::Resettable for PDMA_INT_RAW_SPEC {}
