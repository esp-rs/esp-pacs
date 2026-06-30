#[doc = "Register `THRES1_HIGH_THRES` reader"]
pub type R = crate::R<THRES1_HIGH_THRES_SPEC>;
#[doc = "Register `THRES1_HIGH_THRES` writer"]
pub type W = crate::W<THRES1_HIGH_THRES_SPEC>;
#[doc = "Field `THRES1_HIGH` reader - need_des"]
pub type THRES1_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `THRES1_HIGH` writer - need_des"]
pub type THRES1_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn thres1_high(&self) -> THRES1_HIGH_R {
        THRES1_HIGH_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES1_HIGH_THRES")
            .field("thres1_high", &self.thres1_high())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn thres1_high(&mut self) -> THRES1_HIGH_W<'_, THRES1_HIGH_THRES_SPEC> {
        THRES1_HIGH_W::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres1_high_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres1_high_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES1_HIGH_THRES_SPEC;
impl crate::RegisterSpec for THRES1_HIGH_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres1_high_thres::R`](R) reader structure"]
impl crate::Readable for THRES1_HIGH_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres1_high_thres::W`](W) writer structure"]
impl crate::Writable for THRES1_HIGH_THRES_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES1_HIGH_THRES to value 0xff"]
impl crate::Resettable for THRES1_HIGH_THRES_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
