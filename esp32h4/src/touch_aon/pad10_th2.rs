#[doc = "Register `PAD10_TH2` reader"]
pub type R = crate::R<PAD10_TH2_SPEC>;
#[doc = "Register `PAD10_TH2` writer"]
pub type W = crate::W<PAD10_TH2_SPEC>;
#[doc = "Field `TOUCH_PAD10_TH2` reader - Reserved"]
pub type TOUCH_PAD10_TH2_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD10_TH2` writer - Reserved"]
pub type TOUCH_PAD10_TH2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad10_th2(&self) -> TOUCH_PAD10_TH2_R {
        TOUCH_PAD10_TH2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD10_TH2")
            .field("touch_pad10_th2", &self.touch_pad10_th2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn touch_pad10_th2(&mut self) -> TOUCH_PAD10_TH2_W<'_, PAD10_TH2_SPEC> {
        TOUCH_PAD10_TH2_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad10_th2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad10_th2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD10_TH2_SPEC;
impl crate::RegisterSpec for PAD10_TH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad10_th2::R`](R) reader structure"]
impl crate::Readable for PAD10_TH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad10_th2::W`](W) writer structure"]
impl crate::Writable for PAD10_TH2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD10_TH2 to value 0"]
impl crate::Resettable for PAD10_TH2_SPEC {}
