#[doc = "Register `RD_STATUS` reader"]
pub type R = crate::R<RD_STATUS_SPEC>;
#[doc = "Register `RD_STATUS` writer"]
pub type W = crate::W<RD_STATUS_SPEC>;
#[doc = "Field `STATUS` reader - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
pub type STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `STATUS` writer - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
pub type STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type WB_MODE_R = crate::FieldReader;
#[doc = "Field `WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_STATUS")
            .field("status", &format_args!("{}", self.status().bits()))
            .field("wb_mode", &format_args!("{}", self.wb_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<RD_STATUS_SPEC> {
        STATUS_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    #[must_use]
    pub fn wb_mode(&mut self) -> WB_MODE_W<RD_STATUS_SPEC> {
        WB_MODE_W::new(self, 16)
    }
}
#[doc = "SPI1 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_status::R`](R) reader structure"]
impl crate::Readable for RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_status::W`](W) writer structure"]
impl crate::Writable for RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD_STATUS to value 0"]
impl crate::Resettable for RD_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
