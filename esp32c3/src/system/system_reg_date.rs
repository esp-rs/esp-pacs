#[doc = "Register `SYSTEM_REG_DATE` reader"]
pub type R = crate::R<SYSTEM_REG_DATE_SPEC>;
#[doc = "Register `SYSTEM_REG_DATE` writer"]
pub type W = crate::W<SYSTEM_REG_DATE_SPEC>;
#[doc = "Field `SYSTEM_REG_DATE` reader - reg_system_reg_date"]
pub type SYSTEM_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SYSTEM_REG_DATE` writer - reg_system_reg_date"]
pub type SYSTEM_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - reg_system_reg_date"]
    #[inline(always)]
    pub fn system_reg_date(&self) -> SYSTEM_REG_DATE_R {
        SYSTEM_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM_REG_DATE")
            .field(
                "system_reg_date",
                &format_args!("{}", self.system_reg_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYSTEM_REG_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - reg_system_reg_date"]
    #[inline(always)]
    #[must_use]
    pub fn system_reg_date(&mut self) -> SYSTEM_REG_DATE_W<SYSTEM_REG_DATE_SPEC> {
        SYSTEM_REG_DATE_W::new(self, 0)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_REG_DATE_SPEC;
impl crate::RegisterSpec for SYSTEM_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_reg_date::R`](R) reader structure"]
impl crate::Readable for SYSTEM_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`system_reg_date::W`](W) writer structure"]
impl crate::Writable for SYSTEM_REG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_REG_DATE to value 0x0200_7150"]
impl crate::Resettable for SYSTEM_REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0200_7150;
}
