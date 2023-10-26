#[doc = "Register `HCCHAR7` reader"]
pub type R = crate::R<HCCHAR7_SPEC>;
#[doc = "Register `HCCHAR7` writer"]
pub type W = crate::W<HCCHAR7_SPEC>;
#[doc = "Field `H_MPS7` reader - "]
pub type H_MPS7_R = crate::FieldReader<u16>;
#[doc = "Field `H_MPS7` writer - "]
pub type H_MPS7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `H_EPNUM7` reader - "]
pub type H_EPNUM7_R = crate::FieldReader;
#[doc = "Field `H_EPNUM7` writer - "]
pub type H_EPNUM7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `H_EPDIR7` reader - "]
pub type H_EPDIR7_R = crate::BitReader;
#[doc = "Field `H_EPDIR7` writer - "]
pub type H_EPDIR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_LSPDDEV7` reader - "]
pub type H_LSPDDEV7_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV7` writer - "]
pub type H_LSPDDEV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_EPTYPE7` reader - "]
pub type H_EPTYPE7_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE7` writer - "]
pub type H_EPTYPE7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `H_EC7` reader - "]
pub type H_EC7_R = crate::BitReader;
#[doc = "Field `H_EC7` writer - "]
pub type H_EC7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_DEVADDR7` reader - "]
pub type H_DEVADDR7_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR7` writer - "]
pub type H_DEVADDR7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `H_ODDFRM7` reader - "]
pub type H_ODDFRM7_R = crate::BitReader;
#[doc = "Field `H_ODDFRM7` writer - "]
pub type H_ODDFRM7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_CHDIS7` reader - "]
pub type H_CHDIS7_R = crate::BitReader;
#[doc = "Field `H_CHDIS7` writer - "]
pub type H_CHDIS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_CHENA7` reader - "]
pub type H_CHENA7_R = crate::BitReader;
#[doc = "Field `H_CHENA7` writer - "]
pub type H_CHENA7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps7(&self) -> H_MPS7_R {
        H_MPS7_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum7(&self) -> H_EPNUM7_R {
        H_EPNUM7_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir7(&self) -> H_EPDIR7_R {
        H_EPDIR7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev7(&self) -> H_LSPDDEV7_R {
        H_LSPDDEV7_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype7(&self) -> H_EPTYPE7_R {
        H_EPTYPE7_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec7(&self) -> H_EC7_R {
        H_EC7_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr7(&self) -> H_DEVADDR7_R {
        H_DEVADDR7_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm7(&self) -> H_ODDFRM7_R {
        H_ODDFRM7_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis7(&self) -> H_CHDIS7_R {
        H_CHDIS7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena7(&self) -> H_CHENA7_R {
        H_CHENA7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR7")
            .field("h_mps7", &format_args!("{}", self.h_mps7().bits()))
            .field("h_epnum7", &format_args!("{}", self.h_epnum7().bits()))
            .field("h_epdir7", &format_args!("{}", self.h_epdir7().bit()))
            .field("h_lspddev7", &format_args!("{}", self.h_lspddev7().bit()))
            .field("h_eptype7", &format_args!("{}", self.h_eptype7().bits()))
            .field("h_ec7", &format_args!("{}", self.h_ec7().bit()))
            .field("h_devaddr7", &format_args!("{}", self.h_devaddr7().bits()))
            .field("h_oddfrm7", &format_args!("{}", self.h_oddfrm7().bit()))
            .field("h_chdis7", &format_args!("{}", self.h_chdis7().bit()))
            .field("h_chena7", &format_args!("{}", self.h_chena7().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps7(&mut self) -> H_MPS7_W<HCCHAR7_SPEC, 0> {
        H_MPS7_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum7(&mut self) -> H_EPNUM7_W<HCCHAR7_SPEC, 11> {
        H_EPNUM7_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir7(&mut self) -> H_EPDIR7_W<HCCHAR7_SPEC, 15> {
        H_EPDIR7_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev7(&mut self) -> H_LSPDDEV7_W<HCCHAR7_SPEC, 17> {
        H_LSPDDEV7_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype7(&mut self) -> H_EPTYPE7_W<HCCHAR7_SPEC, 18> {
        H_EPTYPE7_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec7(&mut self) -> H_EC7_W<HCCHAR7_SPEC, 21> {
        H_EC7_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr7(&mut self) -> H_DEVADDR7_W<HCCHAR7_SPEC, 22> {
        H_DEVADDR7_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm7(&mut self) -> H_ODDFRM7_W<HCCHAR7_SPEC, 29> {
        H_ODDFRM7_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis7(&mut self) -> H_CHDIS7_W<HCCHAR7_SPEC, 30> {
        H_CHDIS7_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena7(&mut self) -> H_CHENA7_W<HCCHAR7_SPEC, 31> {
        H_CHENA7_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR7_SPEC;
impl crate::RegisterSpec for HCCHAR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar7::R`](R) reader structure"]
impl crate::Readable for HCCHAR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar7::W`](W) writer structure"]
impl crate::Writable for HCCHAR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR7 to value 0"]
impl crate::Resettable for HCCHAR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
