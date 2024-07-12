#[doc = "Register `STORE9` reader"]
pub type R = crate::R<STORE9_SPEC>;
#[doc = "Register `STORE9` writer"]
pub type W = crate::W<STORE9_SPEC>;
#[doc = "Field `LP_AON_STORE9` reader - need_des"]
pub type LP_AON_STORE9_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE9` writer - need_des"]
pub type LP_AON_STORE9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store9(&self) -> LP_AON_STORE9_R {
        LP_AON_STORE9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE9")
            .field("lp_aon_store9", &self.lp_aon_store9())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store9(&mut self) -> LP_AON_STORE9_W<STORE9_SPEC> {
        LP_AON_STORE9_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`store9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE9_SPEC;
impl crate::RegisterSpec for STORE9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store9::R`](R) reader structure"]
impl crate::Readable for STORE9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store9::W`](W) writer structure"]
impl crate::Writable for STORE9_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STORE9 to value 0"]
impl crate::Resettable for STORE9_SPEC {
    const RESET_VALUE: u32 = 0;
}
