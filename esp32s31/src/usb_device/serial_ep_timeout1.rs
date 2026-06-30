#[doc = "Register `SERIAL_EP_TIMEOUT1` reader"]
pub type R = crate::R<SERIAL_EP_TIMEOUT1_SPEC>;
#[doc = "Register `SERIAL_EP_TIMEOUT1` writer"]
pub type W = crate::W<SERIAL_EP_TIMEOUT1_SPEC>;
#[doc = "Field `SERIAL_TIMEOUT_MAX` reader - USB serial out ep timeout max threshold value, indicates the maximum time that waiting for ESP to take away data in memory. This value is in steps of 20.83ns."]
pub type SERIAL_TIMEOUT_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `SERIAL_TIMEOUT_MAX` writer - USB serial out ep timeout max threshold value, indicates the maximum time that waiting for ESP to take away data in memory. This value is in steps of 20.83ns."]
pub type SERIAL_TIMEOUT_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB serial out ep timeout max threshold value, indicates the maximum time that waiting for ESP to take away data in memory. This value is in steps of 20.83ns."]
    #[inline(always)]
    pub fn serial_timeout_max(&self) -> SERIAL_TIMEOUT_MAX_R {
        SERIAL_TIMEOUT_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERIAL_EP_TIMEOUT1")
            .field("serial_timeout_max", &self.serial_timeout_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - USB serial out ep timeout max threshold value, indicates the maximum time that waiting for ESP to take away data in memory. This value is in steps of 20.83ns."]
    #[inline(always)]
    pub fn serial_timeout_max(&mut self) -> SERIAL_TIMEOUT_MAX_W<'_, SERIAL_EP_TIMEOUT1_SPEC> {
        SERIAL_TIMEOUT_MAX_W::new(self, 0)
    }
}
#[doc = "USB uart out endpoint timeout configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`serial_ep_timeout1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serial_ep_timeout1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SERIAL_EP_TIMEOUT1_SPEC;
impl crate::RegisterSpec for SERIAL_EP_TIMEOUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`serial_ep_timeout1::R`](R) reader structure"]
impl crate::Readable for SERIAL_EP_TIMEOUT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`serial_ep_timeout1::W`](W) writer structure"]
impl crate::Writable for SERIAL_EP_TIMEOUT1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SERIAL_EP_TIMEOUT1 to value 0x0049_4100"]
impl crate::Resettable for SERIAL_EP_TIMEOUT1_SPEC {
    const RESET_VALUE: u32 = 0x0049_4100;
}
