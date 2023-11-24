#[doc = "Register `FRC2_LOAD` reader"]
pub type R = crate::R<FRC2_LOAD_SPEC>;
#[doc = "Register `FRC2_LOAD` writer"]
pub type W = crate::W<FRC2_LOAD_SPEC>;
#[doc = "Field `frc2_load_value` reader - the load value into the counter"]
pub type FRC2_LOAD_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `frc2_load_value` writer - the load value into the counter"]
pub type FRC2_LOAD_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the load value into the counter"]
    #[inline(always)]
    pub fn frc2_load_value(&self) -> FRC2_LOAD_VALUE_R {
        FRC2_LOAD_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC2_LOAD")
            .field(
                "frc2_load_value",
                &format_args!("{}", self.frc2_load_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC2_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the load value into the counter"]
    #[inline(always)]
    #[must_use]
    pub fn frc2_load_value(&mut self) -> FRC2_LOAD_VALUE_W<FRC2_LOAD_SPEC> {
        FRC2_LOAD_VALUE_W::new(self, 0)
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
#[doc = "the load value into the counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC2_LOAD_SPEC;
impl crate::RegisterSpec for FRC2_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc2_load::R`](R) reader structure"]
impl crate::Readable for FRC2_LOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frc2_load::W`](W) writer structure"]
impl crate::Writable for FRC2_LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_LOAD to value 0"]
impl crate::Resettable for FRC2_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
