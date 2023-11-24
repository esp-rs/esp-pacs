#[doc = "Register `SPI_USER2` reader"]
pub type R = crate::R<SPI_USER2_SPEC>;
#[doc = "Register `SPI_USER2` writer"]
pub type W = crate::W<SPI_USER2_SPEC>;
#[doc = "Field `reg_usr_command_value` reader - The value of \"command\" phase"]
pub type REG_USR_COMMAND_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `reg_usr_command_value` writer - The value of \"command\" phase"]
pub type REG_USR_COMMAND_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_usr_command_bitlen` reader - The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_COMMAND_BITLEN_R = crate::FieldReader;
#[doc = "Field `reg_usr_command_bitlen` writer - The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_COMMAND_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - The value of \"command\" phase"]
    #[inline(always)]
    pub fn reg_usr_command_value(&self) -> REG_USR_COMMAND_VALUE_R {
        REG_USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn reg_usr_command_bitlen(&self) -> REG_USR_COMMAND_BITLEN_R {
        REG_USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER2")
            .field(
                "reg_usr_command_bitlen",
                &format_args!("{}", self.reg_usr_command_bitlen().bits()),
            )
            .field(
                "reg_usr_command_value",
                &format_args!("{}", self.reg_usr_command_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_USER2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of \"command\" phase"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_command_value(&mut self) -> REG_USR_COMMAND_VALUE_W<SPI_USER2_SPEC> {
        REG_USR_COMMAND_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 28:31 - The length in bits of \"command\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_command_bitlen(&mut self) -> REG_USR_COMMAND_BITLEN_W<SPI_USER2_SPEC> {
        REG_USR_COMMAND_BITLEN_W::new(self, 28)
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
#[doc = "The length in bits of \"command\" phase. The register value shall be (bit_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_USER2_SPEC;
impl crate::RegisterSpec for SPI_USER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_user2::R`](R) reader structure"]
impl crate::Readable for SPI_USER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_user2::W`](W) writer structure"]
impl crate::Writable for SPI_USER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_USER2 to value 0"]
impl crate::Resettable for SPI_USER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
