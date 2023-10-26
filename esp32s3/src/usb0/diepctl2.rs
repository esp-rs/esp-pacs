#[doc = "Register `DIEPCTL2` reader"]
pub type R = crate::R<DIEPCTL2_SPEC>;
#[doc = "Register `DIEPCTL2` writer"]
pub type W = crate::W<DIEPCTL2_SPEC>;
#[doc = "Field `D_MPS2` reader - "]
pub type D_MPS2_R = crate::FieldReader;
#[doc = "Field `D_MPS2` writer - "]
pub type D_MPS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `D_USBACTEP2` reader - "]
pub type D_USBACTEP2_R = crate::BitReader;
#[doc = "Field `D_NAKSTS2` reader - "]
pub type D_NAKSTS2_R = crate::BitReader;
#[doc = "Field `D_EPTYPE2` reader - "]
pub type D_EPTYPE2_R = crate::FieldReader;
#[doc = "Field `D_STALL2` reader - "]
pub type D_STALL2_R = crate::BitReader;
#[doc = "Field `D_STALL2` writer - "]
pub type D_STALL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D_TXFNUM2` reader - "]
pub type D_TXFNUM2_R = crate::FieldReader;
#[doc = "Field `D_TXFNUM2` writer - "]
pub type D_TXFNUM2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `D_CNAK2` writer - "]
pub type D_CNAK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SNAK2` writer - "]
pub type DI_SNAK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SETD0PID2` writer - "]
pub type DI_SETD0PID2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI_SETD1PID2` writer - "]
pub type DI_SETD1PID2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D_EPDIS2` reader - "]
pub type D_EPDIS2_R = crate::BitReader;
#[doc = "Field `D_EPDIS2` writer - "]
pub type D_EPDIS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D_EPENA2` reader - "]
pub type D_EPENA2_R = crate::BitReader;
#[doc = "Field `D_EPENA2` writer - "]
pub type D_EPENA2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn d_mps2(&self) -> D_MPS2_R {
        D_MPS2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn d_usbactep2(&self) -> D_USBACTEP2_R {
        D_USBACTEP2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn d_naksts2(&self) -> D_NAKSTS2_R {
        D_NAKSTS2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn d_eptype2(&self) -> D_EPTYPE2_R {
        D_EPTYPE2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn d_stall2(&self) -> D_STALL2_R {
        D_STALL2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn d_txfnum2(&self) -> D_TXFNUM2_R {
        D_TXFNUM2_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn d_epdis2(&self) -> D_EPDIS2_R {
        D_EPDIS2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn d_epena2(&self) -> D_EPENA2_R {
        D_EPENA2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL2")
            .field("d_mps2", &format_args!("{}", self.d_mps2().bits()))
            .field("d_usbactep2", &format_args!("{}", self.d_usbactep2().bit()))
            .field("d_naksts2", &format_args!("{}", self.d_naksts2().bit()))
            .field("d_eptype2", &format_args!("{}", self.d_eptype2().bits()))
            .field("d_stall2", &format_args!("{}", self.d_stall2().bit()))
            .field("d_txfnum2", &format_args!("{}", self.d_txfnum2().bits()))
            .field("d_epdis2", &format_args!("{}", self.d_epdis2().bit()))
            .field("d_epena2", &format_args!("{}", self.d_epena2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn d_mps2(&mut self) -> D_MPS2_W<DIEPCTL2_SPEC, 0> {
        D_MPS2_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn d_stall2(&mut self) -> D_STALL2_W<DIEPCTL2_SPEC, 21> {
        D_STALL2_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfnum2(&mut self) -> D_TXFNUM2_W<DIEPCTL2_SPEC, 22> {
        D_TXFNUM2_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn d_cnak2(&mut self) -> D_CNAK2_W<DIEPCTL2_SPEC, 26> {
        D_CNAK2_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak2(&mut self) -> DI_SNAK2_W<DIEPCTL2_SPEC, 27> {
        DI_SNAK2_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid2(&mut self) -> DI_SETD0PID2_W<DIEPCTL2_SPEC, 28> {
        DI_SETD0PID2_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid2(&mut self) -> DI_SETD1PID2_W<DIEPCTL2_SPEC, 29> {
        DI_SETD1PID2_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdis2(&mut self) -> D_EPDIS2_W<DIEPCTL2_SPEC, 30> {
        D_EPDIS2_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn d_epena2(&mut self) -> D_EPENA2_W<DIEPCTL2_SPEC, 31> {
        D_EPENA2_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL2_SPEC;
impl crate::RegisterSpec for DIEPCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl2::R`](R) reader structure"]
impl crate::Readable for DIEPCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl2::W`](W) writer structure"]
impl crate::Writable for DIEPCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL2 to value 0x8000"]
impl crate::Resettable for DIEPCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
