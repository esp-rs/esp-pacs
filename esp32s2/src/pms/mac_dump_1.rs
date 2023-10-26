#[doc = "Register `MAC_DUMP_1` reader"]
pub type R = crate::R<MAC_DUMP_1_SPEC>;
#[doc = "Register `MAC_DUMP_1` writer"]
pub type W = crate::W<MAC_DUMP_1_SPEC>;
#[doc = "Field `MAC_DUMP_CONNECT` reader - Configure MAC dump connection."]
pub type MAC_DUMP_CONNECT_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_DUMP_CONNECT` writer - Configure MAC dump connection."]
pub type MAC_DUMP_CONNECT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
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
            .field(
                "mac_dump_connect",
                &format_args!("{}", self.mac_dump_connect().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC_DUMP_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configure MAC dump connection."]
    #[inline(always)]
    #[must_use]
    pub fn mac_dump_connect(&mut self) -> MAC_DUMP_CONNECT_W<MAC_DUMP_1_SPEC, 0> {
        MAC_DUMP_CONNECT_W::new(self)
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
#[doc = "MAC dump permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_dump_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_dump_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_DUMP_1_SPEC;
impl crate::RegisterSpec for MAC_DUMP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_dump_1::R`](R) reader structure"]
impl crate::Readable for MAC_DUMP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_dump_1::W`](W) writer structure"]
impl crate::Writable for MAC_DUMP_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_DUMP_1 to value 0xe4"]
impl crate::Resettable for MAC_DUMP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xe4;
}
