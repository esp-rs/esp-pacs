#[doc = "Register `WIFI_INT_STATUS` reader"]
pub type R = crate::R<WIFI_INT_STATUS_SPEC>;
#[doc = "Register `WIFI_INT_STATUS` writer"]
pub type W = crate::W<WIFI_INT_STATUS_SPEC>;
#[doc = "Field `TXQ_COMPLETE` reader - Indicates the completion of a transmission"]
pub type TXQ_COMPLETE_R = crate::BitReader;
#[doc = "Field `TXQ_COMPLETE` writer - Indicates the completion of a transmission"]
pub type TXQ_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXQ_COLLISION` reader - Indicates a collision, while transmitting"]
pub type TXQ_COLLISION_R = crate::BitReader;
#[doc = "Field `TXQ_COLLISION` writer - Indicates a collision, while transmitting"]
pub type TXQ_COLLISION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXQ_TIMEOUT` reader - Indicates a timeout, while transmitting"]
pub type TXQ_TIMEOUT_R = crate::BitReader;
#[doc = "Field `TXQ_TIMEOUT` writer - Indicates a timeout, while transmitting"]
pub type TXQ_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Indicates the completion of a transmission"]
    #[inline(always)]
    pub fn txq_complete(&self) -> TXQ_COMPLETE_R {
        TXQ_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates a collision, while transmitting"]
    #[inline(always)]
    pub fn txq_collision(&self) -> TXQ_COLLISION_R {
        TXQ_COLLISION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates a timeout, while transmitting"]
    #[inline(always)]
    pub fn txq_timeout(&self) -> TXQ_TIMEOUT_R {
        TXQ_TIMEOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_INT_STATUS")
            .field("txq_complete", &self.txq_complete())
            .field("txq_collision", &self.txq_collision())
            .field("txq_timeout", &self.txq_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - Indicates the completion of a transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txq_complete(&mut self) -> TXQ_COMPLETE_W<WIFI_INT_STATUS_SPEC> {
        TXQ_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates a collision, while transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn txq_collision(&mut self) -> TXQ_COLLISION_W<WIFI_INT_STATUS_SPEC> {
        TXQ_COLLISION_W::new(self, 8)
    }
    #[doc = "Bit 19 - Indicates a timeout, while transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn txq_timeout(&mut self) -> TXQ_TIMEOUT_W<WIFI_INT_STATUS_SPEC> {
        TXQ_TIMEOUT_W::new(self, 19)
    }
}
#[doc = "Interrupt status of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_INT_STATUS_SPEC;
impl crate::RegisterSpec for WIFI_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_int_status::R`](R) reader structure"]
impl crate::Readable for WIFI_INT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_int_status::W`](W) writer structure"]
impl crate::Writable for WIFI_INT_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIFI_INT_STATUS to value 0"]
impl crate::Resettable for WIFI_INT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
