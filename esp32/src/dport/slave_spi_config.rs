#[doc = "Register `SLAVE_SPI_CONFIG` reader"]
pub struct R(crate::R<SLAVE_SPI_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPI_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPI_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPI_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_SPI_CONFIG` writer"]
pub struct W(crate::W<SLAVE_SPI_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPI_CONFIG_SPEC>;
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
impl From<crate::W<SLAVE_SPI_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPI_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_SPI_MASK_PRO` reader - "]
pub type SLAVE_SPI_MASK_PRO_R = crate::BitReader;
#[doc = "Field `SLAVE_SPI_MASK_PRO` writer - "]
pub type SLAVE_SPI_MASK_PRO_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPI_CONFIG_SPEC, O>;
#[doc = "Field `SLAVE_SPI_MASK_APP` reader - "]
pub type SLAVE_SPI_MASK_APP_R = crate::BitReader;
#[doc = "Field `SLAVE_SPI_MASK_APP` writer - "]
pub type SLAVE_SPI_MASK_APP_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPI_CONFIG_SPEC, O>;
#[doc = "Field `SPI_ENCRYPT_ENABLE` reader - "]
pub type SPI_ENCRYPT_ENABLE_R = crate::BitReader;
#[doc = "Field `SPI_ENCRYPT_ENABLE` writer - "]
pub type SPI_ENCRYPT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPI_CONFIG_SPEC, O>;
#[doc = "Field `SPI_DECRYPT_ENABLE` reader - "]
pub type SPI_DECRYPT_ENABLE_R = crate::BitReader;
#[doc = "Field `SPI_DECRYPT_ENABLE` writer - "]
pub type SPI_DECRYPT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPI_CONFIG_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_spi_mask_pro(&self) -> SLAVE_SPI_MASK_PRO_R {
        SLAVE_SPI_MASK_PRO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_spi_mask_app(&self) -> SLAVE_SPI_MASK_APP_R {
        SLAVE_SPI_MASK_APP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_encrypt_enable(&self) -> SPI_ENCRYPT_ENABLE_R {
        SPI_ENCRYPT_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_decrypt_enable(&self) -> SPI_DECRYPT_ENABLE_R {
        SPI_DECRYPT_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE_SPI_CONFIG")
            .field(
                "slave_spi_mask_pro",
                &format_args!("{}", self.slave_spi_mask_pro().bit()),
            )
            .field(
                "slave_spi_mask_app",
                &format_args!("{}", self.slave_spi_mask_app().bit()),
            )
            .field(
                "spi_encrypt_enable",
                &format_args!("{}", self.spi_encrypt_enable().bit()),
            )
            .field(
                "spi_decrypt_enable",
                &format_args!("{}", self.spi_decrypt_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE_SPI_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slave_spi_mask_pro(&mut self) -> SLAVE_SPI_MASK_PRO_W<0> {
        SLAVE_SPI_MASK_PRO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slave_spi_mask_app(&mut self) -> SLAVE_SPI_MASK_APP_W<4> {
        SLAVE_SPI_MASK_APP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn spi_encrypt_enable(&mut self) -> SPI_ENCRYPT_ENABLE_W<8> {
        SPI_ENCRYPT_ENABLE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn spi_decrypt_enable(&mut self) -> SPI_DECRYPT_ENABLE_W<12> {
        SPI_DECRYPT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_spi_config](index.html) module"]
pub struct SLAVE_SPI_CONFIG_SPEC;
impl crate::RegisterSpec for SLAVE_SPI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_spi_config::R](R) reader structure"]
impl crate::Readable for SLAVE_SPI_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_spi_config::W](W) writer structure"]
impl crate::Writable for SLAVE_SPI_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE_SPI_CONFIG to value 0"]
impl crate::Resettable for SLAVE_SPI_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
