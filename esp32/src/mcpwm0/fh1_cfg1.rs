#[doc = "Register `FH1_CFG1` reader"]
pub type R = crate::R<FH1_CFG1_SPEC>;
#[doc = "Register `FH1_CFG1` writer"]
pub type W = crate::W<FH1_CFG1_SPEC>;
#[doc = "Field `FH1_CLR_OST` reader - "]
pub type FH1_CLR_OST_R = crate::BitReader;
#[doc = "Field `FH1_CLR_OST` writer - "]
pub type FH1_CLR_OST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH1_CBCPULSE` reader - "]
pub type FH1_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `FH1_CBCPULSE` writer - "]
pub type FH1_CBCPULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FH1_FORCE_CBC` reader - "]
pub type FH1_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `FH1_FORCE_CBC` writer - "]
pub type FH1_FORCE_CBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FH1_FORCE_OST` reader - "]
pub type FH1_FORCE_OST_R = crate::BitReader;
#[doc = "Field `FH1_FORCE_OST` writer - "]
pub type FH1_FORCE_OST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh1_clr_ost(&self) -> FH1_CLR_OST_R {
        FH1_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh1_cbcpulse(&self) -> FH1_CBCPULSE_R {
        FH1_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh1_force_cbc(&self) -> FH1_FORCE_CBC_R {
        FH1_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh1_force_ost(&self) -> FH1_FORCE_OST_R {
        FH1_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH1_CFG1")
            .field("fh1_clr_ost", &format_args!("{}", self.fh1_clr_ost().bit()))
            .field(
                "fh1_cbcpulse",
                &format_args!("{}", self.fh1_cbcpulse().bits()),
            )
            .field(
                "fh1_force_cbc",
                &format_args!("{}", self.fh1_force_cbc().bit()),
            )
            .field(
                "fh1_force_ost",
                &format_args!("{}", self.fh1_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH1_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_clr_ost(&mut self) -> FH1_CLR_OST_W<FH1_CFG1_SPEC, 0> {
        FH1_CLR_OST_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_cbcpulse(&mut self) -> FH1_CBCPULSE_W<FH1_CFG1_SPEC, 1> {
        FH1_CBCPULSE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_force_cbc(&mut self) -> FH1_FORCE_CBC_W<FH1_CFG1_SPEC, 3> {
        FH1_FORCE_CBC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fh1_force_ost(&mut self) -> FH1_FORCE_OST_W<FH1_CFG1_SPEC, 4> {
        FH1_FORCE_OST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh1_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh1_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH1_CFG1_SPEC;
impl crate::RegisterSpec for FH1_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh1_cfg1::R`](R) reader structure"]
impl crate::Readable for FH1_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh1_cfg1::W`](W) writer structure"]
impl crate::Writable for FH1_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FH1_CFG1 to value 0"]
impl crate::Resettable for FH1_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
