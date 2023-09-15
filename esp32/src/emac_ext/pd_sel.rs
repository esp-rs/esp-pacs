#[doc = "Register `PD_SEL` reader"]
pub type R = crate::R<PD_SEL_SPEC>;
#[doc = "Register `PD_SEL` writer"]
pub type W = crate::W<PD_SEL_SPEC>;
#[doc = "Field `RAM_PD_EN` reader - "]
pub type RAM_PD_EN_R = crate::FieldReader;
#[doc = "Field `RAM_PD_EN` writer - "]
pub type RAM_PD_EN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ram_pd_en(&self) -> RAM_PD_EN_R {
        RAM_PD_EN_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_SEL")
            .field("ram_pd_en", &format_args!("{}", self.ram_pd_en().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PD_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ram_pd_en(&mut self) -> RAM_PD_EN_W<PD_SEL_SPEC, 0> {
        RAM_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet RAM power-down enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_SEL_SPEC;
impl crate::RegisterSpec for PD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_sel::R`](R) reader structure"]
impl crate::Readable for PD_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_sel::W`](W) writer structure"]
impl crate::Writable for PD_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD_SEL to value 0"]
impl crate::Resettable for PD_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
