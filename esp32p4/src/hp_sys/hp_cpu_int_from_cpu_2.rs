#[doc = "Register `HP_CPU_INT_FROM_CPU_2` reader"]
pub type R = crate::R<HP_CPU_INT_FROM_CPU_2_SPEC>;
#[doc = "Register `HP_CPU_INT_FROM_CPU_2` writer"]
pub type W = crate::W<HP_CPU_INT_FROM_CPU_2_SPEC>;
#[doc = "Field `HP_CPU_INT_FROM_CPU_2` reader - set 1 will triger a interrupt"]
pub type HP_CPU_INT_FROM_CPU_2_R = crate::BitReader;
#[doc = "Field `HP_CPU_INT_FROM_CPU_2` writer - set 1 will triger a interrupt"]
pub type HP_CPU_INT_FROM_CPU_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set 1 will triger a interrupt"]
    #[inline(always)]
    pub fn hp_cpu_int_from_cpu_2(&self) -> HP_CPU_INT_FROM_CPU_2_R {
        HP_CPU_INT_FROM_CPU_2_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CPU_INT_FROM_CPU_2")
            .field(
                "hp_cpu_int_from_cpu_2",
                &format_args!("{}", self.hp_cpu_int_from_cpu_2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CPU_INT_FROM_CPU_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - set 1 will triger a interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn hp_cpu_int_from_cpu_2(&mut self) -> HP_CPU_INT_FROM_CPU_2_W<HP_CPU_INT_FROM_CPU_2_SPEC> {
        HP_CPU_INT_FROM_CPU_2_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_int_from_cpu_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CPU_INT_FROM_CPU_2_SPEC;
impl crate::RegisterSpec for HP_CPU_INT_FROM_CPU_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cpu_int_from_cpu_2::R`](R) reader structure"]
impl crate::Readable for HP_CPU_INT_FROM_CPU_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cpu_int_from_cpu_2::W`](W) writer structure"]
impl crate::Writable for HP_CPU_INT_FROM_CPU_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CPU_INT_FROM_CPU_2 to value 0"]
impl crate::Resettable for HP_CPU_INT_FROM_CPU_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
