#[doc = "Register `DIEPCTL4` reader"]
pub type R = crate::R<DIEPCTL4_SPEC>;
#[doc = "Register `DIEPCTL4` writer"]
pub type W = crate::W<DIEPCTL4_SPEC>;
#[doc = "Field `D_MPS4` reader - "]
pub type D_MPS4_R = crate::FieldReader;
#[doc = "Field `D_MPS4` writer - "]
pub type D_MPS4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `D_USBACTEP4` reader - "]
pub type D_USBACTEP4_R = crate::BitReader;
#[doc = "Field `D_NAKSTS4` reader - "]
pub type D_NAKSTS4_R = crate::BitReader;
#[doc = "Field `D_EPTYPE4` reader - "]
pub type D_EPTYPE4_R = crate::FieldReader;
#[doc = "Field `D_STALL4` reader - "]
pub type D_STALL4_R = crate::BitReader;
#[doc = "Field `D_STALL4` writer - "]
pub type D_STALL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D_TXFNUM4` reader - "]
pub type D_TXFNUM4_R = crate::FieldReader;
#[doc = "Field `D_TXFNUM4` writer - "]
pub type D_TXFNUM4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `D_CNAK4` writer - "]
pub type D_CNAK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SNAK4` writer - "]
pub type DI_SNAK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SETD0PID4` writer - "]
pub type DI_SETD0PID4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SETD1PID4` writer - "]
pub type DI_SETD1PID4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D_EPDIS4` reader - "]
pub type D_EPDIS4_R = crate::BitReader;
#[doc = "Field `D_EPDIS4` writer - "]
pub type D_EPDIS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D_EPENA4` reader - "]
pub type D_EPENA4_R = crate::BitReader;
#[doc = "Field `D_EPENA4` writer - "]
pub type D_EPENA4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn d_mps4(&self) -> D_MPS4_R {
        D_MPS4_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn d_usbactep4(&self) -> D_USBACTEP4_R {
        D_USBACTEP4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn d_naksts4(&self) -> D_NAKSTS4_R {
        D_NAKSTS4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn d_eptype4(&self) -> D_EPTYPE4_R {
        D_EPTYPE4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn d_stall4(&self) -> D_STALL4_R {
        D_STALL4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn d_txfnum4(&self) -> D_TXFNUM4_R {
        D_TXFNUM4_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn d_epdis4(&self) -> D_EPDIS4_R {
        D_EPDIS4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn d_epena4(&self) -> D_EPENA4_R {
        D_EPENA4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL4")
            .field("d_mps4", &format_args!("{}", self.d_mps4().bits()))
            .field("d_usbactep4", &format_args!("{}", self.d_usbactep4().bit()))
            .field("d_naksts4", &format_args!("{}", self.d_naksts4().bit()))
            .field("d_eptype4", &format_args!("{}", self.d_eptype4().bits()))
            .field("d_stall4", &format_args!("{}", self.d_stall4().bit()))
            .field("d_txfnum4", &format_args!("{}", self.d_txfnum4().bits()))
            .field("d_epdis4", &format_args!("{}", self.d_epdis4().bit()))
            .field("d_epena4", &format_args!("{}", self.d_epena4().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn d_mps4(&mut self) -> D_MPS4_W<DIEPCTL4_SPEC, 0> {
        D_MPS4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn d_stall4(&mut self) -> D_STALL4_W<DIEPCTL4_SPEC, 21> {
        D_STALL4_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfnum4(&mut self) -> D_TXFNUM4_W<DIEPCTL4_SPEC, 22> {
        D_TXFNUM4_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn d_cnak4(&mut self) -> D_CNAK4_W<DIEPCTL4_SPEC, 26> {
        D_CNAK4_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak4(&mut self) -> DI_SNAK4_W<DIEPCTL4_SPEC, 27> {
        DI_SNAK4_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid4(&mut self) -> DI_SETD0PID4_W<DIEPCTL4_SPEC, 28> {
        DI_SETD0PID4_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid4(&mut self) -> DI_SETD1PID4_W<DIEPCTL4_SPEC, 29> {
        DI_SETD1PID4_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdis4(&mut self) -> D_EPDIS4_W<DIEPCTL4_SPEC, 30> {
        D_EPDIS4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn d_epena4(&mut self) -> D_EPENA4_W<DIEPCTL4_SPEC, 31> {
        D_EPENA4_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL4_SPEC;
impl crate::RegisterSpec for DIEPCTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl4::R`](R) reader structure"]
impl crate::Readable for DIEPCTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl4::W`](W) writer structure"]
impl crate::Writable for DIEPCTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL4 to value 0x8000"]
impl crate::Resettable for DIEPCTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
