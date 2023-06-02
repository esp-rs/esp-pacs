#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_8` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_8` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP` reader - core_0_pif_pms_constrain_world_1_usb_wrap"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP` writer - core_0_pif_pms_constrain_world_1_usb_wrap"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI` reader - core_0_pif_pms_constrain_world_1_crypto_peri"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI` writer - core_0_pif_pms_constrain_world_1_crypto_peri"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA` reader - core_0_pif_pms_constrain_world_1_crypto_dma"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA` writer - core_0_pif_pms_constrain_world_1_crypto_dma"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC` reader - core_0_pif_pms_constrain_world_1_apb_adc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC` writer - core_0_pif_pms_constrain_world_1_apb_adc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR` reader - core_0_pif_pms_constrain_world_1_bt_pwr"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR` writer - core_0_pif_pms_constrain_world_1_bt_pwr"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE` reader - core_0_pif_pms_constrain_world_1_usb_device"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE` writer - core_0_pif_pms_constrain_world_1_usb_device"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM` reader - core_0_pif_pms_constrain_world_1_system"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM` writer - core_0_pif_pms_constrain_world_1_system"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE` reader - core_0_pif_pms_constrain_world_1_sensitive"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE` writer - core_0_pif_pms_constrain_world_1_sensitive"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT` reader - core_0_pif_pms_constrain_world_1_interrupt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT` writer - core_0_pif_pms_constrain_world_1_interrupt"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY` reader - core_0_pif_pms_constrain_world_1_dma_copy"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY` writer - core_0_pif_pms_constrain_world_1_dma_copy"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG` reader - core_0_pif_pms_constrain_world_1_cache_config"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG` writer - core_0_pif_pms_constrain_world_1_cache_config"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD` reader - core_0_pif_pms_constrain_world_1_ad"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD` writer - core_0_pif_pms_constrain_world_1_ad"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO` reader - core_0_pif_pms_constrain_world_1_dio"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO` writer - core_0_pif_pms_constrain_world_1_dio"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER` reader - core_0_pif_pms_constrain_world_1_world_controller"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER` writer - core_0_pif_pms_constrain_world_1_world_controller"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_8_SPEC, 2, O>;
impl R {
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_1_usb_wrap"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_usb_wrap(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_crypto_peri"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_crypto_peri(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_1_crypto_dma"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_crypto_dma(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_1_apb_adc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_apb_adc(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_1_bt_pwr"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_bt_pwr(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_1_usb_device"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_usb_device(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_1_system"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_system(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_1_sensitive"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_sensitive(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - core_0_pif_pms_constrain_world_1_interrupt"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_interrupt(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_dma_copy"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_dma_copy(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_1_cache_config"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_cache_config(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_ad"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_ad(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_dio"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_dio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_1_world_controller"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_1_world_controller(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_8")
            .field(
                "core_0_pif_pms_constrain_world_1_usb_wrap",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_usb_wrap().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_crypto_peri",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_crypto_peri().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_crypto_dma",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_crypto_dma().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_apb_adc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_apb_adc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_bt_pwr",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_bt_pwr().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_usb_device",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_usb_device().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_system",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_system().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_sensitive",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_sensitive().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_interrupt",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_interrupt().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_dma_copy",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_dma_copy().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_cache_config",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_cache_config().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_ad",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_ad().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_dio",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_1_dio().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_1_world_controller",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_1_world_controller()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_1_usb_wrap"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_usb_wrap(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W<2> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP_W::new(self)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_1_crypto_peri"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_crypto_peri(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI_W::new(self)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_1_crypto_dma"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_crypto_dma(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA_W::new(self)
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_1_apb_adc"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_apb_adc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC_W::new(self)
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_1_bt_pwr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_bt_pwr(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W<12> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR_W::new(self)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_1_usb_device"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_usb_device(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE_W::new(self)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_1_system"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_system(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM_W::new(self)
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_1_sensitive"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_sensitive(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W<18> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE_W::new(self)
    }
    #[doc = "Bits 20:21 - core_0_pif_pms_constrain_world_1_interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_interrupt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W<20> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT_W::new(self)
    }
    #[doc = "Bits 22:23 - core_0_pif_pms_constrain_world_1_dma_copy"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_dma_copy(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W<22> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY_W::new(self)
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_1_cache_config"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_cache_config(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W<24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG_W::new(self)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_1_ad"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_ad(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD_W::new(self)
    }
    #[doc = "Bits 28:29 - core_0_pif_pms_constrain_world_1_dio"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_dio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO_W::new(self)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_1_world_controller"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_1_world_controller(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W<30> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_8](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_8_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_8::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_8::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_8 to value 0xffff_f3fc"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_8_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_f3fc;
}
