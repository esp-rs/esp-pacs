#[doc = "Register `UHS` reader"]
pub type R = crate::R<UHS_SPEC>;
#[doc = "Register `UHS` writer"]
pub type W = crate::W<UHS_SPEC>;
#[doc = "Field `DDR` reader - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DDR_R = crate::FieldReader;
#[doc = "Field `DDR` writer - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHS")
            .field("ddr", &format_args!("{}", self.ddr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UHS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<UHS_SPEC, 16> {
        DDR_W::new(self)
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
#[doc = "UHS-1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHS_SPEC;
impl crate::RegisterSpec for UHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs::R`](R) reader structure"]
impl crate::Readable for UHS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhs::W`](W) writer structure"]
impl crate::Writable for UHS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHS to value 0"]
impl crate::Resettable for UHS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
