#[doc = "Register `FRC1_LOAD` reader"]
pub type R = crate::R<FRC1_LOAD_SPEC>;
#[doc = "Register `FRC1_LOAD` writer"]
pub type W = crate::W<FRC1_LOAD_SPEC>;
#[doc = "Field `frc1_load_value` reader - the load value into the counter"]
pub type FRC1_LOAD_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `frc1_load_value` writer - the load value into the counter"]
pub type FRC1_LOAD_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 23, O, u32>;
impl R {
    #[doc = "Bits 0:22 - the load value into the counter"]
    #[inline(always)]
    pub fn frc1_load_value(&self) -> FRC1_LOAD_VALUE_R {
        FRC1_LOAD_VALUE_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC1_LOAD")
            .field(
                "frc1_load_value",
                &format_args!("{}", self.frc1_load_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC1_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:22 - the load value into the counter"]
    #[inline(always)]
    #[must_use]
    pub fn frc1_load_value(&mut self) -> FRC1_LOAD_VALUE_W<FRC1_LOAD_SPEC, 0> {
        FRC1_LOAD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "the load value into the counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc1_load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC1_LOAD_SPEC;
impl crate::RegisterSpec for FRC1_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc1_load::R`](R) reader structure"]
impl crate::Readable for FRC1_LOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frc1_load::W`](W) writer structure"]
impl crate::Writable for FRC1_LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC1_LOAD to value 0"]
impl crate::Resettable for FRC1_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
