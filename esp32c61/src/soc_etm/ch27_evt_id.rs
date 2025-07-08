#[doc = "Register `CH27_EVT_ID` reader"]
pub type R = crate::R<CH27_EVT_ID_SPEC>;
#[doc = "Register `CH27_EVT_ID` writer"]
pub type W = crate::W<CH27_EVT_ID_SPEC>;
#[doc = "Field `CH27_EVT_ID` reader - Configures ch27_evt_id"]
pub type CH27_EVT_ID_R = crate::FieldReader;
#[doc = "Field `CH27_EVT_ID` writer - Configures ch27_evt_id"]
pub type CH27_EVT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configures ch27_evt_id"]
    #[inline(always)]
    pub fn ch27_evt_id(&self) -> CH27_EVT_ID_R {
        CH27_EVT_ID_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH27_EVT_ID")
            .field("ch27_evt_id", &self.ch27_evt_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures ch27_evt_id"]
    #[inline(always)]
    pub fn ch27_evt_id(&mut self) -> CH27_EVT_ID_W<CH27_EVT_ID_SPEC> {
        CH27_EVT_ID_W::new(self, 0)
    }
}
#[doc = "Channel27 event id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch27_evt_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch27_evt_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH27_EVT_ID_SPEC;
impl crate::RegisterSpec for CH27_EVT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch27_evt_id::R`](R) reader structure"]
impl crate::Readable for CH27_EVT_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch27_evt_id::W`](W) writer structure"]
impl crate::Writable for CH27_EVT_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH27_EVT_ID to value 0"]
impl crate::Resettable for CH27_EVT_ID_SPEC {}
