#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_4` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_4` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` reader - BackUp access usb_device permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` writer - BackUp access usb_device permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` reader - BackUp access usb_wrap permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` writer - BackUp access usb_wrap permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` reader - BackUp access crypto_peri permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` writer - BackUp access crypto_peri permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` reader - BackUp access crypto_dma permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` writer - BackUp access crypto_dma permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` reader - BackUp access apb_adc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` writer - BackUp access apb_adc permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM` reader - BackUp access lcd_cam permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM` writer - BackUp access lcd_cam permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` reader - BackUp access bt_pwr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` writer - BackUp access bt_pwr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB` reader - BackUp access usb permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB` writer - BackUp access usb permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTEM` reader - BackUp access system permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SYSTEM` writer - BackUp access system permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE` reader - BackUp access sensitive permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE` writer - BackUp access sensitive permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT` reader - BackUp access interrupt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT` writer - BackUp access interrupt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY` reader - BackUp access dma_copy permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY` writer - BackUp access dma_copy permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG` reader - BackUp access cache_config permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG` writer - BackUp access cache_config permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_AD` reader - BackUp access ad permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_AD_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_AD` writer - BackUp access ad permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_AD_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DIO` reader - BackUp access dio permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_DIO_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_DIO` writer - BackUp access dio permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_DIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER` reader - BackUp access world_controller permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER` writer - BackUp access world_controller permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - BackUp access usb_device permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_device(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BackUp access usb_wrap permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_wrap(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access crypto_peri permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_peri(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access crypto_dma permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_dma(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access apb_adc permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_adc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access lcd_cam permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_lcd_cam(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R {
        BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BackUp access bt_pwr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access usb permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access system permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_system(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - BackUp access sensitive permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_sensitive(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R {
        BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - BackUp access interrupt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_interrupt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R {
        BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BackUp access dma_copy permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_dma_copy(&self) -> BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R {
        BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - BackUp access cache_config permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_cache_config(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R {
        BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access ad permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_ad(&self) -> BACKUP_BUS_PMS_CONSTRAIN_AD_R {
        BACKUP_BUS_PMS_CONSTRAIN_AD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access dio permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_dio(&self) -> BACKUP_BUS_PMS_CONSTRAIN_DIO_R {
        BACKUP_BUS_PMS_CONSTRAIN_DIO_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BackUp access world_controller permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_world_controller(
        &self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R {
        BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_4")
            .field(
                "backup_bus_pms_constrain_usb_device",
                &format_args!("{}", self.backup_bus_pms_constrain_usb_device().bits()),
            )
            .field(
                "backup_bus_pms_constrain_usb_wrap",
                &format_args!("{}", self.backup_bus_pms_constrain_usb_wrap().bits()),
            )
            .field(
                "backup_bus_pms_constrain_crypto_peri",
                &format_args!("{}", self.backup_bus_pms_constrain_crypto_peri().bits()),
            )
            .field(
                "backup_bus_pms_constrain_crypto_dma",
                &format_args!("{}", self.backup_bus_pms_constrain_crypto_dma().bits()),
            )
            .field(
                "backup_bus_pms_constrain_apb_adc",
                &format_args!("{}", self.backup_bus_pms_constrain_apb_adc().bits()),
            )
            .field(
                "backup_bus_pms_constrain_lcd_cam",
                &format_args!("{}", self.backup_bus_pms_constrain_lcd_cam().bits()),
            )
            .field(
                "backup_bus_pms_constrain_bt_pwr",
                &format_args!("{}", self.backup_bus_pms_constrain_bt_pwr().bits()),
            )
            .field(
                "backup_bus_pms_constrain_usb",
                &format_args!("{}", self.backup_bus_pms_constrain_usb().bits()),
            )
            .field(
                "backup_bus_pms_constrain_system",
                &format_args!("{}", self.backup_bus_pms_constrain_system().bits()),
            )
            .field(
                "backup_bus_pms_constrain_sensitive",
                &format_args!("{}", self.backup_bus_pms_constrain_sensitive().bits()),
            )
            .field(
                "backup_bus_pms_constrain_interrupt",
                &format_args!("{}", self.backup_bus_pms_constrain_interrupt().bits()),
            )
            .field(
                "backup_bus_pms_constrain_dma_copy",
                &format_args!("{}", self.backup_bus_pms_constrain_dma_copy().bits()),
            )
            .field(
                "backup_bus_pms_constrain_cache_config",
                &format_args!("{}", self.backup_bus_pms_constrain_cache_config().bits()),
            )
            .field(
                "backup_bus_pms_constrain_ad",
                &format_args!("{}", self.backup_bus_pms_constrain_ad().bits()),
            )
            .field(
                "backup_bus_pms_constrain_dio",
                &format_args!("{}", self.backup_bus_pms_constrain_dio().bits()),
            )
            .field(
                "backup_bus_pms_constrain_world_controller",
                &format_args!(
                    "{}",
                    self.backup_bus_pms_constrain_world_controller().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access usb_device permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_usb_device(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<0> {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W::new(self)
    }
    #[doc = "Bits 2:3 - BackUp access usb_wrap permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_usb_wrap(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<2> {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W::new(self)
    }
    #[doc = "Bits 4:5 - BackUp access crypto_peri permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_crypto_peri(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<4> {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W::new(self)
    }
    #[doc = "Bits 6:7 - BackUp access crypto_dma permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_crypto_dma(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<6> {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W::new(self)
    }
    #[doc = "Bits 8:9 - BackUp access apb_adc permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_apb_adc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<8> {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W::new(self)
    }
    #[doc = "Bits 10:11 - BackUp access lcd_cam permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_lcd_cam(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W<10> {
        BACKUP_BUS_PMS_CONSTRAIN_LCD_CAM_W::new(self)
    }
    #[doc = "Bits 12:13 - BackUp access bt_pwr permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bt_pwr(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<12> {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W::new(self)
    }
    #[doc = "Bits 14:15 - BackUp access usb permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_usb(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_W<14> {
        BACKUP_BUS_PMS_CONSTRAIN_USB_W::new(self)
    }
    #[doc = "Bits 16:17 - BackUp access system permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_system(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W<16> {
        BACKUP_BUS_PMS_CONSTRAIN_SYSTEM_W::new(self)
    }
    #[doc = "Bits 18:19 - BackUp access sensitive permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_sensitive(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W<18> {
        BACKUP_BUS_PMS_CONSTRAIN_SENSITIVE_W::new(self)
    }
    #[doc = "Bits 20:21 - BackUp access interrupt permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_interrupt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W<20> {
        BACKUP_BUS_PMS_CONSTRAIN_INTERRUPT_W::new(self)
    }
    #[doc = "Bits 22:23 - BackUp access dma_copy permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_dma_copy(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W<22> {
        BACKUP_BUS_PMS_CONSTRAIN_DMA_COPY_W::new(self)
    }
    #[doc = "Bits 24:25 - BackUp access cache_config permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_cache_config(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W<24> {
        BACKUP_BUS_PMS_CONSTRAIN_CACHE_CONFIG_W::new(self)
    }
    #[doc = "Bits 26:27 - BackUp access ad permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_ad(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_AD_W<26> {
        BACKUP_BUS_PMS_CONSTRAIN_AD_W::new(self)
    }
    #[doc = "Bits 28:29 - BackUp access dio permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_dio(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_DIO_W<28> {
        BACKUP_BUS_PMS_CONSTRAIN_DIO_W::new(self)
    }
    #[doc = "Bits 30:31 - BackUp access world_controller permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_world_controller(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W<30> {
        BACKUP_BUS_PMS_CONSTRAIN_WORLD_CONTROLLER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_4](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_4::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_4::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_4 to value 0xffff_ffff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
