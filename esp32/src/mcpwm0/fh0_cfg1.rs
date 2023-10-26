#[doc = "Register `FH0_CFG1` reader"]
pub type R = crate::R<FH0_CFG1_SPEC>;
#[doc = "Register `FH0_CFG1` writer"]
pub type W = crate::W<FH0_CFG1_SPEC>;
#[doc = "Field `FH0_CLR_OST` reader - "]
pub type FH0_CLR_OST_R = crate::BitReader;
#[doc = "Field `FH0_CLR_OST` writer - "]
pub type FH0_CLR_OST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH0_CBCPULSE` reader - "]
pub type FH0_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `FH0_CBCPULSE` writer - "]
pub type FH0_CBCPULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FH0_FORCE_CBC` reader - "]
pub type FH0_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `FH0_FORCE_CBC` writer - "]
pub type FH0_FORCE_CBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH0_FORCE_OST` reader - "]
pub type FH0_FORCE_OST_R = crate::BitReader;
#[doc = "Field `FH0_FORCE_OST` writer - "]
pub type FH0_FORCE_OST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh0_clr_ost(&self) -> FH0_CLR_OST_R {
        FH0_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh0_cbcpulse(&self) -> FH0_CBCPULSE_R {
        FH0_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh0_force_cbc(&self) -> FH0_FORCE_CBC_R {
        FH0_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh0_force_ost(&self) -> FH0_FORCE_OST_R {
        FH0_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH0_CFG1")
            .field("fh0_clr_ost", &format_args!("{}", self.fh0_clr_ost().bit()))
            .field(
                "fh0_cbcpulse",
                &format_args!("{}", self.fh0_cbcpulse().bits()),
            )
            .field(
                "fh0_force_cbc",
                &format_args!("{}", self.fh0_force_cbc().bit()),
            )
            .field(
                "fh0_force_ost",
                &format_args!("{}", self.fh0_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH0_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_clr_ost(&mut self) -> FH0_CLR_OST_W<FH0_CFG1_SPEC, 0> {
        FH0_CLR_OST_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_cbcpulse(&mut self) -> FH0_CBCPULSE_W<FH0_CFG1_SPEC, 1> {
        FH0_CBCPULSE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_force_cbc(&mut self) -> FH0_FORCE_CBC_W<FH0_CFG1_SPEC, 3> {
        FH0_FORCE_CBC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fh0_force_ost(&mut self) -> FH0_FORCE_OST_W<FH0_CFG1_SPEC, 4> {
        FH0_FORCE_OST_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh0_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh0_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH0_CFG1_SPEC;
impl crate::RegisterSpec for FH0_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh0_cfg1::R`](R) reader structure"]
impl crate::Readable for FH0_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh0_cfg1::W`](W) writer structure"]
impl crate::Writable for FH0_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FH0_CFG1 to value 0"]
impl crate::Resettable for FH0_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
