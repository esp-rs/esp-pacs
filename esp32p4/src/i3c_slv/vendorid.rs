#[doc = "Register `VENDORID` reader"]
pub type R = crate::R<VENDORID_SPEC>;
#[doc = "Register `VENDORID` writer"]
pub type W = crate::W<VENDORID_SPEC>;
#[doc = "Field `VID` reader - NA"]
pub type VID_R = crate::FieldReader<u16>;
#[doc = "Field `VID` writer - NA"]
pub type VID_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn vid(&self) -> VID_R {
        VID_R::new((self.bits & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VENDORID")
            .field("vid", &format_args!("{}", self.vid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VENDORID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid(&mut self) -> VID_W<VENDORID_SPEC> {
        VID_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vendorid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vendorid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VENDORID_SPEC;
impl crate::RegisterSpec for VENDORID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vendorid::R`](R) reader structure"]
impl crate::Readable for VENDORID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vendorid::W`](W) writer structure"]
impl crate::Writable for VENDORID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VENDORID to value 0x5550"]
impl crate::Resettable for VENDORID_SPEC {
    const RESET_VALUE: Self::Ux = 0x5550;
}
