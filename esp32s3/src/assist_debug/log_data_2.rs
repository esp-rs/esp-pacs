#[doc = "Register `LOG_DATA_2` reader"]
pub type R = crate::R<LOG_DATA_2_SPEC>;
#[doc = "Register `LOG_DATA_2` writer"]
pub type W = crate::W<LOG_DATA_2_SPEC>;
#[doc = "Field `LOG_DATA_2` reader - check data2"]
pub type LOG_DATA_2_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_DATA_2` writer - check data2"]
pub type LOG_DATA_2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - check data2"]
    #[inline(always)]
    pub fn log_data_2(&self) -> LOG_DATA_2_R {
        LOG_DATA_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_2")
            .field("log_data_2", &self.log_data_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - check data2"]
    #[inline(always)]
    pub fn log_data_2(&mut self) -> LOG_DATA_2_W<LOG_DATA_2_SPEC> {
        LOG_DATA_2_W::new(self, 0)
    }
}
#[doc = "log check data register\n\nYou can [`read`](crate::Reg::read) this register and get [`log_data_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`log_data_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_2_SPEC;
impl crate::RegisterSpec for LOG_DATA_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data_2::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data_2::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOG_DATA_2 to value 0"]
impl crate::Resettable for LOG_DATA_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
