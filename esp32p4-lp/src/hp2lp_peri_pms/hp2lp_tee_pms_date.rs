#[doc = "Register `HP2LP_TEE_PMS_DATE` reader"]
pub type R = crate::R<HP2LP_TEE_PMS_DATE_SPEC>;
#[doc = "Register `HP2LP_TEE_PMS_DATE` writer"]
pub type W = crate::W<HP2LP_TEE_PMS_DATE_SPEC>;
#[doc = "Field `TEE_DATE` reader - NA"]
pub type TEE_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `TEE_DATE` writer - NA"]
pub type TEE_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn tee_date(&self) -> TEE_DATE_R {
        TEE_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP2LP_TEE_PMS_DATE")
            .field("tee_date", &self.tee_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn tee_date(&mut self) -> TEE_DATE_W<'_, HP2LP_TEE_PMS_DATE_SPEC> {
        TEE_DATE_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_tee_pms_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_tee_pms_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP2LP_TEE_PMS_DATE_SPEC;
impl crate::RegisterSpec for HP2LP_TEE_PMS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_tee_pms_date::R`](R) reader structure"]
impl crate::Readable for HP2LP_TEE_PMS_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp2lp_tee_pms_date::W`](W) writer structure"]
impl crate::Writable for HP2LP_TEE_PMS_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP2LP_TEE_PMS_DATE to value 0"]
impl crate::Resettable for HP2LP_TEE_PMS_DATE_SPEC {}
