#[doc = "Register `SPI_SMEM_S_AC` reader"]
pub type R = crate::R<SPI_SMEM_S_AC_SPEC>;
#[doc = "Register `SPI_SMEM_S_AC` writer"]
pub type W = crate::W<SPI_SMEM_S_AC_SPEC>;
#[doc = "Field `SPI_SMEM_S_CS_SETUP` reader - "]
pub type SPI_SMEM_S_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_CS_SETUP` writer - "]
pub type SPI_SMEM_S_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_CS_HOLD` reader - "]
pub type SPI_SMEM_S_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_CS_HOLD` writer - "]
pub type SPI_SMEM_S_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_CS_SETUP_TIME` reader - "]
pub type SPI_SMEM_S_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_CS_SETUP_TIME` writer - "]
pub type SPI_SMEM_S_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPI_SMEM_S_CS_HOLD_TIME` reader - "]
pub type SPI_SMEM_S_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_CS_HOLD_TIME` writer - "]
pub type SPI_SMEM_S_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPI_SMEM_S_ECC_CS_HOLD_TIME` reader - "]
pub type SPI_SMEM_S_ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_ECC_CS_HOLD_TIME` writer - "]
pub type SPI_SMEM_S_ECC_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_S_ECC_SKIP_PAGE_CORNER` reader - "]
pub type SPI_SMEM_S_ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_ECC_SKIP_PAGE_CORNER` writer - "]
pub type SPI_SMEM_S_ECC_SKIP_PAGE_CORNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_ECC_16TO18_BYTE_EN` reader - "]
pub type SPI_SMEM_S_ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_ECC_16TO18_BYTE_EN` writer - "]
pub type SPI_SMEM_S_ECC_16TO18_BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_CS_HOLD_DELAY` reader - "]
pub type SPI_SMEM_S_CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_CS_HOLD_DELAY` writer - "]
pub type SPI_SMEM_S_CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_SMEM_S_SPLIT_TRANS_EN` reader - "]
pub type SPI_SMEM_S_SPLIT_TRANS_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_SPLIT_TRANS_EN` writer - "]
pub type SPI_SMEM_S_SPLIT_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_s_cs_setup(&self) -> SPI_SMEM_S_CS_SETUP_R {
        SPI_SMEM_S_CS_SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_s_cs_hold(&self) -> SPI_SMEM_S_CS_HOLD_R {
        SPI_SMEM_S_CS_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn spi_smem_s_cs_setup_time(&self) -> SPI_SMEM_S_CS_SETUP_TIME_R {
        SPI_SMEM_S_CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn spi_smem_s_cs_hold_time(&self) -> SPI_SMEM_S_CS_HOLD_TIME_R {
        SPI_SMEM_S_CS_HOLD_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_cs_hold_time(&self) -> SPI_SMEM_S_ECC_CS_HOLD_TIME_R {
        SPI_SMEM_S_ECC_CS_HOLD_TIME_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_skip_page_corner(&self) -> SPI_SMEM_S_ECC_SKIP_PAGE_CORNER_R {
        SPI_SMEM_S_ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_16to18_byte_en(&self) -> SPI_SMEM_S_ECC_16TO18_BYTE_EN_R {
        SPI_SMEM_S_ECC_16TO18_BYTE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 25:30"]
    #[inline(always)]
    pub fn spi_smem_s_cs_hold_delay(&self) -> SPI_SMEM_S_CS_HOLD_DELAY_R {
        SPI_SMEM_S_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_smem_s_split_trans_en(&self) -> SPI_SMEM_S_SPLIT_TRANS_EN_R {
        SPI_SMEM_S_SPLIT_TRANS_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_S_AC")
            .field("spi_smem_s_cs_setup", &self.spi_smem_s_cs_setup())
            .field("spi_smem_s_cs_hold", &self.spi_smem_s_cs_hold())
            .field("spi_smem_s_cs_setup_time", &self.spi_smem_s_cs_setup_time())
            .field("spi_smem_s_cs_hold_time", &self.spi_smem_s_cs_hold_time())
            .field(
                "spi_smem_s_ecc_cs_hold_time",
                &self.spi_smem_s_ecc_cs_hold_time(),
            )
            .field(
                "spi_smem_s_ecc_skip_page_corner",
                &self.spi_smem_s_ecc_skip_page_corner(),
            )
            .field(
                "spi_smem_s_ecc_16to18_byte_en",
                &self.spi_smem_s_ecc_16to18_byte_en(),
            )
            .field("spi_smem_s_cs_hold_delay", &self.spi_smem_s_cs_hold_delay())
            .field(
                "spi_smem_s_split_trans_en",
                &self.spi_smem_s_split_trans_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_smem_s_cs_setup(&mut self) -> SPI_SMEM_S_CS_SETUP_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_CS_SETUP_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_smem_s_cs_hold(&mut self) -> SPI_SMEM_S_CS_HOLD_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_CS_HOLD_W::new(self, 1)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn spi_smem_s_cs_setup_time(
        &mut self,
    ) -> SPI_SMEM_S_CS_SETUP_TIME_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_CS_SETUP_TIME_W::new(self, 2)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn spi_smem_s_cs_hold_time(&mut self) -> SPI_SMEM_S_CS_HOLD_TIME_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_CS_HOLD_TIME_W::new(self, 7)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_cs_hold_time(
        &mut self,
    ) -> SPI_SMEM_S_ECC_CS_HOLD_TIME_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_ECC_CS_HOLD_TIME_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_skip_page_corner(
        &mut self,
    ) -> SPI_SMEM_S_ECC_SKIP_PAGE_CORNER_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_ECC_SKIP_PAGE_CORNER_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_16to18_byte_en(
        &mut self,
    ) -> SPI_SMEM_S_ECC_16TO18_BYTE_EN_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_ECC_16TO18_BYTE_EN_W::new(self, 16)
    }
    #[doc = "Bits 25:30"]
    #[inline(always)]
    pub fn spi_smem_s_cs_hold_delay(
        &mut self,
    ) -> SPI_SMEM_S_CS_HOLD_DELAY_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_CS_HOLD_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_smem_s_split_trans_en(
        &mut self,
    ) -> SPI_SMEM_S_SPLIT_TRANS_EN_W<'_, SPI_SMEM_S_AC_SPEC> {
        SPI_SMEM_S_SPLIT_TRANS_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_S_AC_SPEC;
impl crate::RegisterSpec for SPI_SMEM_S_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_s_ac::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_S_AC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_s_ac::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_S_AC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_S_AC to value 0xb084"]
impl crate::Resettable for SPI_SMEM_S_AC_SPEC {
    const RESET_VALUE: u32 = 0xb084;
}
