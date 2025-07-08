#[doc = "Register `CH13_EVT_ID` reader"]
pub type R = crate::R<CH13_EVT_ID_SPEC>;
#[doc = "Register `CH13_EVT_ID` writer"]
pub type W = crate::W<CH13_EVT_ID_SPEC>;
#[doc = "Field `CH13_EVT_ID` reader - Configures ch13_evt_id"]
pub type CH13_EVT_ID_R = crate::FieldReader;
#[doc = "Field `CH13_EVT_ID` writer - Configures ch13_evt_id"]
pub type CH13_EVT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configures ch13_evt_id"]
    #[inline(always)]
    pub fn ch13_evt_id(&self) -> CH13_EVT_ID_R {
        CH13_EVT_ID_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH13_EVT_ID")
            .field("ch13_evt_id", &self.ch13_evt_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures ch13_evt_id"]
    #[inline(always)]
    pub fn ch13_evt_id(&mut self) -> CH13_EVT_ID_W<CH13_EVT_ID_SPEC> {
        CH13_EVT_ID_W::new(self, 0)
    }
}
#[doc = "Channel13 event id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_evt_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_evt_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH13_EVT_ID_SPEC;
impl crate::RegisterSpec for CH13_EVT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch13_evt_id::R`](R) reader structure"]
impl crate::Readable for CH13_EVT_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch13_evt_id::W`](W) writer structure"]
impl crate::Writable for CH13_EVT_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH13_EVT_ID to value 0"]
impl crate::Resettable for CH13_EVT_ID_SPEC {}
