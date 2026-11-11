#[doc = "Register `SERIAL_EP_TIMEOUT0` reader"]
pub type R = crate::R<SERIAL_EP_TIMEOUT0_SPEC>;
#[doc = "Register `SERIAL_EP_TIMEOUT0` writer"]
pub type W = crate::W<SERIAL_EP_TIMEOUT0_SPEC>;
#[doc = "Field `SERIAL_TIMEOUT_EN` reader - USB serial out ep timeout enable. When a timeout event occurs, serial out ep buffer is automatically cleared and reg_serial_timeout_status is asserted."]
pub type SERIAL_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `SERIAL_TIMEOUT_EN` writer - USB serial out ep timeout enable. When a timeout event occurs, serial out ep buffer is automatically cleared and reg_serial_timeout_status is asserted."]
pub type SERIAL_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_TIMEOUT_STATUS` reader - Serial out ep triggers a timeout event."]
pub type SERIAL_TIMEOUT_STATUS_R = crate::BitReader;
#[doc = "Field `SERIAL_TIMEOUT_STATUS` writer - Serial out ep triggers a timeout event."]
pub type SERIAL_TIMEOUT_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_TIMEOUT_STATUS_CLR` writer - Write 1 to clear reg_serial_timeout_status."]
pub type SERIAL_TIMEOUT_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB serial out ep timeout enable. When a timeout event occurs, serial out ep buffer is automatically cleared and reg_serial_timeout_status is asserted."]
    #[inline(always)]
    pub fn serial_timeout_en(&self) -> SERIAL_TIMEOUT_EN_R {
        SERIAL_TIMEOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial out ep triggers a timeout event."]
    #[inline(always)]
    pub fn serial_timeout_status(&self) -> SERIAL_TIMEOUT_STATUS_R {
        SERIAL_TIMEOUT_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERIAL_EP_TIMEOUT0")
            .field("serial_timeout_en", &self.serial_timeout_en())
            .field("serial_timeout_status", &self.serial_timeout_status())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB serial out ep timeout enable. When a timeout event occurs, serial out ep buffer is automatically cleared and reg_serial_timeout_status is asserted."]
    #[inline(always)]
    pub fn serial_timeout_en(&mut self) -> SERIAL_TIMEOUT_EN_W<'_, SERIAL_EP_TIMEOUT0_SPEC> {
        SERIAL_TIMEOUT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Serial out ep triggers a timeout event."]
    #[inline(always)]
    pub fn serial_timeout_status(
        &mut self,
    ) -> SERIAL_TIMEOUT_STATUS_W<'_, SERIAL_EP_TIMEOUT0_SPEC> {
        SERIAL_TIMEOUT_STATUS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear reg_serial_timeout_status."]
    #[inline(always)]
    pub fn serial_timeout_status_clr(
        &mut self,
    ) -> SERIAL_TIMEOUT_STATUS_CLR_W<'_, SERIAL_EP_TIMEOUT0_SPEC> {
        SERIAL_TIMEOUT_STATUS_CLR_W::new(self, 2)
    }
}
#[doc = "USB uart out endpoint timeout configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`serial_ep_timeout0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serial_ep_timeout0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERIAL_EP_TIMEOUT0_SPEC;
impl crate::RegisterSpec for SERIAL_EP_TIMEOUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`serial_ep_timeout0::R`](R) reader structure"]
impl crate::Readable for SERIAL_EP_TIMEOUT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`serial_ep_timeout0::W`](W) writer structure"]
impl crate::Writable for SERIAL_EP_TIMEOUT0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SERIAL_EP_TIMEOUT0 to value 0"]
impl crate::Resettable for SERIAL_EP_TIMEOUT0_SPEC {}
