#[doc = "Register `DIEPCTL` reader"]
pub type R = crate::R<DIEPCTL_SPEC>;
#[doc = "Register `DIEPCTL` writer"]
pub type W = crate::W<DIEPCTL_SPEC>;
#[doc = "Field `D_MPS1` reader - "]
pub type D_MPS1_R = crate::FieldReader;
#[doc = "Field `D_MPS1` writer - "]
pub type D_MPS1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D_USBACTEP1` reader - "]
pub type D_USBACTEP1_R = crate::BitReader;
#[doc = "Field `D_NAKSTS1` reader - "]
pub type D_NAKSTS1_R = crate::BitReader;
#[doc = "Field `D_EPTYPE1` reader - "]
pub type D_EPTYPE1_R = crate::FieldReader;
#[doc = "Field `D_STALL1` reader - "]
pub type D_STALL1_R = crate::BitReader;
#[doc = "Field `D_STALL1` writer - "]
pub type D_STALL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_TXFNUM1` reader - "]
pub type D_TXFNUM1_R = crate::FieldReader;
#[doc = "Field `D_TXFNUM1` writer - "]
pub type D_TXFNUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D_CNAK1` writer - "]
pub type D_CNAK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI_SNAK1` writer - "]
pub type DI_SNAK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI_SETD0PID1` writer - "]
pub type DI_SETD0PID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI_SETD1PID1` writer - "]
pub type DI_SETD1PID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_EPDIS1` reader - "]
pub type D_EPDIS1_R = crate::BitReader;
#[doc = "Field `D_EPDIS1` writer - "]
pub type D_EPDIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_EPENA1` reader - "]
pub type D_EPENA1_R = crate::BitReader;
#[doc = "Field `D_EPENA1` writer - "]
pub type D_EPENA1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn d_mps1(&self) -> D_MPS1_R {
        D_MPS1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn d_usbactep1(&self) -> D_USBACTEP1_R {
        D_USBACTEP1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn d_naksts1(&self) -> D_NAKSTS1_R {
        D_NAKSTS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn d_eptype1(&self) -> D_EPTYPE1_R {
        D_EPTYPE1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn d_stall1(&self) -> D_STALL1_R {
        D_STALL1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn d_txfnum1(&self) -> D_TXFNUM1_R {
        D_TXFNUM1_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn d_epdis1(&self) -> D_EPDIS1_R {
        D_EPDIS1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn d_epena1(&self) -> D_EPENA1_R {
        D_EPENA1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL")
            .field("d_mps1", &format_args!("{}", self.d_mps1().bits()))
            .field("d_usbactep1", &format_args!("{}", self.d_usbactep1().bit()))
            .field("d_naksts1", &format_args!("{}", self.d_naksts1().bit()))
            .field("d_eptype1", &format_args!("{}", self.d_eptype1().bits()))
            .field("d_stall1", &format_args!("{}", self.d_stall1().bit()))
            .field("d_txfnum1", &format_args!("{}", self.d_txfnum1().bits()))
            .field("d_epdis1", &format_args!("{}", self.d_epdis1().bit()))
            .field("d_epena1", &format_args!("{}", self.d_epena1().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn d_mps1(&mut self) -> D_MPS1_W<DIEPCTL_SPEC> {
        D_MPS1_W::new(self, 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn d_stall1(&mut self) -> D_STALL1_W<DIEPCTL_SPEC> {
        D_STALL1_W::new(self, 21)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfnum1(&mut self) -> D_TXFNUM1_W<DIEPCTL_SPEC> {
        D_TXFNUM1_W::new(self, 22)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn d_cnak1(&mut self) -> D_CNAK1_W<DIEPCTL_SPEC> {
        D_CNAK1_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak1(&mut self) -> DI_SNAK1_W<DIEPCTL_SPEC> {
        DI_SNAK1_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid1(&mut self) -> DI_SETD0PID1_W<DIEPCTL_SPEC> {
        DI_SETD0PID1_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid1(&mut self) -> DI_SETD1PID1_W<DIEPCTL_SPEC> {
        DI_SETD1PID1_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdis1(&mut self) -> D_EPDIS1_W<DIEPCTL_SPEC> {
        D_EPDIS1_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn d_epena1(&mut self) -> D_EPENA1_W<DIEPCTL_SPEC> {
        D_EPENA1_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL_SPEC;
impl crate::RegisterSpec for DIEPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl::R`](R) reader structure"]
impl crate::Readable for DIEPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl::W`](W) writer structure"]
impl crate::Writable for DIEPCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPCTL to value 0x8000"]
impl crate::Resettable for DIEPCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
