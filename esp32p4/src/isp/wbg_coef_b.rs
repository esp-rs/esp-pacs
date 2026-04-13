#[doc = "Register `WBG_COEF_B` reader"]
pub type R = crate::R<WBG_COEF_B_SPEC>;
#[doc = "Register `WBG_COEF_B` writer"]
pub type W = crate::W<WBG_COEF_B_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WBG_COEF_B")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, WBG_COEF_B_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WBG_COEF_B_SPEC;
impl crate::RegisterSpec for WBG_COEF_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wbg_coef_b::R`](R) reader structure"]
impl crate::Readable for WBG_COEF_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wbg_coef_b::W`](W) writer structure"]
impl crate::Writable for WBG_COEF_B_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WBG_COEF_B to value 0"]
impl crate::Resettable for WBG_COEF_B_SPEC {}
