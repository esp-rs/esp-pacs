#[doc = "Register `STORE3` reader"]
pub type R = crate::R<STORE3_SPEC>;
#[doc = "Register `STORE3` writer"]
pub type W = crate::W<STORE3_SPEC>;
#[doc = "Field `LP_AON_STORE3` reader - store the software massege3 in always-on field"]
pub type LP_AON_STORE3_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE3` writer - store the software massege3 in always-on field"]
pub type LP_AON_STORE3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - store the software massege3 in always-on field"]
    #[inline(always)]
    pub fn lp_aon_store3(&self) -> LP_AON_STORE3_R {
        LP_AON_STORE3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE3")
            .field("lp_aon_store3", &self.lp_aon_store3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - store the software massege3 in always-on field"]
    #[inline(always)]
    pub fn lp_aon_store3(&mut self) -> LP_AON_STORE3_W<STORE3_SPEC> {
        LP_AON_STORE3_W::new(self, 0)
    }
}
#[doc = "store the software massege3 in always-on field\n\nYou can [`read`](crate::Reg::read) this register and get [`store3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`store3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE3_SPEC;
impl crate::RegisterSpec for STORE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store3::R`](R) reader structure"]
impl crate::Readable for STORE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store3::W`](W) writer structure"]
impl crate::Writable for STORE3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STORE3 to value 0"]
impl crate::Resettable for STORE3_SPEC {}
