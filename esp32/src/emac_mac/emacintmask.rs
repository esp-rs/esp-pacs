#[doc = "Register `EMACINTMASK` reader"]
pub type R = crate::R<EMACINTMASK_SPEC>;
#[doc = "Register `EMACINTMASK` writer"]
pub type W = crate::W<EMACINTMASK_SPEC>;
#[doc = "Field `PMTINTMASK` reader - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
pub type PMTINTMASK_R = crate::BitReader;
#[doc = "Field `PMTINTMASK` writer - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
pub type PMTINTMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPIINTMASK` reader - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
pub type LPIINTMASK_R = crate::BitReader;
#[doc = "Field `LPIINTMASK` writer - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
pub type LPIINTMASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtintmask(&self) -> PMTINTMASK_R {
        PMTINTMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiintmask(&self) -> LPIINTMASK_R {
        LPIINTMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACINTMASK")
            .field("pmtintmask", &format_args!("{}", self.pmtintmask().bit()))
            .field("lpiintmask", &format_args!("{}", self.lpiintmask().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACINTMASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    #[must_use]
    pub fn pmtintmask(&mut self) -> PMTINTMASK_W<EMACINTMASK_SPEC, 3> {
        PMTINTMASK_W::new(self)
    }
    #[doc = "Bit 10 - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    #[must_use]
    pub fn lpiintmask(&mut self) -> LPIINTMASK_W<EMACINTMASK_SPEC, 10> {
        LPIINTMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacintmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacintmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACINTMASK_SPEC;
impl crate::RegisterSpec for EMACINTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacintmask::R`](R) reader structure"]
impl crate::Readable for EMACINTMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacintmask::W`](W) writer structure"]
impl crate::Writable for EMACINTMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACINTMASK to value 0"]
impl crate::Resettable for EMACINTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
