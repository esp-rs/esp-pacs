#[doc = "Register `REGION_FILTER_EN` reader"]
pub type R = crate::R<REGION_FILTER_EN_SPEC>;
#[doc = "Register `REGION_FILTER_EN` writer"]
pub type W = crate::W<REGION_FILTER_EN_SPEC>;
#[doc = "Field `REGION_FILTER_EN` reader - Region filter enable"]
pub type REGION_FILTER_EN_R = crate::FieldReader<u16>;
#[doc = "Field `REGION_FILTER_EN` writer - Region filter enable"]
pub type REGION_FILTER_EN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Region filter enable"]
    #[inline(always)]
    pub fn region_filter_en(&self) -> REGION_FILTER_EN_R {
        REGION_FILTER_EN_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_FILTER_EN")
            .field(
                "region_filter_en",
                &format_args!("{}", self.region_filter_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION_FILTER_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Region filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn region_filter_en(&mut self) -> REGION_FILTER_EN_W<REGION_FILTER_EN_SPEC, 0> {
        REGION_FILTER_EN_W::new(self)
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
#[doc = "Region filter enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region_filter_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_filter_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION_FILTER_EN_SPEC;
impl crate::RegisterSpec for REGION_FILTER_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_filter_en::R`](R) reader structure"]
impl crate::Readable for REGION_FILTER_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region_filter_en::W`](W) writer structure"]
impl crate::Writable for REGION_FILTER_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION_FILTER_EN to value 0x01"]
impl crate::Resettable for REGION_FILTER_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
