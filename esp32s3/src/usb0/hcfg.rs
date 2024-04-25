#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFG_SPEC>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFG_SPEC>;
#[doc = "Field `FSLSPCLKSEL` reader - "]
pub type FSLSPCLKSEL_R = crate::FieldReader;
#[doc = "Field `FSLSPCLKSEL` writer - "]
pub type FSLSPCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSSUPP` reader - "]
pub type FSLSSUPP_R = crate::BitReader;
#[doc = "Field `FSLSSUPP` writer - "]
pub type FSLSSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZS` reader - "]
pub type ENA32KHZS_R = crate::BitReader;
#[doc = "Field `ENA32KHZS` writer - "]
pub type ENA32KHZS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCDMA` reader - "]
pub type DESCDMA_R = crate::BitReader;
#[doc = "Field `DESCDMA` writer - "]
pub type DESCDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRLISTEN` reader - "]
pub type FRLISTEN_R = crate::FieldReader;
#[doc = "Field `FRLISTEN` writer - "]
pub type FRLISTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERSCHEDENA` reader - "]
pub type PERSCHEDENA_R = crate::BitReader;
#[doc = "Field `PERSCHEDENA` writer - "]
pub type PERSCHEDENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODECHTIMEN` reader - "]
pub type MODECHTIMEN_R = crate::BitReader;
#[doc = "Field `MODECHTIMEN` writer - "]
pub type MODECHTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ena32khzs(&self) -> ENA32KHZS_R {
        ENA32KHZS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn frlisten(&self) -> FRLISTEN_R {
        FRLISTEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn perschedena(&self) -> PERSCHEDENA_R {
        PERSCHEDENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn modechtimen(&self) -> MODECHTIMEN_R {
        MODECHTIMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFG")
            .field(
                "fslspclksel",
                &format_args!("{}", self.fslspclksel().bits()),
            )
            .field("fslssupp", &format_args!("{}", self.fslssupp().bit()))
            .field("ena32khzs", &format_args!("{}", self.ena32khzs().bit()))
            .field("descdma", &format_args!("{}", self.descdma().bit()))
            .field("frlisten", &format_args!("{}", self.frlisten().bits()))
            .field("perschedena", &format_args!("{}", self.perschedena().bit()))
            .field("modechtimen", &format_args!("{}", self.modechtimen().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclksel(&mut self) -> FSLSPCLKSEL_W<HCFG_SPEC> {
        FSLSPCLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W<HCFG_SPEC> {
        FSLSSUPP_W::new(self, 2)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ena32khzs(&mut self) -> ENA32KHZS_W<HCFG_SPEC> {
        ENA32KHZS_W::new(self, 7)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn descdma(&mut self) -> DESCDMA_W<HCFG_SPEC> {
        DESCDMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn frlisten(&mut self) -> FRLISTEN_W<HCFG_SPEC> {
        FRLISTEN_W::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn perschedena(&mut self) -> PERSCHEDENA_W<HCFG_SPEC> {
        PERSCHEDENA_W::new(self, 26)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn modechtimen(&mut self) -> MODECHTIMEN_W<HCFG_SPEC> {
        MODECHTIMEN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
