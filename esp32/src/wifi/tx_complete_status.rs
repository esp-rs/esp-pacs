#[doc = "Register `TX_COMPLETE_STATUS` reader"]
pub type R = crate::R<TX_COMPLETE_STATUS_SPEC>;
#[doc = "Register `TX_COMPLETE_STATUS` writer"]
pub type W = crate::W<TX_COMPLETE_STATUS_SPEC>;
#[doc = "Field `SLOTS` reader - "]
pub type SLOTS_R = crate::FieldReader;
#[doc = "Field `SLOTS` writer - "]
pub type SLOTS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn slots(&self) -> SLOTS_R {
        SLOTS_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_COMPLETE_STATUS")
            .field("slots", &self.slots())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn slots(&mut self) -> SLOTS_W<TX_COMPLETE_STATUS_SPEC> {
        SLOTS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_COMPLETE_STATUS_SPEC;
impl crate::RegisterSpec for TX_COMPLETE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_complete_status::R`](R) reader structure"]
impl crate::Readable for TX_COMPLETE_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_complete_status::W`](W) writer structure"]
impl crate::Writable for TX_COMPLETE_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_COMPLETE_STATUS to value 0"]
impl crate::Resettable for TX_COMPLETE_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
