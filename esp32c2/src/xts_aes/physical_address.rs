#[doc = "Register `PHYSICAL_ADDRESS` reader"]
pub type R = crate::R<PHYSICAL_ADDRESS_SPEC>;
#[doc = "Register `PHYSICAL_ADDRESS` writer"]
pub type W = crate::W<PHYSICAL_ADDRESS_SPEC>;
#[doc = "Field `PHYSICAL_ADDRESS` reader - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
pub type PHYSICAL_ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `PHYSICAL_ADDRESS` writer - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
pub type PHYSICAL_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
    #[inline(always)]
    pub fn physical_address(&self) -> PHYSICAL_ADDRESS_R {
        PHYSICAL_ADDRESS_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHYSICAL_ADDRESS")
            .field(
                "physical_address",
                &format_args!("{}", self.physical_address().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHYSICAL_ADDRESS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn physical_address(&mut self) -> PHYSICAL_ADDRESS_W<PHYSICAL_ADDRESS_SPEC> {
        PHYSICAL_ADDRESS_W::new(self, 0)
    }
}
#[doc = "XTS-AES physical address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHYSICAL_ADDRESS_SPEC;
impl crate::RegisterSpec for PHYSICAL_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`physical_address::R`](R) reader structure"]
impl crate::Readable for PHYSICAL_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`physical_address::W`](W) writer structure"]
impl crate::Writable for PHYSICAL_ADDRESS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHYSICAL_ADDRESS to value 0"]
impl crate::Resettable for PHYSICAL_ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
