#[doc = "Register `POWER_VDD_SPI_CNTL` reader"]
pub type R = crate::R<POWER_VDD_SPI_CNTL_SPEC>;
#[doc = "Register `POWER_VDD_SPI_CNTL` writer"]
pub type W = crate::W<POWER_VDD_SPI_CNTL_SPEC>;
#[doc = "Field `VDD_SPI_PWR_WAIT` reader - need_des"]
pub type VDD_SPI_PWR_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `VDD_SPI_PWR_WAIT` writer - need_des"]
pub type VDD_SPI_PWR_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `VDD_SPI_PWR_SW` reader - need_des"]
pub type VDD_SPI_PWR_SW_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_PWR_SW` writer - need_des"]
pub type VDD_SPI_PWR_SW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VDD_SPI_PWR_SEL_SW` reader - need_des"]
pub type VDD_SPI_PWR_SEL_SW_R = crate::BitReader;
#[doc = "Field `VDD_SPI_PWR_SEL_SW` writer - need_des"]
pub type VDD_SPI_PWR_SEL_SW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 18:28 - need_des"]
    #[inline(always)]
    pub fn vdd_spi_pwr_wait(&self) -> VDD_SPI_PWR_WAIT_R {
        VDD_SPI_PWR_WAIT_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 29:30 - need_des"]
    #[inline(always)]
    pub fn vdd_spi_pwr_sw(&self) -> VDD_SPI_PWR_SW_R {
        VDD_SPI_PWR_SW_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn vdd_spi_pwr_sel_sw(&self) -> VDD_SPI_PWR_SEL_SW_R {
        VDD_SPI_PWR_SEL_SW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_VDD_SPI_CNTL")
            .field(
                "vdd_spi_pwr_wait",
                &format_args!("{}", self.vdd_spi_pwr_wait().bits()),
            )
            .field(
                "vdd_spi_pwr_sw",
                &format_args!("{}", self.vdd_spi_pwr_sw().bits()),
            )
            .field(
                "vdd_spi_pwr_sel_sw",
                &format_args!("{}", self.vdd_spi_pwr_sel_sw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_VDD_SPI_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 18:28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_wait(&mut self) -> VDD_SPI_PWR_WAIT_W<POWER_VDD_SPI_CNTL_SPEC, 18> {
        VDD_SPI_PWR_WAIT_W::new(self)
    }
    #[doc = "Bits 29:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_sw(&mut self) -> VDD_SPI_PWR_SW_W<POWER_VDD_SPI_CNTL_SPEC, 29> {
        VDD_SPI_PWR_SW_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_sel_sw(&mut self) -> VDD_SPI_PWR_SEL_SW_W<POWER_VDD_SPI_CNTL_SPEC, 31> {
        VDD_SPI_PWR_SEL_SW_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_vdd_spi_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_vdd_spi_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_VDD_SPI_CNTL_SPEC;
impl crate::RegisterSpec for POWER_VDD_SPI_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_vdd_spi_cntl::R`](R) reader structure"]
impl crate::Readable for POWER_VDD_SPI_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_vdd_spi_cntl::W`](W) writer structure"]
impl crate::Writable for POWER_VDD_SPI_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_VDD_SPI_CNTL to value 0x63fc_0000"]
impl crate::Resettable for POWER_VDD_SPI_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x63fc_0000;
}
