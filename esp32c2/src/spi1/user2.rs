#[doc = "Register `USER2` reader"]
pub type R = crate::R<USER2_SPEC>;
#[doc = "Register `USER2` writer"]
pub type W = crate::W<USER2_SPEC>;
#[doc = "Field `USR_COMMAND_VALUE` reader - The value of command."]
pub type USR_COMMAND_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `USR_COMMAND_VALUE` writer - The value of command."]
pub type USR_COMMAND_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `USR_COMMAND_BITLEN` reader - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub type USR_COMMAND_BITLEN_R = crate::FieldReader;
#[doc = "Field `USR_COMMAND_BITLEN` writer - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub type USR_COMMAND_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn usr_command_value(&self) -> USR_COMMAND_VALUE_R {
        USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn usr_command_bitlen(&self) -> USR_COMMAND_BITLEN_R {
        USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER2")
            .field(
                "usr_command_value",
                &format_args!("{}", self.usr_command_value().bits()),
            )
            .field(
                "usr_command_bitlen",
                &format_args!("{}", self.usr_command_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USER2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    #[must_use]
    pub fn usr_command_value(&mut self) -> USR_COMMAND_VALUE_W<USER2_SPEC> {
        USR_COMMAND_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn usr_command_bitlen(&mut self) -> USR_COMMAND_BITLEN_W<USER2_SPEC> {
        USR_COMMAND_BITLEN_W::new(self, 28)
    }
}
#[doc = "SPI1 user2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER2_SPEC;
impl crate::RegisterSpec for USER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user2::R`](R) reader structure"]
impl crate::Readable for USER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user2::W`](W) writer structure"]
impl crate::Writable for USER2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER2 to value 0x7000_0000"]
impl crate::Resettable for USER2_SPEC {
    const RESET_VALUE: u32 = 0x7000_0000;
}
