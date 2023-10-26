#[doc = "Register `DIEPCTL5` reader"]
pub type R = crate::R<DIEPCTL5_SPEC>;
#[doc = "Register `DIEPCTL5` writer"]
pub type W = crate::W<DIEPCTL5_SPEC>;
#[doc = "Field `DI_MPS5` reader - "]
pub type DI_MPS5_R = crate::FieldReader;
#[doc = "Field `DI_MPS5` writer - "]
pub type DI_MPS5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DI_USBACTEP5` reader - "]
pub type DI_USBACTEP5_R = crate::BitReader;
#[doc = "Field `DI_NAKSTS5` reader - "]
pub type DI_NAKSTS5_R = crate::BitReader;
#[doc = "Field `DI_EPTYPE5` reader - "]
pub type DI_EPTYPE5_R = crate::FieldReader;
#[doc = "Field `DI_STALL5` reader - "]
pub type DI_STALL5_R = crate::BitReader;
#[doc = "Field `DI_STALL5` writer - "]
pub type DI_STALL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_TXFNUM5` reader - "]
pub type DI_TXFNUM5_R = crate::FieldReader;
#[doc = "Field `DI_TXFNUM5` writer - "]
pub type DI_TXFNUM5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DI_CNAK5` writer - "]
pub type DI_CNAK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SNAK5` writer - "]
pub type DI_SNAK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SETD0PID5` writer - "]
pub type DI_SETD0PID5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SETD1PID5` writer - "]
pub type DI_SETD1PID5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_EPDIS5` reader - "]
pub type DI_EPDIS5_R = crate::BitReader;
#[doc = "Field `DI_EPDIS5` writer - "]
pub type DI_EPDIS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_EPENA5` reader - "]
pub type DI_EPENA5_R = crate::BitReader;
#[doc = "Field `DI_EPENA5` writer - "]
pub type DI_EPENA5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn di_mps5(&self) -> DI_MPS5_R {
        DI_MPS5_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn di_usbactep5(&self) -> DI_USBACTEP5_R {
        DI_USBACTEP5_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn di_naksts5(&self) -> DI_NAKSTS5_R {
        DI_NAKSTS5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn di_eptype5(&self) -> DI_EPTYPE5_R {
        DI_EPTYPE5_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn di_stall5(&self) -> DI_STALL5_R {
        DI_STALL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn di_txfnum5(&self) -> DI_TXFNUM5_R {
        DI_TXFNUM5_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn di_epdis5(&self) -> DI_EPDIS5_R {
        DI_EPDIS5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn di_epena5(&self) -> DI_EPENA5_R {
        DI_EPENA5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL5")
            .field("di_mps5", &format_args!("{}", self.di_mps5().bits()))
            .field(
                "di_usbactep5",
                &format_args!("{}", self.di_usbactep5().bit()),
            )
            .field("di_naksts5", &format_args!("{}", self.di_naksts5().bit()))
            .field("di_eptype5", &format_args!("{}", self.di_eptype5().bits()))
            .field("di_stall5", &format_args!("{}", self.di_stall5().bit()))
            .field("di_txfnum5", &format_args!("{}", self.di_txfnum5().bits()))
            .field("di_epdis5", &format_args!("{}", self.di_epdis5().bit()))
            .field("di_epena5", &format_args!("{}", self.di_epena5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn di_mps5(&mut self) -> DI_MPS5_W<DIEPCTL5_SPEC, 0> {
        DI_MPS5_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn di_stall5(&mut self) -> DI_STALL5_W<DIEPCTL5_SPEC, 21> {
        DI_STALL5_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn di_txfnum5(&mut self) -> DI_TXFNUM5_W<DIEPCTL5_SPEC, 22> {
        DI_TXFNUM5_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn di_cnak5(&mut self) -> DI_CNAK5_W<DIEPCTL5_SPEC, 26> {
        DI_CNAK5_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak5(&mut self) -> DI_SNAK5_W<DIEPCTL5_SPEC, 27> {
        DI_SNAK5_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid5(&mut self) -> DI_SETD0PID5_W<DIEPCTL5_SPEC, 28> {
        DI_SETD0PID5_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid5(&mut self) -> DI_SETD1PID5_W<DIEPCTL5_SPEC, 29> {
        DI_SETD1PID5_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn di_epdis5(&mut self) -> DI_EPDIS5_W<DIEPCTL5_SPEC, 30> {
        DI_EPDIS5_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn di_epena5(&mut self) -> DI_EPENA5_W<DIEPCTL5_SPEC, 31> {
        DI_EPENA5_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL5_SPEC;
impl crate::RegisterSpec for DIEPCTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl5::R`](R) reader structure"]
impl crate::Readable for DIEPCTL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl5::W`](W) writer structure"]
impl crate::Writable for DIEPCTL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL5 to value 0x8000"]
impl crate::Resettable for DIEPCTL5_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
