#[doc = "Register `CPU_INT_TYPE` reader"]
pub type R = crate::R<CPU_INT_TYPE_SPEC>;
#[doc = "Register `CPU_INT_TYPE` writer"]
pub type W = crate::W<CPU_INT_TYPE_SPEC>;
#[doc = "Field `CPU_INT_TYPE` reader - reg_core0_cpu_int_type"]
pub type CPU_INT_TYPE_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_INT_TYPE` writer - reg_core0_cpu_int_type"]
pub type CPU_INT_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_type"]
    #[inline(always)]
    pub fn cpu_int_type(&self) -> CPU_INT_TYPE_R {
        CPU_INT_TYPE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_TYPE")
            .field(
                "cpu_int_type",
                &format_args!("{}", self.cpu_int_type().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_INT_TYPE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_type"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_type(&mut self) -> CPU_INT_TYPE_W<CPU_INT_TYPE_SPEC, 0> {
        CPU_INT_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_TYPE_SPEC;
impl crate::RegisterSpec for CPU_INT_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_type::R`](R) reader structure"]
impl crate::Readable for CPU_INT_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_type::W`](W) writer structure"]
impl crate::Writable for CPU_INT_TYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_INT_TYPE to value 0"]
impl crate::Resettable for CPU_INT_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
