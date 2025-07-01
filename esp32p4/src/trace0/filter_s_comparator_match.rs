#[doc = "Register `FILTER_S_COMPARATOR_MATCH` reader"]
pub type R = crate::R<FILTER_S_COMPARATOR_MATCH_SPEC>;
#[doc = "Register `FILTER_S_COMPARATOR_MATCH` writer"]
pub type W = crate::W<FILTER_S_COMPARATOR_MATCH_SPEC>;
#[doc = "Field `S_MATCH` reader - secondary comparator match value"]
pub type S_MATCH_R = crate::FieldReader<u32>;
#[doc = "Field `S_MATCH` writer - secondary comparator match value"]
pub type S_MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - secondary comparator match value"]
    #[inline(always)]
    pub fn s_match(&self) -> S_MATCH_R {
        S_MATCH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_S_COMPARATOR_MATCH")
            .field("s_match", &self.s_match())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - secondary comparator match value"]
    #[inline(always)]
    pub fn s_match(&mut self) -> S_MATCH_W<FILTER_S_COMPARATOR_MATCH_SPEC> {
        S_MATCH_W::new(self, 0)
    }
}
#[doc = "secondary comparator match value\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_s_comparator_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_s_comparator_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_S_COMPARATOR_MATCH_SPEC;
impl crate::RegisterSpec for FILTER_S_COMPARATOR_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_s_comparator_match::R`](R) reader structure"]
impl crate::Readable for FILTER_S_COMPARATOR_MATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_s_comparator_match::W`](W) writer structure"]
impl crate::Writable for FILTER_S_COMPARATOR_MATCH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_S_COMPARATOR_MATCH to value 0"]
impl crate::Resettable for FILTER_S_COMPARATOR_MATCH_SPEC {}
