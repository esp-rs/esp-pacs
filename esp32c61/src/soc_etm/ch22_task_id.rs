#[doc = "Register `CH22_TASK_ID` reader"]
pub type R = crate::R<CH22_TASK_ID_SPEC>;
#[doc = "Register `CH22_TASK_ID` writer"]
pub type W = crate::W<CH22_TASK_ID_SPEC>;
#[doc = "Field `CH22_TASK_ID` reader - Configures ch22_task_id"]
pub type CH22_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH22_TASK_ID` writer - Configures ch22_task_id"]
pub type CH22_TASK_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures ch22_task_id"]
    #[inline(always)]
    pub fn ch22_task_id(&self) -> CH22_TASK_ID_R {
        CH22_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH22_TASK_ID")
            .field("ch22_task_id", &self.ch22_task_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures ch22_task_id"]
    #[inline(always)]
    pub fn ch22_task_id(&mut self) -> CH22_TASK_ID_W<CH22_TASK_ID_SPEC> {
        CH22_TASK_ID_W::new(self, 0)
    }
}
#[doc = "Channel22 task id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch22_task_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch22_task_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH22_TASK_ID_SPEC;
impl crate::RegisterSpec for CH22_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch22_task_id::R`](R) reader structure"]
impl crate::Readable for CH22_TASK_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch22_task_id::W`](W) writer structure"]
impl crate::Writable for CH22_TASK_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH22_TASK_ID to value 0"]
impl crate::Resettable for CH22_TASK_ID_SPEC {}
