#[doc = "Register `APP_CACHE_CTRL` reader"]
pub struct R(crate::R<APP_CACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_CACHE_CTRL` writer"]
pub struct W(crate::W<APP_CACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_CACHE_CTRL_SPEC>;
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
impl From<crate::W<APP_CACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_CACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CACHE_MODE` reader - "]
pub type APP_CACHE_MODE_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MODE` writer - "]
pub type APP_CACHE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_CACHE_ENABLE` reader - "]
pub type APP_CACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `APP_CACHE_ENABLE` writer - "]
pub type APP_CACHE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_CACHE_FLUSH_ENA` reader - "]
pub type APP_CACHE_FLUSH_ENA_R = crate::BitReader;
#[doc = "Field `APP_CACHE_FLUSH_ENA` writer - "]
pub type APP_CACHE_FLUSH_ENA_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_CACHE_FLUSH_DONE` reader - "]
pub type APP_CACHE_FLUSH_DONE_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_0_EN` reader - "]
pub type APP_CACHE_LOCK_0_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_0_EN` writer - "]
pub type APP_CACHE_LOCK_0_EN_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_CACHE_LOCK_1_EN` reader - "]
pub type APP_CACHE_LOCK_1_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_1_EN` writer - "]
pub type APP_CACHE_LOCK_1_EN_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_CACHE_LOCK_2_EN` reader - "]
pub type APP_CACHE_LOCK_2_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_2_EN` writer - "]
pub type APP_CACHE_LOCK_2_EN_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_CACHE_LOCK_3_EN` reader - "]
pub type APP_CACHE_LOCK_3_EN_R = crate::BitReader;
#[doc = "Field `APP_CACHE_LOCK_3_EN` writer - "]
pub type APP_CACHE_LOCK_3_EN_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_SINGLE_IRAM_ENA` reader - "]
pub type APP_SINGLE_IRAM_ENA_R = crate::BitReader;
#[doc = "Field `APP_SINGLE_IRAM_ENA` writer - "]
pub type APP_SINGLE_IRAM_ENA_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_DRAM_SPLIT` reader - "]
pub type APP_DRAM_SPLIT_R = crate::BitReader;
#[doc = "Field `APP_DRAM_SPLIT` writer - "]
pub type APP_DRAM_SPLIT_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
#[doc = "Field `APP_AHB_SPI_REQ` reader - "]
pub type APP_AHB_SPI_REQ_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_REQ` reader - "]
pub type APP_SLAVE_REQ_R = crate::BitReader;
#[doc = "Field `APP_DRAM_HL` reader - "]
pub type APP_DRAM_HL_R = crate::BitReader;
#[doc = "Field `APP_DRAM_HL` writer - "]
pub type APP_DRAM_HL_W<'a, const O: u8> = crate::BitWriter<'a, APP_CACHE_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mode(&self) -> APP_CACHE_MODE_R {
        APP_CACHE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_enable(&self) -> APP_CACHE_ENABLE_R {
        APP_CACHE_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_flush_ena(&self) -> APP_CACHE_FLUSH_ENA_R {
        APP_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_flush_done(&self) -> APP_CACHE_FLUSH_DONE_R {
        APP_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn app_cache_lock_0_en(&self) -> APP_CACHE_LOCK_0_EN_R {
        APP_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_cache_lock_1_en(&self) -> APP_CACHE_LOCK_1_EN_R {
        APP_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_lock_2_en(&self) -> APP_CACHE_LOCK_2_EN_R {
        APP_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_lock_3_en(&self) -> APP_CACHE_LOCK_3_EN_R {
        APP_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn app_single_iram_ena(&self) -> APP_SINGLE_IRAM_ENA_R {
        APP_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_dram_split(&self) -> APP_DRAM_SPLIT_R {
        APP_DRAM_SPLIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_ahb_spi_req(&self) -> APP_AHB_SPI_REQ_R {
        APP_AHB_SPI_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_slave_req(&self) -> APP_SLAVE_REQ_R {
        APP_SLAVE_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn app_dram_hl(&self) -> APP_DRAM_HL_R {
        APP_DRAM_HL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CACHE_CTRL")
            .field(
                "app_cache_mode",
                &format_args!("{}", self.app_cache_mode().bit()),
            )
            .field(
                "app_cache_enable",
                &format_args!("{}", self.app_cache_enable().bit()),
            )
            .field(
                "app_cache_flush_ena",
                &format_args!("{}", self.app_cache_flush_ena().bit()),
            )
            .field(
                "app_cache_flush_done",
                &format_args!("{}", self.app_cache_flush_done().bit()),
            )
            .field(
                "app_cache_lock_0_en",
                &format_args!("{}", self.app_cache_lock_0_en().bit()),
            )
            .field(
                "app_cache_lock_1_en",
                &format_args!("{}", self.app_cache_lock_1_en().bit()),
            )
            .field(
                "app_cache_lock_2_en",
                &format_args!("{}", self.app_cache_lock_2_en().bit()),
            )
            .field(
                "app_cache_lock_3_en",
                &format_args!("{}", self.app_cache_lock_3_en().bit()),
            )
            .field(
                "app_single_iram_ena",
                &format_args!("{}", self.app_single_iram_ena().bit()),
            )
            .field(
                "app_dram_split",
                &format_args!("{}", self.app_dram_split().bit()),
            )
            .field(
                "app_ahb_spi_req",
                &format_args!("{}", self.app_ahb_spi_req().bit()),
            )
            .field(
                "app_slave_req",
                &format_args!("{}", self.app_slave_req().bit()),
            )
            .field("app_dram_hl", &format_args!("{}", self.app_dram_hl().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_CACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mode(&mut self) -> APP_CACHE_MODE_W<2> {
        APP_CACHE_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_enable(&mut self) -> APP_CACHE_ENABLE_W<3> {
        APP_CACHE_ENABLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_flush_ena(&mut self) -> APP_CACHE_FLUSH_ENA_W<4> {
        APP_CACHE_FLUSH_ENA_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_lock_0_en(&mut self) -> APP_CACHE_LOCK_0_EN_W<6> {
        APP_CACHE_LOCK_0_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_lock_1_en(&mut self) -> APP_CACHE_LOCK_1_EN_W<7> {
        APP_CACHE_LOCK_1_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_lock_2_en(&mut self) -> APP_CACHE_LOCK_2_EN_W<8> {
        APP_CACHE_LOCK_2_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_lock_3_en(&mut self) -> APP_CACHE_LOCK_3_EN_W<9> {
        APP_CACHE_LOCK_3_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn app_single_iram_ena(&mut self) -> APP_SINGLE_IRAM_ENA_W<10> {
        APP_SINGLE_IRAM_ENA_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn app_dram_split(&mut self) -> APP_DRAM_SPLIT_W<11> {
        APP_DRAM_SPLIT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn app_dram_hl(&mut self) -> APP_DRAM_HL_W<14> {
        APP_DRAM_HL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cache_ctrl](index.html) module"]
pub struct APP_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for APP_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cache_ctrl::R](R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_cache_ctrl::W](W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_CACHE_CTRL to value 0x10"]
impl crate::Resettable for APP_CACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
