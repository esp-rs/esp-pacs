#[doc = "Register `MAC_DUMP_1` reader"]
pub type R = crate::R<MAC_DUMP_1_SPEC>;
#[doc = "Register `MAC_DUMP_1` writer"]
pub type W = crate::W<MAC_DUMP_1_SPEC>;
#[doc = "Field `MAC_DUMP_CONNECT` reader - Configure MAC dump connection."]
pub type MAC_DUMP_CONNECT_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_DUMP_CONNECT` writer - Configure MAC dump connection."]
pub type MAC_DUMP_CONNECT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configure MAC dump connection."]
    #[inline(always)]
    pub fn mac_dump_connect(&self) -> MAC_DUMP_CONNECT_R {
        MAC_DUMP_CONNECT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_DUMP_1")
            .field("mac_dump_connect", &self.mac_dump_connect())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configure MAC dump connection."]
    #[inline(always)]
    pub fn mac_dump_connect(&mut self) -> MAC_DUMP_CONNECT_W<MAC_DUMP_1_SPEC> {
        MAC_DUMP_CONNECT_W::new(self, 0)
    }
}
#[doc = "MAC dump permission control register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_dump_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_dump_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_DUMP_1_SPEC;
impl crate::RegisterSpec for MAC_DUMP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_dump_1::R`](R) reader structure"]
impl crate::Readable for MAC_DUMP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_dump_1::W`](W) writer structure"]
impl crate::Writable for MAC_DUMP_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_DUMP_1 to value 0xe4"]
impl crate::Resettable for MAC_DUMP_1_SPEC {
    const RESET_VALUE: u32 = 0xe4;
}
