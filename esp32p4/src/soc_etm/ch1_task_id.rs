#[doc = "Register `CH1_TASK_ID` reader"]
pub type R = crate::R<CH1_TASK_ID_SPEC>;
#[doc = "Register `CH1_TASK_ID` writer"]
pub type W = crate::W<CH1_TASK_ID_SPEC>;
#[doc = "Field `CH1_TASK_ID` reader - Configures ch1_task_id"]
pub type CH1_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH1_TASK_ID` writer - Configures ch1_task_id"]
pub type CH1_TASK_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures ch1_task_id"]
    #[inline(always)]
    pub fn ch1_task_id(&self) -> CH1_TASK_ID_R {
        CH1_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_TASK_ID")
            .field(
                "ch1_task_id",
                &format_args!("{}", self.ch1_task_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH1_TASK_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures ch1_task_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_task_id(&mut self) -> CH1_TASK_ID_W<CH1_TASK_ID_SPEC> {
        CH1_TASK_ID_W::new(self, 0)
    }
}
#[doc = "Channel1 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_task_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_task_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_TASK_ID_SPEC;
impl crate::RegisterSpec for CH1_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_task_id::R`](R) reader structure"]
impl crate::Readable for CH1_TASK_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1_task_id::W`](W) writer structure"]
impl crate::Writable for CH1_TASK_ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_TASK_ID to value 0"]
impl crate::Resettable for CH1_TASK_ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
