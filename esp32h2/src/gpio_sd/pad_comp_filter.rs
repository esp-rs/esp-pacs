#[doc = "Register `PAD_COMP_FILTER` reader"]
pub type R = crate::R<PAD_COMP_FILTER_SPEC>;
#[doc = "Register `PAD_COMP_FILTER` writer"]
pub type W = crate::W<PAD_COMP_FILTER_SPEC>;
#[doc = "Field `ZERO_DET_FILTER_CNT` reader - Zero Detect filter cycle length"]
pub type ZERO_DET_FILTER_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `ZERO_DET_FILTER_CNT` writer - Zero Detect filter cycle length"]
pub type ZERO_DET_FILTER_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Zero Detect filter cycle length"]
    #[inline(always)]
    pub fn zero_det_filter_cnt(&self) -> ZERO_DET_FILTER_CNT_R {
        ZERO_DET_FILTER_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP_FILTER")
            .field(
                "zero_det_filter_cnt",
                &format_args!("{}", self.zero_det_filter_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD_COMP_FILTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Zero Detect filter cycle length"]
    #[inline(always)]
    #[must_use]
    pub fn zero_det_filter_cnt(&mut self) -> ZERO_DET_FILTER_CNT_W<PAD_COMP_FILTER_SPEC> {
        ZERO_DET_FILTER_CNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Zero Detect filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP_FILTER_SPEC;
impl crate::RegisterSpec for PAD_COMP_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp_filter::R`](R) reader structure"]
impl crate::Readable for PAD_COMP_FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp_filter::W`](W) writer structure"]
impl crate::Writable for PAD_COMP_FILTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_COMP_FILTER to value 0"]
impl crate::Resettable for PAD_COMP_FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
