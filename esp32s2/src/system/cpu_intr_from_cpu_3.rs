#[doc = "Register `CPU_INTR_FROM_CPU_3` reader"]
pub type R = crate::R<CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "Register `CPU_INTR_FROM_CPU_3` writer"]
pub type W = crate::W<CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "Field `CPU_INTR_FROM_CPU_3` reader - Set this bit to generate CPU interrupt 3. This bit needs to be reset by software in the ISR process."]
pub type CPU_INTR_FROM_CPU_3_R = crate::BitReader;
#[doc = "Field `CPU_INTR_FROM_CPU_3` writer - Set this bit to generate CPU interrupt 3. This bit needs to be reset by software in the ISR process."]
pub type CPU_INTR_FROM_CPU_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to generate CPU interrupt 3. This bit needs to be reset by software in the ISR process."]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_3(&self) -> CPU_INTR_FROM_CPU_3_R {
        CPU_INTR_FROM_CPU_3_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_3")
            .field(
                "cpu_intr_from_cpu_3",
                &format_args!("{}", self.cpu_intr_from_cpu_3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_INTR_FROM_CPU_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to generate CPU interrupt 3. This bit needs to be reset by software in the ISR process."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_intr_from_cpu_3(&mut self) -> CPU_INTR_FROM_CPU_3_W<CPU_INTR_FROM_CPU_3_SPEC, 0> {
        CPU_INTR_FROM_CPU_3_W::new(self)
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
#[doc = "CPU interrupt controlling register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INTR_FROM_CPU_3_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_intr_from_cpu_3::R`](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_intr_from_cpu_3::W`](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_3 to value 0"]
impl crate::Resettable for CPU_INTR_FROM_CPU_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
