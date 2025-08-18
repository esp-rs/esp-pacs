#[doc = "Register `LOG_DATA%s` reader"]
pub type R = crate::R<LOG_DATA_SPEC>;
#[doc = "Register `LOG_DATA%s` writer"]
pub type W = crate::W<LOG_DATA_SPEC>;
#[doc = "Field `LOG_DATA` reader - check data0"]
pub type LOG_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_DATA` writer - check data0"]
pub type LOG_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - check data0"]
    #[inline(always)]
    pub fn log_data(&self) -> LOG_DATA_R {
        LOG_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA")
            .field("log_data", &self.log_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - check data0"]
    #[inline(always)]
    pub fn log_data(&mut self) -> LOG_DATA_W<'_, LOG_DATA_SPEC> {
        LOG_DATA_W::new(self, 0)
    }
}
#[doc = "log check data register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_SPEC;
impl crate::RegisterSpec for LOG_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOG_DATA%s to value 0"]
impl crate::Resettable for LOG_DATA_SPEC {}
