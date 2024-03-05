#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `USR_ADDR_VALUE` reader - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
pub type USR_ADDR_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `USR_ADDR_VALUE` writer - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
pub type USR_ADDR_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    #[inline(always)]
    pub fn usr_addr_value(&self) -> USR_ADDR_VALUE_R {
        USR_ADDR_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR")
            .field(
                "usr_addr_value",
                &format_args!("{}", self.usr_addr_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    #[inline(always)]
    #[must_use]
    pub fn usr_addr_value(&mut self) -> USR_ADDR_VALUE_W<ADDR_SPEC> {
        USR_ADDR_VALUE_W::new(self, 0)
    }
}
#[doc = "SPI1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
