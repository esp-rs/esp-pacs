#[doc = "Register `DIG_PWC` reader"]
pub struct R(crate::R<DIG_PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_PWC` writer"]
pub struct W(crate::W<DIG_PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_PWC_SPEC>;
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
impl From<crate::W<DIG_PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDD_SPI_PWR_DRV` reader - Need add desc"]
pub type VDD_SPI_PWR_DRV_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_PWR_DRV` writer - Need add desc"]
pub type VDD_SPI_PWR_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, DIG_PWC_SPEC, 2, O>;
#[doc = "Field `VDD_SPI_PWR_FORCE` reader - Need add desc"]
pub type VDD_SPI_PWR_FORCE_R = crate::BitReader;
#[doc = "Field `VDD_SPI_PWR_FORCE` writer - Need add desc"]
pub type VDD_SPI_PWR_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `VDD_SPI_PD_EN` reader - Need add desc"]
pub type VDD_SPI_PD_EN_R = crate::BitReader;
#[doc = "Field `VDD_SPI_PD_EN` writer - Need add desc"]
pub type VDD_SPI_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `LSLP_MEM_FORCE_PD` reader - memories in digital core force PD in sleep"]
pub type LSLP_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PD` writer - memories in digital core force PD in sleep"]
pub type LSLP_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `LSLP_MEM_FORCE_PU` reader - memories in digital core force no PD in sleep"]
pub type LSLP_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PU` writer - memories in digital core force no PD in sleep"]
pub type LSLP_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_PD` reader - digital core force power down"]
pub type DG_WRAP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PD` writer - digital core force power down"]
pub type DG_WRAP_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_PU` reader - digital core force power up"]
pub type DG_WRAP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PU` writer - digital core force power up"]
pub type DG_WRAP_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_WRAP_PD_EN` reader - Need add desc"]
pub type DG_WRAP_PD_EN_R = crate::BitReader;
#[doc = "Field `DG_WRAP_PD_EN` writer - Need add desc"]
pub type DG_WRAP_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - Need add desc"]
    #[inline(always)]
    pub fn vdd_spi_pwr_drv(&self) -> VDD_SPI_PWR_DRV_R {
        VDD_SPI_PWR_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn vdd_spi_pwr_force(&self) -> VDD_SPI_PWR_FORCE_R {
        VDD_SPI_PWR_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn vdd_spi_pd_en(&self) -> VDD_SPI_PD_EN_R {
        VDD_SPI_PD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - memories in digital core force PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PWC")
            .field(
                "vdd_spi_pwr_drv",
                &format_args!("{}", self.vdd_spi_pwr_drv().bits()),
            )
            .field(
                "vdd_spi_pwr_force",
                &format_args!("{}", self.vdd_spi_pwr_force().bit()),
            )
            .field(
                "vdd_spi_pd_en",
                &format_args!("{}", self.vdd_spi_pd_en().bit()),
            )
            .field(
                "lslp_mem_force_pd",
                &format_args!("{}", self.lslp_mem_force_pd().bit()),
            )
            .field(
                "lslp_mem_force_pu",
                &format_args!("{}", self.lslp_mem_force_pu().bit()),
            )
            .field(
                "dg_wrap_force_pd",
                &format_args!("{}", self.dg_wrap_force_pd().bit()),
            )
            .field(
                "dg_wrap_force_pu",
                &format_args!("{}", self.dg_wrap_force_pu().bit()),
            )
            .field(
                "dg_wrap_pd_en",
                &format_args!("{}", self.dg_wrap_pd_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_PWC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_drv(&mut self) -> VDD_SPI_PWR_DRV_W<0> {
        VDD_SPI_PWR_DRV_W::new(self)
    }
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_force(&mut self) -> VDD_SPI_PWR_FORCE_W<2> {
        VDD_SPI_PWR_FORCE_W::new(self)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pd_en(&mut self) -> VDD_SPI_PD_EN_W<3> {
        VDD_SPI_PD_EN_W::new(self)
    }
    #[doc = "Bit 4 - memories in digital core force PD in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W<4> {
        LSLP_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 5 - memories in digital core force no PD in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W<5> {
        LSLP_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 19 - digital core force power down"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W<19> {
        DG_WRAP_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 20 - digital core force power up"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W<20> {
        DG_WRAP_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W<31> {
        DG_WRAP_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pwc](index.html) module"]
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_pwc::R](R) reader structure"]
impl crate::Readable for DIG_PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_pwc::W](W) writer structure"]
impl crate::Writable for DIG_PWC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_PWC to value 0x0010_0020"]
impl crate::Resettable for DIG_PWC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0020;
}
