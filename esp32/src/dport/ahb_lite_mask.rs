#[doc = "Register `AHB_LITE_MASK` reader"]
pub struct R(crate::R<AHB_LITE_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_LITE_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_LITE_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_LITE_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_LITE_MASK` writer"]
pub struct W(crate::W<AHB_LITE_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_LITE_MASK_SPEC>;
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
impl From<crate::W<AHB_LITE_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_LITE_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO` reader - "]
pub type PRO_R = crate::BitReader<bool>;
#[doc = "Field `PRO` writer - "]
pub type PRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB_LITE_MASK_SPEC, bool, O>;
#[doc = "Field `APP` reader - "]
pub type APP_R = crate::BitReader<bool>;
#[doc = "Field `APP` writer - "]
pub type APP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB_LITE_MASK_SPEC, bool, O>;
#[doc = "Field `SDIO` reader - "]
pub type SDIO_R = crate::BitReader<bool>;
#[doc = "Field `SDIO` writer - "]
pub type SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB_LITE_MASK_SPEC, bool, O>;
#[doc = "Field `PRODPORT` reader - "]
pub type PRODPORT_R = crate::BitReader<bool>;
#[doc = "Field `PRODPORT` writer - "]
pub type PRODPORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB_LITE_MASK_SPEC, bool, O>;
#[doc = "Field `APPDPORT` reader - "]
pub type APPDPORT_R = crate::BitReader<bool>;
#[doc = "Field `APPDPORT` writer - "]
pub type APPDPORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB_LITE_MASK_SPEC, bool, O>;
#[doc = "Field `AHB_LITE_SDHOST_PID` reader - "]
pub type AHB_LITE_SDHOST_PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHB_LITE_SDHOST_PID` writer - "]
pub type AHB_LITE_SDHOST_PID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AHB_LITE_MASK_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn prodport(&self) -> PRODPORT_R {
        PRODPORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn appdport(&self) -> APPDPORT_R {
        APPDPORT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ahb_lite_sdhost_pid(&self) -> AHB_LITE_SDHOST_PID_R {
        AHB_LITE_SDHOST_PID_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&mut self) -> PRO_W<0> {
        PRO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app(&mut self) -> APP_W<4> {
        APP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W<8> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn prodport(&mut self) -> PRODPORT_W<9> {
        PRODPORT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn appdport(&mut self) -> APPDPORT_W<10> {
        APPDPORT_W::new(self)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ahb_lite_sdhost_pid(&mut self) -> AHB_LITE_SDHOST_PID_W<11> {
        AHB_LITE_SDHOST_PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_lite_mask](index.html) module"]
pub struct AHB_LITE_MASK_SPEC;
impl crate::RegisterSpec for AHB_LITE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_lite_mask::R](R) reader structure"]
impl crate::Readable for AHB_LITE_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_lite_mask::W](W) writer structure"]
impl crate::Writable for AHB_LITE_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB_LITE_MASK to value 0"]
impl crate::Resettable for AHB_LITE_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}