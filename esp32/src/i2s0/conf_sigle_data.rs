#[doc = "Register `CONF_SIGLE_DATA` reader"]
pub type R = crate::R<CONF_SIGLE_DATA_SPEC>;
#[doc = "Register `CONF_SIGLE_DATA` writer"]
pub type W = crate::W<CONF_SIGLE_DATA_SPEC>;
#[doc = "Field `SIGLE_DATA` reader - "]
pub type SIGLE_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `SIGLE_DATA` writer - "]
pub type SIGLE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sigle_data(&self) -> SIGLE_DATA_R {
        SIGLE_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_SIGLE_DATA")
            .field("sigle_data", &self.sigle_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sigle_data(&mut self) -> SIGLE_DATA_W<'_, CONF_SIGLE_DATA_SPEC> {
        SIGLE_DATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`conf_sigle_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf_sigle_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SIGLE_DATA_SPEC;
impl crate::RegisterSpec for CONF_SIGLE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_sigle_data::R`](R) reader structure"]
impl crate::Readable for CONF_SIGLE_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_sigle_data::W`](W) writer structure"]
impl crate::Writable for CONF_SIGLE_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF_SIGLE_DATA to value 0"]
impl crate::Resettable for CONF_SIGLE_DATA_SPEC {}
