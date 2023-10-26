#[doc = "Register `AHB_LITE_MASK` reader"]
pub type R = crate::R<AHB_LITE_MASK_SPEC>;
#[doc = "Register `AHB_LITE_MASK` writer"]
pub type W = crate::W<AHB_LITE_MASK_SPEC>;
#[doc = "Field `PRO` reader - "]
pub type PRO_R = crate::BitReader;
#[doc = "Field `PRO` writer - "]
pub type PRO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APP` reader - "]
pub type APP_R = crate::BitReader;
#[doc = "Field `APP` writer - "]
pub type APP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO` reader - "]
pub type SDIO_R = crate::BitReader;
#[doc = "Field `SDIO` writer - "]
pub type SDIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRODPORT` reader - "]
pub type PRODPORT_R = crate::BitReader;
#[doc = "Field `PRODPORT` writer - "]
pub type PRODPORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APPDPORT` reader - "]
pub type APPDPORT_R = crate::BitReader;
#[doc = "Field `APPDPORT` writer - "]
pub type APPDPORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHB_LITE_SDHOST_PID` reader - "]
pub type AHB_LITE_SDHOST_PID_R = crate::FieldReader;
#[doc = "Field `AHB_LITE_SDHOST_PID` writer - "]
pub type AHB_LITE_SDHOST_PID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_LITE_MASK")
            .field("pro", &format_args!("{}", self.pro().bit()))
            .field("app", &format_args!("{}", self.app().bit()))
            .field("sdio", &format_args!("{}", self.sdio().bit()))
            .field("prodport", &format_args!("{}", self.prodport().bit()))
            .field("appdport", &format_args!("{}", self.appdport().bit()))
            .field(
                "ahb_lite_sdhost_pid",
                &format_args!("{}", self.ahb_lite_sdhost_pid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_LITE_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro(&mut self) -> PRO_W<AHB_LITE_MASK_SPEC, 0> {
        PRO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn app(&mut self) -> APP_W<AHB_LITE_MASK_SPEC, 4> {
        APP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<AHB_LITE_MASK_SPEC, 8> {
        SDIO_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn prodport(&mut self) -> PRODPORT_W<AHB_LITE_MASK_SPEC, 9> {
        PRODPORT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn appdport(&mut self) -> APPDPORT_W<AHB_LITE_MASK_SPEC, 10> {
        APPDPORT_W::new(self)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_lite_sdhost_pid(&mut self) -> AHB_LITE_SDHOST_PID_W<AHB_LITE_MASK_SPEC, 11> {
        AHB_LITE_SDHOST_PID_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_lite_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_lite_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_LITE_MASK_SPEC;
impl crate::RegisterSpec for AHB_LITE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_lite_mask::R`](R) reader structure"]
impl crate::Readable for AHB_LITE_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_lite_mask::W`](W) writer structure"]
impl crate::Writable for AHB_LITE_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_LITE_MASK to value 0"]
impl crate::Resettable for AHB_LITE_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
