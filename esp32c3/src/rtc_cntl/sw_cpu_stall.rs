#[doc = "Register `SW_CPU_STALL` reader"]
pub type R = crate::R<SW_CPU_STALL_SPEC>;
#[doc = "Register `SW_CPU_STALL` writer"]
pub type W = crate::W<SW_CPU_STALL_SPEC>;
#[doc = "Field `SW_STALL_APPCPU_C1` reader - {reg_sw_stall_appcpu_c1\\[5:0\\]"]
pub type SW_STALL_APPCPU_C1_R = crate::FieldReader;
#[doc = "Field `SW_STALL_APPCPU_C1` writer - {reg_sw_stall_appcpu_c1\\[5:0\\]"]
pub type SW_STALL_APPCPU_C1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SW_STALL_PROCPU_C1` reader - stall cpu by software"]
pub type SW_STALL_PROCPU_C1_R = crate::FieldReader;
#[doc = "Field `SW_STALL_PROCPU_C1` writer - stall cpu by software"]
pub type SW_STALL_PROCPU_C1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\]"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&self) -> SW_STALL_APPCPU_C1_R {
        SW_STALL_APPCPU_C1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - stall cpu by software"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&self) -> SW_STALL_PROCPU_C1_R {
        SW_STALL_PROCPU_C1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_CPU_STALL")
            .field(
                "sw_stall_appcpu_c1",
                &format_args!("{}", self.sw_stall_appcpu_c1().bits()),
            )
            .field(
                "sw_stall_procpu_c1",
                &format_args!("{}", self.sw_stall_procpu_c1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_CPU_STALL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sw_stall_appcpu_c1(&mut self) -> SW_STALL_APPCPU_C1_W<SW_CPU_STALL_SPEC> {
        SW_STALL_APPCPU_C1_W::new(self, 20)
    }
    #[doc = "Bits 26:31 - stall cpu by software"]
    #[inline(always)]
    #[must_use]
    pub fn sw_stall_procpu_c1(&mut self) -> SW_STALL_PROCPU_C1_W<SW_CPU_STALL_SPEC> {
        SW_STALL_PROCPU_C1_W::new(self, 26)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_CPU_STALL_SPEC;
impl crate::RegisterSpec for SW_CPU_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_cpu_stall::R`](R) reader structure"]
impl crate::Readable for SW_CPU_STALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_cpu_stall::W`](W) writer structure"]
impl crate::Writable for SW_CPU_STALL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_CPU_STALL to value 0"]
impl crate::Resettable for SW_CPU_STALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
