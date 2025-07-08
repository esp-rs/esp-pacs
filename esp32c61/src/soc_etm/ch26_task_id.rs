#[doc = "Register `CH26_TASK_ID` reader"]
pub type R = crate::R<CH26_TASK_ID_SPEC>;
#[doc = "Register `CH26_TASK_ID` writer"]
pub type W = crate::W<CH26_TASK_ID_SPEC>;
#[doc = "Field `CH26_TASK_ID` reader - Configures ch26_task_id"]
pub type CH26_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH26_TASK_ID` writer - Configures ch26_task_id"]
pub type CH26_TASK_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures ch26_task_id"]
    #[inline(always)]
    pub fn ch26_task_id(&self) -> CH26_TASK_ID_R {
        CH26_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH26_TASK_ID")
            .field("ch26_task_id", &self.ch26_task_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures ch26_task_id"]
    #[inline(always)]
    pub fn ch26_task_id(&mut self) -> CH26_TASK_ID_W<CH26_TASK_ID_SPEC> {
        CH26_TASK_ID_W::new(self, 0)
    }
}
#[doc = "Channel26 task id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch26_task_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch26_task_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH26_TASK_ID_SPEC;
impl crate::RegisterSpec for CH26_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch26_task_id::R`](R) reader structure"]
impl crate::Readable for CH26_TASK_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch26_task_id::W`](W) writer structure"]
impl crate::Writable for CH26_TASK_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH26_TASK_ID to value 0"]
impl crate::Resettable for CH26_TASK_ID_SPEC {}
