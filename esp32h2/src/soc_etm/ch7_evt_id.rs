#[doc = "Register `CH7_EVT_ID` reader"]
pub type R = crate::R<CH7_EVT_ID_SPEC>;
#[doc = "Register `CH7_EVT_ID` writer"]
pub type W = crate::W<CH7_EVT_ID_SPEC>;
#[doc = "Field `CH7_EVT_ID` reader - ch7_evt_id"]
pub type CH7_EVT_ID_R = crate::FieldReader;
#[doc = "Field `CH7_EVT_ID` writer - ch7_evt_id"]
pub type CH7_EVT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ch7_evt_id"]
    #[inline(always)]
    pub fn ch7_evt_id(&self) -> CH7_EVT_ID_R {
        CH7_EVT_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH7_EVT_ID")
            .field("ch7_evt_id", &format_args!("{}", self.ch7_evt_id().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH7_EVT_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ch7_evt_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_evt_id(&mut self) -> CH7_EVT_ID_W<CH7_EVT_ID_SPEC> {
        CH7_EVT_ID_W::new(self, 0)
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
#[doc = "channel7 event id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7_evt_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7_evt_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH7_EVT_ID_SPEC;
impl crate::RegisterSpec for CH7_EVT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_evt_id::R`](R) reader structure"]
impl crate::Readable for CH7_EVT_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch7_evt_id::W`](W) writer structure"]
impl crate::Writable for CH7_EVT_ID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH7_EVT_ID to value 0"]
impl crate::Resettable for CH7_EVT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
