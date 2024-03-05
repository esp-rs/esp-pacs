#[doc = "Register `SPI_MEM_ADDR` reader"]
pub type R = crate::R<SPI_MEM_ADDR_SPEC>;
#[doc = "Register `SPI_MEM_ADDR` writer"]
pub type W = crate::W<SPI_MEM_ADDR_SPEC>;
#[doc = "Field `SPI_MEM_USR_ADDR_VALUE` reader - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
pub type SPI_MEM_USR_ADDR_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_USR_ADDR_VALUE` writer - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
pub type SPI_MEM_USR_ADDR_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_value(&self) -> SPI_MEM_USR_ADDR_VALUE_R {
        SPI_MEM_USR_ADDR_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_ADDR")
            .field(
                "spi_mem_usr_addr_value",
                &format_args!("{}", self.spi_mem_usr_addr_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_addr_value(&mut self) -> SPI_MEM_USR_ADDR_VALUE_W<SPI_MEM_ADDR_SPEC> {
        SPI_MEM_USR_ADDR_VALUE_W::new(self, 0)
    }
}
#[doc = "SPI1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_addr::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_addr::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
