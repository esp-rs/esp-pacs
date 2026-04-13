#[doc = "Register `EXTD_CONFIG` reader"]
pub type R = crate::R<EXTD_CONFIG_SPEC>;
#[doc = "Register `EXTD_CONFIG` writer"]
pub type W = crate::W<EXTD_CONFIG_SPEC>;
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
        f.debug_struct("EXTD_CONFIG")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, EXTD_CONFIG_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`extd_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extd_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTD_CONFIG_SPEC;
impl crate::RegisterSpec for EXTD_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extd_config::R`](R) reader structure"]
impl crate::Readable for EXTD_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extd_config::W`](W) writer structure"]
impl crate::Writable for EXTD_CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTD_CONFIG to value 0"]
impl crate::Resettable for EXTD_CONFIG_SPEC {}
