#[doc = "Register `ICM_VER_DATE` reader"]
pub type R = crate::R<ICM_VER_DATE_SPEC>;
#[doc = "Register `ICM_VER_DATE` writer"]
pub type W = crate::W<ICM_VER_DATE_SPEC>;
#[doc = "Field `VER_DATE` reader - "]
pub type VER_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `VER_DATE` writer - "]
pub type VER_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ver_date(&self) -> VER_DATE_R {
        VER_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_VER_DATE")
            .field("ver_date", &self.ver_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ver_date(&mut self) -> VER_DATE_W<'_, ICM_VER_DATE_SPEC> {
        VER_DATE_W::new(self, 0)
    }
}
#[doc = "ICM version / date\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_VER_DATE_SPEC;
impl crate::RegisterSpec for ICM_VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_ver_date::R`](R) reader structure"]
impl crate::Readable for ICM_VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_ver_date::W`](W) writer structure"]
impl crate::Writable for ICM_VER_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_VER_DATE to value 0"]
impl crate::Resettable for ICM_VER_DATE_SPEC {}
