#[doc = "Register `CH0_TASK_ID` reader"]
pub type R = crate::R<CH0_TASK_ID_SPEC>;
#[doc = "Register `CH0_TASK_ID` writer"]
pub type W = crate::W<CH0_TASK_ID_SPEC>;
#[doc = "Field `CH0_TASK_ID` reader - Configures ch0_task_id"]
pub type CH0_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH0_TASK_ID` writer - Configures ch0_task_id"]
pub type CH0_TASK_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures ch0_task_id"]
    #[inline(always)]
    pub fn ch0_task_id(&self) -> CH0_TASK_ID_R {
        CH0_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH0_TASK_ID")
            .field("ch0_task_id", &self.ch0_task_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures ch0_task_id"]
    #[inline(always)]
    pub fn ch0_task_id(&mut self) -> CH0_TASK_ID_W<CH0_TASK_ID_SPEC> {
        CH0_TASK_ID_W::new(self, 0)
    }
}
#[doc = "Channel0 task id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_task_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_task_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0_TASK_ID_SPEC;
impl crate::RegisterSpec for CH0_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_task_id::R`](R) reader structure"]
impl crate::Readable for CH0_TASK_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch0_task_id::W`](W) writer structure"]
impl crate::Writable for CH0_TASK_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0_TASK_ID to value 0"]
impl crate::Resettable for CH0_TASK_ID_SPEC {}
