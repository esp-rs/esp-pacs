#[doc = "Register `SPI_SMEM_DOUT_NUM` reader"]
pub type R = crate::R<SPI_SMEM_DOUT_NUM_SPEC>;
#[doc = "Register `SPI_SMEM_DOUT_NUM` writer"]
pub type W = crate::W<SPI_SMEM_DOUT_NUM_SPEC>;
#[doc = "Field `SPI_SMEM_DOUT0_NUM` reader - "]
pub type SPI_SMEM_DOUT0_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT0_NUM` writer - "]
pub type SPI_SMEM_DOUT0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT1_NUM` reader - "]
pub type SPI_SMEM_DOUT1_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT1_NUM` writer - "]
pub type SPI_SMEM_DOUT1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT2_NUM` reader - "]
pub type SPI_SMEM_DOUT2_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT2_NUM` writer - "]
pub type SPI_SMEM_DOUT2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT3_NUM` reader - "]
pub type SPI_SMEM_DOUT3_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT3_NUM` writer - "]
pub type SPI_SMEM_DOUT3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT4_NUM` reader - "]
pub type SPI_SMEM_DOUT4_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT4_NUM` writer - "]
pub type SPI_SMEM_DOUT4_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT5_NUM` reader - "]
pub type SPI_SMEM_DOUT5_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT5_NUM` writer - "]
pub type SPI_SMEM_DOUT5_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT6_NUM` reader - "]
pub type SPI_SMEM_DOUT6_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT6_NUM` writer - "]
pub type SPI_SMEM_DOUT6_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUT7_NUM` reader - "]
pub type SPI_SMEM_DOUT7_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUT7_NUM` writer - "]
pub type SPI_SMEM_DOUT7_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DOUTS_NUM` reader - "]
pub type SPI_SMEM_DOUTS_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DOUTS_NUM` writer - "]
pub type SPI_SMEM_DOUTS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_smem_dout0_num(&self) -> SPI_SMEM_DOUT0_NUM_R {
        SPI_SMEM_DOUT0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_smem_dout1_num(&self) -> SPI_SMEM_DOUT1_NUM_R {
        SPI_SMEM_DOUT1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi_smem_dout2_num(&self) -> SPI_SMEM_DOUT2_NUM_R {
        SPI_SMEM_DOUT2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn spi_smem_dout3_num(&self) -> SPI_SMEM_DOUT3_NUM_R {
        SPI_SMEM_DOUT3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn spi_smem_dout4_num(&self) -> SPI_SMEM_DOUT4_NUM_R {
        SPI_SMEM_DOUT4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn spi_smem_dout5_num(&self) -> SPI_SMEM_DOUT5_NUM_R {
        SPI_SMEM_DOUT5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn spi_smem_dout6_num(&self) -> SPI_SMEM_DOUT6_NUM_R {
        SPI_SMEM_DOUT6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn spi_smem_dout7_num(&self) -> SPI_SMEM_DOUT7_NUM_R {
        SPI_SMEM_DOUT7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn spi_smem_douts_num(&self) -> SPI_SMEM_DOUTS_NUM_R {
        SPI_SMEM_DOUTS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DOUT_NUM")
            .field("spi_smem_douts_num", &self.spi_smem_douts_num())
            .field("spi_smem_dout7_num", &self.spi_smem_dout7_num())
            .field("spi_smem_dout6_num", &self.spi_smem_dout6_num())
            .field("spi_smem_dout5_num", &self.spi_smem_dout5_num())
            .field("spi_smem_dout4_num", &self.spi_smem_dout4_num())
            .field("spi_smem_dout3_num", &self.spi_smem_dout3_num())
            .field("spi_smem_dout2_num", &self.spi_smem_dout2_num())
            .field("spi_smem_dout1_num", &self.spi_smem_dout1_num())
            .field("spi_smem_dout0_num", &self.spi_smem_dout0_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi_smem_dout0_num(&mut self) -> SPI_SMEM_DOUT0_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn spi_smem_dout1_num(&mut self) -> SPI_SMEM_DOUT1_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spi_smem_dout2_num(&mut self) -> SPI_SMEM_DOUT2_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn spi_smem_dout3_num(&mut self) -> SPI_SMEM_DOUT3_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT3_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn spi_smem_dout4_num(&mut self) -> SPI_SMEM_DOUT4_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT4_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn spi_smem_dout5_num(&mut self) -> SPI_SMEM_DOUT5_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT5_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn spi_smem_dout6_num(&mut self) -> SPI_SMEM_DOUT6_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT6_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn spi_smem_dout7_num(&mut self) -> SPI_SMEM_DOUT7_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUT7_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn spi_smem_douts_num(&mut self) -> SPI_SMEM_DOUTS_NUM_W<'_, SPI_SMEM_DOUT_NUM_SPEC> {
        SPI_SMEM_DOUTS_NUM_W::new(self, 16)
    }
}
#[doc = "SPI Memory SRAM Data Out Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_dout_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_dout_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_DOUT_NUM_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DOUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_dout_num::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_DOUT_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_dout_num::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_DOUT_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_DOUT_NUM to value 0"]
impl crate::Resettable for SPI_SMEM_DOUT_NUM_SPEC {}
