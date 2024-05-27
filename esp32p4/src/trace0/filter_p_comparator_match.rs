///Register `FILTER_P_COMPARATOR_MATCH` reader
pub type R = crate::R<FILTER_P_COMPARATOR_MATCH_SPEC>;
///Register `FILTER_P_COMPARATOR_MATCH` writer
pub type W = crate::W<FILTER_P_COMPARATOR_MATCH_SPEC>;
///Field `P_MATCH` reader - primary comparator match value
pub type P_MATCH_R = crate::FieldReader<u32>;
///Field `P_MATCH` writer - primary comparator match value
pub type P_MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - primary comparator match value
    #[inline(always)]
    pub fn p_match(&self) -> P_MATCH_R {
        P_MATCH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_P_COMPARATOR_MATCH")
            .field("p_match", &self.p_match())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - primary comparator match value
    #[inline(always)]
    #[must_use]
    pub fn p_match(&mut self) -> P_MATCH_W<FILTER_P_COMPARATOR_MATCH_SPEC> {
        P_MATCH_W::new(self, 0)
    }
}
/**primary comparator match value

You can [`read`](crate::generic::Reg::read) this register and get [`filter_p_comparator_match::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_p_comparator_match::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FILTER_P_COMPARATOR_MATCH_SPEC;
impl crate::RegisterSpec for FILTER_P_COMPARATOR_MATCH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`filter_p_comparator_match::R`](R) reader structure
impl crate::Readable for FILTER_P_COMPARATOR_MATCH_SPEC {}
///`write(|w| ..)` method takes [`filter_p_comparator_match::W`](W) writer structure
impl crate::Writable for FILTER_P_COMPARATOR_MATCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FILTER_P_COMPARATOR_MATCH to value 0
impl crate::Resettable for FILTER_P_COMPARATOR_MATCH_SPEC {
    const RESET_VALUE: u32 = 0;
}
