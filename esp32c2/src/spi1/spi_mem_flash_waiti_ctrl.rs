#[doc = "Register `SPI_MEM_FLASH_WAITI_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_FLASH_WAITI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_FLASH_WAITI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_FLASH_WAITI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_FLASH_WAITI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_FLASH_WAITI_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_FLASH_WAITI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_FLASH_WAITI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_FLASH_WAITI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_FLASH_WAITI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_WAITI_DUMMY` reader - The dummy phase enable when wait flash idle (RDSR)"]
pub type SPI_MEM_WAITI_DUMMY_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_WAITI_DUMMY` writer - The dummy phase enable when wait flash idle (RDSR)"]
pub type SPI_MEM_WAITI_DUMMY_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_FLASH_WAITI_CTRL_SPEC, bool, 1>;
#[doc = "Field `SPI_MEM_WAITI_CMD` reader - The command to wait flash idle(RDSR)."]
pub type SPI_MEM_WAITI_CMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_WAITI_CMD` writer - The command to wait flash idle(RDSR)."]
pub type SPI_MEM_WAITI_CMD_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_FLASH_WAITI_CTRL_SPEC, u8, u8, 8, 2>;
#[doc = "Field `SPI_MEM_WAITI_DUMMY_CYCLELEN` reader - The dummy cycle length when wait flash idle(RDSR)."]
pub type SPI_MEM_WAITI_DUMMY_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_WAITI_DUMMY_CYCLELEN` writer - The dummy cycle length when wait flash idle(RDSR)."]
pub type SPI_MEM_WAITI_DUMMY_CYCLELEN_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_FLASH_WAITI_CTRL_SPEC, u8, u8, 6, 10>;
impl R {
    #[doc = "Bit 1 - The dummy phase enable when wait flash idle (RDSR)"]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy(&self) -> SPI_MEM_WAITI_DUMMY_R {
        SPI_MEM_WAITI_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - The command to wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn spi_mem_waiti_cmd(&self) -> SPI_MEM_WAITI_CMD_R {
        SPI_MEM_WAITI_CMD_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:15 - The dummy cycle length when wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy_cyclelen(&self) -> SPI_MEM_WAITI_DUMMY_CYCLELEN_R {
        SPI_MEM_WAITI_DUMMY_CYCLELEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - The dummy phase enable when wait flash idle (RDSR)"]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy(&mut self) -> SPI_MEM_WAITI_DUMMY_W {
        SPI_MEM_WAITI_DUMMY_W::new(self)
    }
    #[doc = "Bits 2:9 - The command to wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn spi_mem_waiti_cmd(&mut self) -> SPI_MEM_WAITI_CMD_W {
        SPI_MEM_WAITI_CMD_W::new(self)
    }
    #[doc = "Bits 10:15 - The dummy cycle length when wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn spi_mem_waiti_dummy_cyclelen(&mut self) -> SPI_MEM_WAITI_DUMMY_CYCLELEN_W {
        SPI_MEM_WAITI_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 wait idle control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_flash_waiti_ctrl](index.html) module"]
pub struct SPI_MEM_FLASH_WAITI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_FLASH_WAITI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_flash_waiti_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_FLASH_WAITI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_flash_waiti_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_FLASH_WAITI_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_FLASH_WAITI_CTRL to value 0x14"]
impl crate::Resettable for SPI_MEM_FLASH_WAITI_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
