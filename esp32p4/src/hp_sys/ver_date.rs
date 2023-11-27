#[doc = "Register `VER_DATE` reader"]
pub type R = crate::R<VER_DATE_SPEC>;
#[doc = "Register `VER_DATE` writer"]
pub type W = crate::W<VER_DATE_SPEC>;
#[doc = "Field `HP_REG_VER_DATE` reader - NA"]
pub type HP_REG_VER_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `HP_REG_VER_DATE` writer - NA"]
pub type HP_REG_VER_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn hp_reg_ver_date(&self) -> HP_REG_VER_DATE_R {
        HP_REG_VER_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER_DATE")
            .field(
                "hp_reg_ver_date",
                &format_args!("{}", self.hp_reg_ver_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VER_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_ver_date(&mut self) -> HP_REG_VER_DATE_W<VER_DATE_SPEC> {
        HP_REG_VER_DATE_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_DATE_SPEC;
impl crate::RegisterSpec for VER_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_date::R`](R) reader structure"]
impl crate::Readable for VER_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ver_date::W`](W) writer structure"]
impl crate::Writable for VER_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VER_DATE to value 0x2023_0519"]
impl crate::Resettable for VER_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2023_0519;
}
