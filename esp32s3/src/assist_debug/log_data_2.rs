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
            .field("log_data_2", &format_args!("{}", self.log_data_2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - check data2"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_2(&mut self) -> LOG_DATA_2_W<LOG_DATA_2_SPEC> {
        LOG_DATA_2_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "log check data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_2_SPEC;
impl crate::RegisterSpec for LOG_DATA_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data_2::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data_2::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_DATA_2 to value 0"]
impl crate::Resettable for LOG_DATA_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
