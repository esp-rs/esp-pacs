#[doc = "Register `CH34_EVT_ID` reader"]
pub type R = crate::R<CH34_EVT_ID_SPEC>;
#[doc = "Register `CH34_EVT_ID` writer"]
pub type W = crate::W<CH34_EVT_ID_SPEC>;
#[doc = "Field `CH34_EVT_ID` reader - Configures ch34_evt_id"]
pub type CH34_EVT_ID_R = crate::FieldReader;
#[doc = "Field `CH34_EVT_ID` writer - Configures ch34_evt_id"]
pub type CH34_EVT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configures ch34_evt_id"]
    #[inline(always)]
    pub fn ch34_evt_id(&self) -> CH34_EVT_ID_R {
        CH34_EVT_ID_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH34_EVT_ID")
            .field("ch34_evt_id", &self.ch34_evt_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures ch34_evt_id"]
    #[inline(always)]
    pub fn ch34_evt_id(&mut self) -> CH34_EVT_ID_W<CH34_EVT_ID_SPEC> {
        CH34_EVT_ID_W::new(self, 0)
    }
}
#[doc = "Channel34 event id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch34_evt_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch34_evt_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH34_EVT_ID_SPEC;
impl crate::RegisterSpec for CH34_EVT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch34_evt_id::R`](R) reader structure"]
impl crate::Readable for CH34_EVT_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch34_evt_id::W`](W) writer structure"]
impl crate::Writable for CH34_EVT_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH34_EVT_ID to value 0"]
impl crate::Resettable for CH34_EVT_ID_SPEC {}
