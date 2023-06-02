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
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` reader - backup_bus_pms_constrain_usb_wrap"]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP` writer - backup_bus_pms_constrain_usb_wrap"]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` reader - backup_bus_pms_constrain_crypto_peri"]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI` writer - backup_bus_pms_constrain_crypto_peri"]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` reader - backup_bus_pms_constrain_crypto_dma"]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA` writer - backup_bus_pms_constrain_crypto_dma"]
pub type BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` reader - backup_bus_pms_constrain_apb_adc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_ADC` writer - backup_bus_pms_constrain_apb_adc"]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` reader - backup_bus_pms_constrain_bt_pwr"]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_BT_PWR` writer - backup_bus_pms_constrain_bt_pwr"]
pub type BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` reader - backup_bus_pms_constrain_usb_device"]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE` writer - backup_bus_pms_constrain_usb_device"]
pub type BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<'a, const O: u8> =
    crate::FieldWriter<'a, BACKUP_BUS_PMS_CONSTRAIN_4_SPEC, 2, O>;
impl R {
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_usb_wrap"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_wrap(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_crypto_peri"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_peri(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_crypto_dma"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_crypto_dma(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_apb_adc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_adc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_bt_pwr"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_bt_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_usb_device"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_usb_device(&self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_4")
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
                "backup_bus_pms_constrain_bt_pwr",
                &format_args!("{}", self.backup_bus_pms_constrain_bt_pwr().bits()),
            )
            .field(
                "backup_bus_pms_constrain_usb_device",
                &format_args!("{}", self.backup_bus_pms_constrain_usb_device().bits()),
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
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_usb_wrap"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_usb_wrap(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W<2> {
        BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP_W::new(self)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_crypto_peri"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_crypto_peri(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W<4> {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI_W::new(self)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_crypto_dma"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_crypto_dma(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W<6> {
        BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA_W::new(self)
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_apb_adc"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_apb_adc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W<8> {
        BACKUP_BUS_PMS_CONSTRAIN_APB_ADC_W::new(self)
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_bt_pwr"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_bt_pwr(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W<12> {
        BACKUP_BUS_PMS_CONSTRAIN_BT_PWR_W::new(self)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_usb_device"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_usb_device(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W<14> {
        BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_4](index.html) module"]
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
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_4 to value 0xf3fc"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_4_SPEC {
    const RESET_VALUE: Self::Ux = 0xf3fc;
}
