#[doc = "Register `LOG_DATA_0` reader"]
pub type R = crate::R<LOG_DATA_0_SPEC>;
#[doc = "Register `LOG_DATA_0` writer"]
pub type W = crate::W<LOG_DATA_0_SPEC>;
#[doc = "Field `LOG_DATA_0` reader - reg_log_data_0"]
pub type LOG_DATA_0_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_DATA_0` writer - reg_log_data_0"]
pub type LOG_DATA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_log_data_0"]
    #[inline(always)]
    pub fn log_data_0(&self) -> LOG_DATA_0_R {
        LOG_DATA_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_0")
            .field("log_data_0", &format_args!("{}", self.log_data_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_log_data_0"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_0(&mut self) -> LOG_DATA_0_W<LOG_DATA_0_SPEC> {
        LOG_DATA_0_W::new(self, 0)
    }
}
#[doc = "ASSIST_DEBUG_LOG_DATA_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_0_SPEC;
impl crate::RegisterSpec for LOG_DATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data_0::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data_0::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOG_DATA_0 to value 0"]
impl crate::Resettable for LOG_DATA_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
