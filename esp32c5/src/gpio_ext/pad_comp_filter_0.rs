#[doc = "Register `PAD_COMP_FILTER_0` reader"]
pub type R = crate::R<PAD_COMP_FILTER_0_SPEC>;
#[doc = "Register `PAD_COMP_FILTER_0` writer"]
pub type W = crate::W<PAD_COMP_FILTER_0_SPEC>;
#[doc = "Field `ZERO_DET_FILTER_CNT_0` reader - Configures the period of masking new interrupt source foe analog PAD voltage comparator.\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
pub type ZERO_DET_FILTER_CNT_0_R = crate::FieldReader<u32>;
#[doc = "Field `ZERO_DET_FILTER_CNT_0` writer - Configures the period of masking new interrupt source foe analog PAD voltage comparator.\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
pub type ZERO_DET_FILTER_CNT_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the period of masking new interrupt source foe analog PAD voltage comparator.\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
    #[inline(always)]
    pub fn zero_det_filter_cnt_0(&self) -> ZERO_DET_FILTER_CNT_0_R {
        ZERO_DET_FILTER_CNT_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP_FILTER_0")
            .field("zero_det_filter_cnt_0", &self.zero_det_filter_cnt_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the period of masking new interrupt source foe analog PAD voltage comparator.\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
    #[inline(always)]
    pub fn zero_det_filter_cnt_0(&mut self) -> ZERO_DET_FILTER_CNT_0_W<'_, PAD_COMP_FILTER_0_SPEC> {
        ZERO_DET_FILTER_CNT_0_W::new(self, 0)
    }
}
#[doc = "Configuration register for interrupt source mask period of zero-crossing detection\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp_filter_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp_filter_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP_FILTER_0_SPEC;
impl crate::RegisterSpec for PAD_COMP_FILTER_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp_filter_0::R`](R) reader structure"]
impl crate::Readable for PAD_COMP_FILTER_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp_filter_0::W`](W) writer structure"]
impl crate::Writable for PAD_COMP_FILTER_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_COMP_FILTER_0 to value 0"]
impl crate::Resettable for PAD_COMP_FILTER_0_SPEC {}
