#[doc = "Register `DDR` reader"]
pub type R = crate::R<DDR_SPEC>;
#[doc = "Register `DDR` writer"]
pub type W = crate::W<DDR_SPEC>;
#[doc = "Field `SPI_FMEM_DDR_EN` reader - "]
pub type SPI_FMEM_DDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_EN` writer - "]
pub type SPI_FMEM_DDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_VAR_DUMMY` reader - "]
pub type SPI_FMEM_VAR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_VAR_DUMMY` writer - "]
pub type SPI_FMEM_VAR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_DDR_RDAT_SWP` reader - "]
pub type SPI_FMEM_DDR_RDAT_SWP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_RDAT_SWP` writer - "]
pub type SPI_FMEM_DDR_RDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_DDR_WDAT_SWP` reader - "]
pub type SPI_FMEM_DDR_WDAT_SWP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_WDAT_SWP` writer - "]
pub type SPI_FMEM_DDR_WDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_DDR_CMD_DIS` reader - "]
pub type SPI_FMEM_DDR_CMD_DIS_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_CMD_DIS` writer - "]
pub type SPI_FMEM_DDR_CMD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_OUTMINBYTELEN` reader - "]
pub type SPI_FMEM_OUTMINBYTELEN_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_OUTMINBYTELEN` writer - "]
pub type SPI_FMEM_OUTMINBYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_FMEM_USR_DDR_DQS_THD` reader - "]
pub type SPI_FMEM_USR_DDR_DQS_THD_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_USR_DDR_DQS_THD` writer - "]
pub type SPI_FMEM_USR_DDR_DQS_THD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP` reader - "]
pub type SPI_FMEM_DDR_DQS_LOOP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP` writer - "]
pub type SPI_FMEM_DDR_DQS_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP_MODE` reader - "]
pub type SPI_FMEM_DDR_DQS_LOOP_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP_MODE` writer - "]
pub type SPI_FMEM_DDR_DQS_LOOP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_fmem_ddr_en(&self) -> SPI_FMEM_DDR_EN_R {
        SPI_FMEM_DDR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_fmem_var_dummy(&self) -> SPI_FMEM_VAR_DUMMY_R {
        SPI_FMEM_VAR_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_fmem_ddr_rdat_swp(&self) -> SPI_FMEM_DDR_RDAT_SWP_R {
        SPI_FMEM_DDR_RDAT_SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_fmem_ddr_wdat_swp(&self) -> SPI_FMEM_DDR_WDAT_SWP_R {
        SPI_FMEM_DDR_WDAT_SWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_fmem_ddr_cmd_dis(&self) -> SPI_FMEM_DDR_CMD_DIS_R {
        SPI_FMEM_DDR_CMD_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn spi_fmem_outminbytelen(&self) -> SPI_FMEM_OUTMINBYTELEN_R {
        SPI_FMEM_OUTMINBYTELEN_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 13:20"]
    #[inline(always)]
    pub fn spi_fmem_usr_ddr_dqs_thd(&self) -> SPI_FMEM_USR_DDR_DQS_THD_R {
        SPI_FMEM_USR_DDR_DQS_THD_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop(&self) -> SPI_FMEM_DDR_DQS_LOOP_R {
        SPI_FMEM_DDR_DQS_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop_mode(&self) -> SPI_FMEM_DDR_DQS_LOOP_MODE_R {
        SPI_FMEM_DDR_DQS_LOOP_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR")
            .field(
                "spi_fmem_ddr_dqs_loop_mode",
                &self.spi_fmem_ddr_dqs_loop_mode(),
            )
            .field("spi_fmem_ddr_dqs_loop", &self.spi_fmem_ddr_dqs_loop())
            .field("spi_fmem_usr_ddr_dqs_thd", &self.spi_fmem_usr_ddr_dqs_thd())
            .field("spi_fmem_outminbytelen", &self.spi_fmem_outminbytelen())
            .field("spi_fmem_ddr_cmd_dis", &self.spi_fmem_ddr_cmd_dis())
            .field("spi_fmem_ddr_wdat_swp", &self.spi_fmem_ddr_wdat_swp())
            .field("spi_fmem_ddr_rdat_swp", &self.spi_fmem_ddr_rdat_swp())
            .field("spi_fmem_var_dummy", &self.spi_fmem_var_dummy())
            .field("spi_fmem_ddr_en", &self.spi_fmem_ddr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_fmem_ddr_en(&mut self) -> SPI_FMEM_DDR_EN_W<DDR_SPEC> {
        SPI_FMEM_DDR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_fmem_var_dummy(&mut self) -> SPI_FMEM_VAR_DUMMY_W<DDR_SPEC> {
        SPI_FMEM_VAR_DUMMY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_fmem_ddr_rdat_swp(&mut self) -> SPI_FMEM_DDR_RDAT_SWP_W<DDR_SPEC> {
        SPI_FMEM_DDR_RDAT_SWP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_fmem_ddr_wdat_swp(&mut self) -> SPI_FMEM_DDR_WDAT_SWP_W<DDR_SPEC> {
        SPI_FMEM_DDR_WDAT_SWP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_fmem_ddr_cmd_dis(&mut self) -> SPI_FMEM_DDR_CMD_DIS_W<DDR_SPEC> {
        SPI_FMEM_DDR_CMD_DIS_W::new(self, 4)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn spi_fmem_outminbytelen(&mut self) -> SPI_FMEM_OUTMINBYTELEN_W<DDR_SPEC> {
        SPI_FMEM_OUTMINBYTELEN_W::new(self, 5)
    }
    #[doc = "Bits 13:20"]
    #[inline(always)]
    pub fn spi_fmem_usr_ddr_dqs_thd(&mut self) -> SPI_FMEM_USR_DDR_DQS_THD_W<DDR_SPEC> {
        SPI_FMEM_USR_DDR_DQS_THD_W::new(self, 13)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop(&mut self) -> SPI_FMEM_DDR_DQS_LOOP_W<DDR_SPEC> {
        SPI_FMEM_DDR_DQS_LOOP_W::new(self, 21)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop_mode(&mut self) -> SPI_FMEM_DDR_DQS_LOOP_MODE_W<DDR_SPEC> {
        SPI_FMEM_DDR_DQS_LOOP_MODE_W::new(self, 22)
    }
}
#[doc = "SPI Memory DDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDR_SPEC;
impl crate::RegisterSpec for DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr::R`](R) reader structure"]
impl crate::Readable for DDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddr::W`](W) writer structure"]
impl crate::Writable for DDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDR to value 0"]
impl crate::Resettable for DDR_SPEC {}
