///Register `LOG_MAX` reader
pub type R = crate::R<LOG_MAX_SPEC>;
///Register `LOG_MAX` writer
pub type W = crate::W<LOG_MAX_SPEC>;
///Field `LOG_MAX` reader - reg_log_max
pub type LOG_MAX_R = crate::FieldReader<u32>;
///Field `LOG_MAX` writer - reg_log_max
pub type LOG_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reg_log_max
    #[inline(always)]
    pub fn log_max(&self) -> LOG_MAX_R {
        LOG_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MAX")
            .field("log_max", &self.log_max())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - reg_log_max
    #[inline(always)]
    #[must_use]
    pub fn log_max(&mut self) -> LOG_MAX_W<LOG_MAX_SPEC> {
        LOG_MAX_W::new(self, 0)
    }
}
/**ASSIST_DEBUG_LOG_MAX_REG

You can [`read`](crate::generic::Reg::read) this register and get [`log_max::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_max::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOG_MAX_SPEC;
impl crate::RegisterSpec for LOG_MAX_SPEC {
    type Ux = u32;
}
///`read()` method returns [`log_max::R`](R) reader structure
impl crate::Readable for LOG_MAX_SPEC {}
///`write(|w| ..)` method takes [`log_max::W`](W) writer structure
impl crate::Writable for LOG_MAX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOG_MAX to value 0
impl crate::Resettable for LOG_MAX_SPEC {
    const RESET_VALUE: u32 = 0;
}
