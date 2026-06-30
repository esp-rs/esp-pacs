#[doc = "Register `SEL_AG1_METRIC_RANGE2` reader"]
pub type R = crate::R<SEL_AG1_METRIC_RANGE2_SPEC>;
#[doc = "Register `SEL_AG1_METRIC_RANGE2` writer"]
pub type W = crate::W<SEL_AG1_METRIC_RANGE2_SPEC>;
#[doc = "Field `CNT_HIGH` reader - The x Upper limit of interval statistics for sel metric in sel agent"]
pub type CNT_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_HIGH` writer - The x Upper limit of interval statistics for sel metric in sel agent"]
pub type CNT_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_LOW` reader - The x Lower limit of interval statistics for sel metric in sel agent"]
pub type CNT_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_LOW` writer - The x Lower limit of interval statistics for sel metric in sel agent"]
pub type CNT_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The x Upper limit of interval statistics for sel metric in sel agent"]
    #[inline(always)]
    pub fn cnt_high(&self) -> CNT_HIGH_R {
        CNT_HIGH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The x Lower limit of interval statistics for sel metric in sel agent"]
    #[inline(always)]
    pub fn cnt_low(&self) -> CNT_LOW_R {
        CNT_LOW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG1_METRIC_RANGE2")
            .field("cnt_high", &self.cnt_high())
            .field("cnt_low", &self.cnt_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The x Upper limit of interval statistics for sel metric in sel agent"]
    #[inline(always)]
    pub fn cnt_high(&mut self) -> CNT_HIGH_W<'_, SEL_AG1_METRIC_RANGE2_SPEC> {
        CNT_HIGH_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The x Lower limit of interval statistics for sel metric in sel agent"]
    #[inline(always)]
    pub fn cnt_low(&mut self) -> CNT_LOW_W<'_, SEL_AG1_METRIC_RANGE2_SPEC> {
        CNT_LOW_W::new(self, 16)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG1_METRIC_RANGE2_SPEC;
impl crate::RegisterSpec for SEL_AG1_METRIC_RANGE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag1_metric_range2::R`](R) reader structure"]
impl crate::Readable for SEL_AG1_METRIC_RANGE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag1_metric_range2::W`](W) writer structure"]
impl crate::Writable for SEL_AG1_METRIC_RANGE2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG1_METRIC_RANGE2 to value 0"]
impl crate::Resettable for SEL_AG1_METRIC_RANGE2_SPEC {}
