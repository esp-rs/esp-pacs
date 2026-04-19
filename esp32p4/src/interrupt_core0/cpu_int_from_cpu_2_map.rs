#[doc = "Register `CPU_INT_FROM_CPU_2_MAP` reader"]
pub type R = crate::R<CPU_INT_FROM_CPU_2_MAP_SPEC>;
#[doc = "Register `CPU_INT_FROM_CPU_2_MAP` writer"]
pub type W = crate::W<CPU_INT_FROM_CPU_2_MAP_SPEC>;
#[doc = "Field `CPU_INT_FROM_CPU_2_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type CPU_INT_FROM_CPU_2_MAP_R = crate::FieldReader;
#[doc = "Field `CPU_INT_FROM_CPU_2_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type CPU_INT_FROM_CPU_2_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC` reader - NA"]
pub type CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC` writer - NA"]
pub type CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG` reader - NA"]
pub type CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG_R = crate::BitReader;
#[doc = "Field `CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG` writer - NA"]
pub type CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn cpu_int_from_cpu_2_map(&self) -> CPU_INT_FROM_CPU_2_MAP_R {
        CPU_INT_FROM_CPU_2_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn cpu_int_from_cpu_2_src_pass_in_sec(&self) -> CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC_R {
        CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn cpu_int_from_cpu_2_src_in_sec_flag(&self) -> CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG_R {
        CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_FROM_CPU_2_MAP")
            .field("cpu_int_from_cpu_2_map", &self.cpu_int_from_cpu_2_map())
            .field(
                "cpu_int_from_cpu_2_src_pass_in_sec",
                &self.cpu_int_from_cpu_2_src_pass_in_sec(),
            )
            .field(
                "cpu_int_from_cpu_2_src_in_sec_flag",
                &self.cpu_int_from_cpu_2_src_in_sec_flag(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn cpu_int_from_cpu_2_map(
        &mut self,
    ) -> CPU_INT_FROM_CPU_2_MAP_W<'_, CPU_INT_FROM_CPU_2_MAP_SPEC> {
        CPU_INT_FROM_CPU_2_MAP_W::new(self, 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn cpu_int_from_cpu_2_src_pass_in_sec(
        &mut self,
    ) -> CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC_W<'_, CPU_INT_FROM_CPU_2_MAP_SPEC> {
        CPU_INT_FROM_CPU_2_SRC_PASS_IN_SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn cpu_int_from_cpu_2_src_in_sec_flag(
        &mut self,
    ) -> CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG_W<'_, CPU_INT_FROM_CPU_2_MAP_SPEC> {
        CPU_INT_FROM_CPU_2_SRC_IN_SEC_FLAG_W::new(self, 7)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_int_from_cpu_2_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_int_from_cpu_2_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_FROM_CPU_2_MAP_SPEC;
impl crate::RegisterSpec for CPU_INT_FROM_CPU_2_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_from_cpu_2_map::R`](R) reader structure"]
impl crate::Readable for CPU_INT_FROM_CPU_2_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_from_cpu_2_map::W`](W) writer structure"]
impl crate::Writable for CPU_INT_FROM_CPU_2_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_INT_FROM_CPU_2_MAP to value 0"]
impl crate::Resettable for CPU_INT_FROM_CPU_2_MAP_SPEC {}
