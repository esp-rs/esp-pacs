#[doc = "Register `HP_SLEEP_DIG_POWER` reader"]
pub struct R(crate::R<HP_SLEEP_DIG_POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_SLEEP_DIG_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_SLEEP_DIG_POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_SLEEP_DIG_POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_SLEEP_DIG_POWER` writer"]
pub struct W(crate::W<HP_SLEEP_DIG_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_SLEEP_DIG_POWER_SPEC>;
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
impl From<crate::W<HP_SLEEP_DIG_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_SLEEP_DIG_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_SLEEP_VDD_SPI_PD_EN` reader - need_des"]
pub type HP_SLEEP_VDD_SPI_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_VDD_SPI_PD_EN` writer - need_des"]
pub type HP_SLEEP_VDD_SPI_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_DIG_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_HP_MEM_DSLP` reader - need_des"]
pub type HP_SLEEP_HP_MEM_DSLP_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_HP_MEM_DSLP` writer - need_des"]
pub type HP_SLEEP_HP_MEM_DSLP_W<'a, const O: u8> = crate::BitWriter<'a, HP_SLEEP_DIG_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_PD_HP_MEM_PD_EN` reader - need_des"]
pub type HP_SLEEP_PD_HP_MEM_PD_EN_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_PD_HP_MEM_PD_EN` writer - need_des"]
pub type HP_SLEEP_PD_HP_MEM_PD_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_SLEEP_DIG_POWER_SPEC, 4, O>;
#[doc = "Field `HP_SLEEP_PD_HP_WIFI_PD_EN` reader - need_des"]
pub type HP_SLEEP_PD_HP_WIFI_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_HP_WIFI_PD_EN` writer - need_des"]
pub type HP_SLEEP_PD_HP_WIFI_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_DIG_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_PD_HP_CPU_PD_EN` reader - need_des"]
pub type HP_SLEEP_PD_HP_CPU_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_HP_CPU_PD_EN` writer - need_des"]
pub type HP_SLEEP_PD_HP_CPU_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_DIG_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_PD_HP_AON_PD_EN` reader - need_des"]
pub type HP_SLEEP_PD_HP_AON_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_HP_AON_PD_EN` writer - need_des"]
pub type HP_SLEEP_PD_HP_AON_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_DIG_POWER_SPEC, O>;
#[doc = "Field `HP_SLEEP_PD_TOP_PD_EN` reader - need_des"]
pub type HP_SLEEP_PD_TOP_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_TOP_PD_EN` writer - need_des"]
pub type HP_SLEEP_PD_TOP_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, HP_SLEEP_DIG_POWER_SPEC, O>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_vdd_spi_pd_en(&self) -> HP_SLEEP_VDD_SPI_PD_EN_R {
        HP_SLEEP_VDD_SPI_PD_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_hp_mem_dslp(&self) -> HP_SLEEP_HP_MEM_DSLP_R {
        HP_SLEEP_HP_MEM_DSLP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_hp_mem_pd_en(&self) -> HP_SLEEP_PD_HP_MEM_PD_EN_R {
        HP_SLEEP_PD_HP_MEM_PD_EN_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_hp_wifi_pd_en(&self) -> HP_SLEEP_PD_HP_WIFI_PD_EN_R {
        HP_SLEEP_PD_HP_WIFI_PD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_hp_cpu_pd_en(&self) -> HP_SLEEP_PD_HP_CPU_PD_EN_R {
        HP_SLEEP_PD_HP_CPU_PD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_hp_aon_pd_en(&self) -> HP_SLEEP_PD_HP_AON_PD_EN_R {
        HP_SLEEP_PD_HP_AON_PD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_top_pd_en(&self) -> HP_SLEEP_PD_TOP_PD_EN_R {
        HP_SLEEP_PD_TOP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_DIG_POWER")
            .field(
                "hp_sleep_vdd_spi_pd_en",
                &format_args!("{}", self.hp_sleep_vdd_spi_pd_en().bit()),
            )
            .field(
                "hp_sleep_hp_mem_dslp",
                &format_args!("{}", self.hp_sleep_hp_mem_dslp().bit()),
            )
            .field(
                "hp_sleep_pd_hp_mem_pd_en",
                &format_args!("{}", self.hp_sleep_pd_hp_mem_pd_en().bits()),
            )
            .field(
                "hp_sleep_pd_hp_wifi_pd_en",
                &format_args!("{}", self.hp_sleep_pd_hp_wifi_pd_en().bit()),
            )
            .field(
                "hp_sleep_pd_hp_cpu_pd_en",
                &format_args!("{}", self.hp_sleep_pd_hp_cpu_pd_en().bit()),
            )
            .field(
                "hp_sleep_pd_hp_aon_pd_en",
                &format_args!("{}", self.hp_sleep_pd_hp_aon_pd_en().bit()),
            )
            .field(
                "hp_sleep_pd_top_pd_en",
                &format_args!("{}", self.hp_sleep_pd_top_pd_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_DIG_POWER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_vdd_spi_pd_en(&mut self) -> HP_SLEEP_VDD_SPI_PD_EN_W<21> {
        HP_SLEEP_VDD_SPI_PD_EN_W::new(self)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_hp_mem_dslp(&mut self) -> HP_SLEEP_HP_MEM_DSLP_W<22> {
        HP_SLEEP_HP_MEM_DSLP_W::new(self)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_hp_mem_pd_en(&mut self) -> HP_SLEEP_PD_HP_MEM_PD_EN_W<23> {
        HP_SLEEP_PD_HP_MEM_PD_EN_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_hp_wifi_pd_en(&mut self) -> HP_SLEEP_PD_HP_WIFI_PD_EN_W<27> {
        HP_SLEEP_PD_HP_WIFI_PD_EN_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_hp_cpu_pd_en(&mut self) -> HP_SLEEP_PD_HP_CPU_PD_EN_W<29> {
        HP_SLEEP_PD_HP_CPU_PD_EN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_hp_aon_pd_en(&mut self) -> HP_SLEEP_PD_HP_AON_PD_EN_W<30> {
        HP_SLEEP_PD_HP_AON_PD_EN_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_top_pd_en(&mut self) -> HP_SLEEP_PD_TOP_PD_EN_W<31> {
        HP_SLEEP_PD_TOP_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_sleep_dig_power](index.html) module"]
pub struct HP_SLEEP_DIG_POWER_SPEC;
impl crate::RegisterSpec for HP_SLEEP_DIG_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_sleep_dig_power::R](R) reader structure"]
impl crate::Readable for HP_SLEEP_DIG_POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_sleep_dig_power::W](W) writer structure"]
impl crate::Writable for HP_SLEEP_DIG_POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_DIG_POWER to value 0"]
impl crate::Resettable for HP_SLEEP_DIG_POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
