#[doc = "Register `APP_VECBASE_SET` reader"]
pub type R = crate::R<APP_VECBASE_SET_SPEC>;
#[doc = "Register `APP_VECBASE_SET` writer"]
pub type W = crate::W<APP_VECBASE_SET_SPEC>;
#[doc = "Field `APP_OUT_VECBASE` reader - "]
pub type APP_OUT_VECBASE_R = crate::FieldReader<u32>;
#[doc = "Field `APP_OUT_VECBASE` writer - "]
pub type APP_OUT_VECBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn app_out_vecbase(&self) -> APP_OUT_VECBASE_R {
        APP_OUT_VECBASE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_VECBASE_SET")
            .field("app_out_vecbase", &self.app_out_vecbase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn app_out_vecbase(&mut self) -> APP_OUT_VECBASE_W<'_, APP_VECBASE_SET_SPEC> {
        APP_OUT_VECBASE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_vecbase_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_vecbase_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_VECBASE_SET_SPEC;
impl crate::RegisterSpec for APP_VECBASE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_vecbase_set::R`](R) reader structure"]
impl crate::Readable for APP_VECBASE_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_vecbase_set::W`](W) writer structure"]
impl crate::Writable for APP_VECBASE_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APP_VECBASE_SET to value 0"]
impl crate::Resettable for APP_VECBASE_SET_SPEC {}
