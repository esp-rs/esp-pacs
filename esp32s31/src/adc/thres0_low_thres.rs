#[doc = "Register `THRES0_LOW_THRES` reader"]
pub type R = crate::R<THRES0_LOW_THRES_SPEC>;
#[doc = "Register `THRES0_LOW_THRES` writer"]
pub type W = crate::W<THRES0_LOW_THRES_SPEC>;
#[doc = "Field `THRES0_LOW` reader - need_des"]
pub type THRES0_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `THRES0_LOW` writer - need_des"]
pub type THRES0_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES0_LOW_THRES")
            .field("thres0_low", &self.thres0_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W<'_, THRES0_LOW_THRES_SPEC> {
        THRES0_LOW_W::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres0_low_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres0_low_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES0_LOW_THRES_SPEC;
impl crate::RegisterSpec for THRES0_LOW_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres0_low_thres::R`](R) reader structure"]
impl crate::Readable for THRES0_LOW_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres0_low_thres::W`](W) writer structure"]
impl crate::Writable for THRES0_LOW_THRES_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES0_LOW_THRES to value 0"]
impl crate::Resettable for THRES0_LOW_THRES_SPEC {}
