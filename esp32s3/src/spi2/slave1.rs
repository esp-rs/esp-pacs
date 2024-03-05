#[doc = "Register `SLAVE1` reader"]
pub type R = crate::R<SLAVE1_SPEC>;
#[doc = "Register `SLAVE1` writer"]
pub type W = crate::W<SLAVE1_SPEC>;
#[doc = "Field `SLV_DATA_BITLEN` reader - The transferred data bit length in SPI slave FD and HD mode."]
pub type SLV_DATA_BITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_DATA_BITLEN` writer - The transferred data bit length in SPI slave FD and HD mode."]
pub type SLV_DATA_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub type SLV_LAST_COMMAND_R = crate::FieldReader;
#[doc = "Field `SLV_LAST_COMMAND` writer - In the slave mode it is the value of command."]
pub type SLV_LAST_COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLV_LAST_ADDR` reader - In the slave mode it is the value of address."]
pub type SLV_LAST_ADDR_R = crate::FieldReader;
#[doc = "Field `SLV_LAST_ADDR` writer - In the slave mode it is the value of address."]
pub type SLV_LAST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:17 - The transferred data bit length in SPI slave FD and HD mode."]
    #[inline(always)]
    pub fn slv_data_bitlen(&self) -> SLV_DATA_BITLEN_R {
        SLV_DATA_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:25 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn slv_last_addr(&self) -> SLV_LAST_ADDR_R {
        SLV_LAST_ADDR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE1")
            .field(
                "slv_data_bitlen",
                &format_args!("{}", self.slv_data_bitlen().bits()),
            )
            .field(
                "slv_last_command",
                &format_args!("{}", self.slv_last_command().bits()),
            )
            .field(
                "slv_last_addr",
                &format_args!("{}", self.slv_last_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:17 - The transferred data bit length in SPI slave FD and HD mode."]
    #[inline(always)]
    #[must_use]
    pub fn slv_data_bitlen(&mut self) -> SLV_DATA_BITLEN_W<SLAVE1_SPEC> {
        SLV_DATA_BITLEN_W::new(self, 0)
    }
    #[doc = "Bits 18:25 - In the slave mode it is the value of command."]
    #[inline(always)]
    #[must_use]
    pub fn slv_last_command(&mut self) -> SLV_LAST_COMMAND_W<SLAVE1_SPEC> {
        SLV_LAST_COMMAND_W::new(self, 18)
    }
    #[doc = "Bits 26:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    #[must_use]
    pub fn slv_last_addr(&mut self) -> SLV_LAST_ADDR_W<SLAVE1_SPEC> {
        SLV_LAST_ADDR_W::new(self, 26)
    }
}
#[doc = "SPI slave control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave1::R`](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave1::W`](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE1 to value 0"]
impl crate::Resettable for SLAVE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
