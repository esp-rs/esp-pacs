#[doc = "Register `MEM_USER2` reader"]
pub type R = crate::R<MEM_USER2_SPEC>;
#[doc = "Register `MEM_USER2` writer"]
pub type W = crate::W<MEM_USER2_SPEC>;
#[doc = "Field `MEM_USR_COMMAND_VALUE` reader - The value of command."]
pub type MEM_USR_COMMAND_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `MEM_USR_COMMAND_VALUE` writer - The value of command."]
pub type MEM_USR_COMMAND_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEM_USR_COMMAND_BITLEN` reader - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub type MEM_USR_COMMAND_BITLEN_R = crate::FieldReader;
#[doc = "Field `MEM_USR_COMMAND_BITLEN` writer - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub type MEM_USR_COMMAND_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn mem_usr_command_value(&self) -> MEM_USR_COMMAND_VALUE_R {
        MEM_USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn mem_usr_command_bitlen(&self) -> MEM_USR_COMMAND_BITLEN_R {
        MEM_USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_USER2")
            .field("mem_usr_command_value", &self.mem_usr_command_value())
            .field("mem_usr_command_bitlen", &self.mem_usr_command_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn mem_usr_command_value(&mut self) -> MEM_USR_COMMAND_VALUE_W<MEM_USER2_SPEC> {
        MEM_USR_COMMAND_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn mem_usr_command_bitlen(&mut self) -> MEM_USR_COMMAND_BITLEN_W<MEM_USER2_SPEC> {
        MEM_USR_COMMAND_BITLEN_W::new(self, 28)
    }
}
#[doc = "SPI0 user2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_user2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_user2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_USER2_SPEC;
impl crate::RegisterSpec for MEM_USER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_user2::R`](R) reader structure"]
impl crate::Readable for MEM_USER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_user2::W`](W) writer structure"]
impl crate::Writable for MEM_USER2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_USER2 to value 0x7000_0000"]
impl crate::Resettable for MEM_USER2_SPEC {
    const RESET_VALUE: u32 = 0x7000_0000;
}
