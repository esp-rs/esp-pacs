#[doc = "Register `APP_VECBASE_CTRL` reader"]
pub type R = crate::R<APP_VECBASE_CTRL_SPEC>;
#[doc = "Register `APP_VECBASE_CTRL` writer"]
pub type W = crate::W<APP_VECBASE_CTRL_SPEC>;
#[doc = "Field `APP_OUT_VECBASE_SEL` reader - "]
pub type APP_OUT_VECBASE_SEL_R = crate::FieldReader;
#[doc = "Field `APP_OUT_VECBASE_SEL` writer - "]
pub type APP_OUT_VECBASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn app_out_vecbase_sel(&self) -> APP_OUT_VECBASE_SEL_R {
        APP_OUT_VECBASE_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_VECBASE_CTRL")
            .field("app_out_vecbase_sel", &self.app_out_vecbase_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn app_out_vecbase_sel(&mut self) -> APP_OUT_VECBASE_SEL_W<APP_VECBASE_CTRL_SPEC> {
        APP_OUT_VECBASE_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_vecbase_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_vecbase_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_VECBASE_CTRL_SPEC;
impl crate::RegisterSpec for APP_VECBASE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_vecbase_ctrl::R`](R) reader structure"]
impl crate::Readable for APP_VECBASE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_vecbase_ctrl::W`](W) writer structure"]
impl crate::Writable for APP_VECBASE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_VECBASE_CTRL to value 0"]
impl crate::Resettable for APP_VECBASE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
