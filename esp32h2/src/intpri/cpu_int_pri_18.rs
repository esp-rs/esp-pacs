#[doc = "Register `CPU_INT_PRI_18` reader"]
pub type R = crate::R<CPU_INT_PRI_18_SPEC>;
#[doc = "Register `CPU_INT_PRI_18` writer"]
pub type W = crate::W<CPU_INT_PRI_18_SPEC>;
#[doc = "Field `CPU_PRI_18_MAP` reader - Need add description"]
pub type CPU_PRI_18_MAP_R = crate::FieldReader;
#[doc = "Field `CPU_PRI_18_MAP` writer - Need add description"]
pub type CPU_PRI_18_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn cpu_pri_18_map(&self) -> CPU_PRI_18_MAP_R {
        CPU_PRI_18_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_PRI_18")
            .field(
                "cpu_pri_18_map",
                &format_args!("{}", self.cpu_pri_18_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_INT_PRI_18_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_pri_18_map(&mut self) -> CPU_PRI_18_MAP_W<CPU_INT_PRI_18_SPEC, 0> {
        CPU_PRI_18_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_PRI_18_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_pri_18::R`](R) reader structure"]
impl crate::Readable for CPU_INT_PRI_18_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_pri_18::W`](W) writer structure"]
impl crate::Writable for CPU_INT_PRI_18_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_INT_PRI_18 to value 0"]
impl crate::Resettable for CPU_INT_PRI_18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
