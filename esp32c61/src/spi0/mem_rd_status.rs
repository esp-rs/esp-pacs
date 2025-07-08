#[doc = "Register `MEM_RD_STATUS` reader"]
pub type R = crate::R<MEM_RD_STATUS_SPEC>;
#[doc = "Register `MEM_RD_STATUS` writer"]
pub type W = crate::W<MEM_RD_STATUS_SPEC>;
#[doc = "Field `MEM_WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type MEM_WB_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type MEM_WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_WB_MODE_BITLEN` reader - Mode bits length for flash fast read mode."]
pub type MEM_WB_MODE_BITLEN_R = crate::FieldReader;
#[doc = "Field `MEM_WB_MODE_BITLEN` writer - Mode bits length for flash fast read mode."]
pub type MEM_WB_MODE_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_WB_MODE_EN` reader - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
pub type MEM_WB_MODE_EN_R = crate::BitReader;
#[doc = "Field `MEM_WB_MODE_EN` writer - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
pub type MEM_WB_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn mem_wb_mode(&self) -> MEM_WB_MODE_R {
        MEM_WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Mode bits length for flash fast read mode."]
    #[inline(always)]
    pub fn mem_wb_mode_bitlen(&self) -> MEM_WB_MODE_BITLEN_R {
        MEM_WB_MODE_BITLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_wb_mode_en(&self) -> MEM_WB_MODE_EN_R {
        MEM_WB_MODE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_RD_STATUS")
            .field("mem_wb_mode", &self.mem_wb_mode())
            .field("mem_wb_mode_bitlen", &self.mem_wb_mode_bitlen())
            .field("mem_wb_mode_en", &self.mem_wb_mode_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn mem_wb_mode(&mut self) -> MEM_WB_MODE_W<MEM_RD_STATUS_SPEC> {
        MEM_WB_MODE_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Mode bits length for flash fast read mode."]
    #[inline(always)]
    pub fn mem_wb_mode_bitlen(&mut self) -> MEM_WB_MODE_BITLEN_W<MEM_RD_STATUS_SPEC> {
        MEM_WB_MODE_BITLEN_W::new(self, 24)
    }
    #[doc = "Bit 27 - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_wb_mode_en(&mut self) -> MEM_WB_MODE_EN_W<MEM_RD_STATUS_SPEC> {
        MEM_WB_MODE_EN_W::new(self, 27)
    }
}
#[doc = "SPI0 read control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_RD_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rd_status::R`](R) reader structure"]
impl crate::Readable for MEM_RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_rd_status::W`](W) writer structure"]
impl crate::Writable for MEM_RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_RD_STATUS to value 0"]
impl crate::Resettable for MEM_RD_STATUS_SPEC {}
