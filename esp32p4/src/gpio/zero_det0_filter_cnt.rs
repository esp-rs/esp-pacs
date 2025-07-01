#[doc = "Register `ZERO_DET0_FILTER_CNT` reader"]
pub type R = crate::R<ZERO_DET0_FILTER_CNT_SPEC>;
#[doc = "Register `ZERO_DET0_FILTER_CNT` writer"]
pub type W = crate::W<ZERO_DET0_FILTER_CNT_SPEC>;
#[doc = "Field `ZERO_DET0_FILTER_CNT` reader - GPIO analog comparator zero detect filter count"]
pub type ZERO_DET0_FILTER_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `ZERO_DET0_FILTER_CNT` writer - GPIO analog comparator zero detect filter count"]
pub type ZERO_DET0_FILTER_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO analog comparator zero detect filter count"]
    #[inline(always)]
    pub fn zero_det0_filter_cnt(&self) -> ZERO_DET0_FILTER_CNT_R {
        ZERO_DET0_FILTER_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZERO_DET0_FILTER_CNT")
            .field("zero_det0_filter_cnt", &self.zero_det0_filter_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO analog comparator zero detect filter count"]
    #[inline(always)]
    pub fn zero_det0_filter_cnt(&mut self) -> ZERO_DET0_FILTER_CNT_W<ZERO_DET0_FILTER_CNT_SPEC> {
        ZERO_DET0_FILTER_CNT_W::new(self, 0)
    }
}
#[doc = "GPIO analog comparator zero detect filter count\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det0_filter_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det0_filter_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZERO_DET0_FILTER_CNT_SPEC;
impl crate::RegisterSpec for ZERO_DET0_FILTER_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_det0_filter_cnt::R`](R) reader structure"]
impl crate::Readable for ZERO_DET0_FILTER_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zero_det0_filter_cnt::W`](W) writer structure"]
impl crate::Writable for ZERO_DET0_FILTER_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZERO_DET0_FILTER_CNT to value 0xffff_ffff"]
impl crate::Resettable for ZERO_DET0_FILTER_CNT_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
