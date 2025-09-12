#[doc = "Register `STORE7` reader"]
pub type R = crate::R<STORE7_SPEC>;
#[doc = "Register `STORE7` writer"]
pub type W = crate::W<STORE7_SPEC>;
#[doc = "Field `LP_AON_STORE7` reader - need_des"]
pub type LP_AON_STORE7_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE7` writer - need_des"]
pub type LP_AON_STORE7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store7(&self) -> LP_AON_STORE7_R {
        LP_AON_STORE7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE7")
            .field("lp_aon_store7", &self.lp_aon_store7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store7(&mut self) -> LP_AON_STORE7_W<'_, STORE7_SPEC> {
        LP_AON_STORE7_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`store7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE7_SPEC;
impl crate::RegisterSpec for STORE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store7::R`](R) reader structure"]
impl crate::Readable for STORE7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store7::W`](W) writer structure"]
impl crate::Writable for STORE7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE7 to value 0"]
impl crate::Resettable for STORE7_SPEC {}
