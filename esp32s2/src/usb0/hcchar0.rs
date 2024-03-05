#[doc = "Register `HCCHAR0` reader"]
pub type R = crate::R<HCCHAR0_SPEC>;
#[doc = "Register `HCCHAR0` writer"]
pub type W = crate::W<HCCHAR0_SPEC>;
#[doc = "Field `H_MPS0` reader - "]
pub type H_MPS0_R = crate::FieldReader<u16>;
#[doc = "Field `H_MPS0` writer - "]
pub type H_MPS0_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `H_EPNUM0` reader - "]
pub type H_EPNUM0_R = crate::FieldReader;
#[doc = "Field `H_EPNUM0` writer - "]
pub type H_EPNUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H_EPDIR0` reader - "]
pub type H_EPDIR0_R = crate::BitReader;
#[doc = "Field `H_EPDIR0` writer - "]
pub type H_EPDIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_LSPDDEV0` reader - "]
pub type H_LSPDDEV0_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV0` writer - "]
pub type H_LSPDDEV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_EPTYPE0` reader - "]
pub type H_EPTYPE0_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE0` writer - "]
pub type H_EPTYPE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_EC0` reader - "]
pub type H_EC0_R = crate::BitReader;
#[doc = "Field `H_EC0` writer - "]
pub type H_EC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DEVADDR0` reader - "]
pub type H_DEVADDR0_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR0` writer - "]
pub type H_DEVADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `H_ODDFRM0` reader - "]
pub type H_ODDFRM0_R = crate::BitReader;
#[doc = "Field `H_ODDFRM0` writer - "]
pub type H_ODDFRM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHDIS0` reader - "]
pub type H_CHDIS0_R = crate::BitReader;
#[doc = "Field `H_CHDIS0` writer - "]
pub type H_CHDIS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHENA0` reader - "]
pub type H_CHENA0_R = crate::BitReader;
#[doc = "Field `H_CHENA0` writer - "]
pub type H_CHENA0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps0(&self) -> H_MPS0_R {
        H_MPS0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum0(&self) -> H_EPNUM0_R {
        H_EPNUM0_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir0(&self) -> H_EPDIR0_R {
        H_EPDIR0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev0(&self) -> H_LSPDDEV0_R {
        H_LSPDDEV0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype0(&self) -> H_EPTYPE0_R {
        H_EPTYPE0_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec0(&self) -> H_EC0_R {
        H_EC0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr0(&self) -> H_DEVADDR0_R {
        H_DEVADDR0_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm0(&self) -> H_ODDFRM0_R {
        H_ODDFRM0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis0(&self) -> H_CHDIS0_R {
        H_CHDIS0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena0(&self) -> H_CHENA0_R {
        H_CHENA0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR0")
            .field("h_mps0", &format_args!("{}", self.h_mps0().bits()))
            .field("h_epnum0", &format_args!("{}", self.h_epnum0().bits()))
            .field("h_epdir0", &format_args!("{}", self.h_epdir0().bit()))
            .field("h_lspddev0", &format_args!("{}", self.h_lspddev0().bit()))
            .field("h_eptype0", &format_args!("{}", self.h_eptype0().bits()))
            .field("h_ec0", &format_args!("{}", self.h_ec0().bit()))
            .field("h_devaddr0", &format_args!("{}", self.h_devaddr0().bits()))
            .field("h_oddfrm0", &format_args!("{}", self.h_oddfrm0().bit()))
            .field("h_chdis0", &format_args!("{}", self.h_chdis0().bit()))
            .field("h_chena0", &format_args!("{}", self.h_chena0().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps0(&mut self) -> H_MPS0_W<HCCHAR0_SPEC> {
        H_MPS0_W::new(self, 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum0(&mut self) -> H_EPNUM0_W<HCCHAR0_SPEC> {
        H_EPNUM0_W::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir0(&mut self) -> H_EPDIR0_W<HCCHAR0_SPEC> {
        H_EPDIR0_W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev0(&mut self) -> H_LSPDDEV0_W<HCCHAR0_SPEC> {
        H_LSPDDEV0_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype0(&mut self) -> H_EPTYPE0_W<HCCHAR0_SPEC> {
        H_EPTYPE0_W::new(self, 18)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec0(&mut self) -> H_EC0_W<HCCHAR0_SPEC> {
        H_EC0_W::new(self, 21)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr0(&mut self) -> H_DEVADDR0_W<HCCHAR0_SPEC> {
        H_DEVADDR0_W::new(self, 22)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm0(&mut self) -> H_ODDFRM0_W<HCCHAR0_SPEC> {
        H_ODDFRM0_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis0(&mut self) -> H_CHDIS0_W<HCCHAR0_SPEC> {
        H_CHDIS0_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena0(&mut self) -> H_CHENA0_W<HCCHAR0_SPEC> {
        H_CHENA0_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR0_SPEC;
impl crate::RegisterSpec for HCCHAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar0::R`](R) reader structure"]
impl crate::Readable for HCCHAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar0::W`](W) writer structure"]
impl crate::Writable for HCCHAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCHAR0 to value 0"]
impl crate::Resettable for HCCHAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
