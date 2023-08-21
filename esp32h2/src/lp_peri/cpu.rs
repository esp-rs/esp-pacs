#[doc = "Register `CPU` reader"]
pub type R = crate::R<CPU_SPEC>;
#[doc = "Register `CPU` writer"]
pub type W = crate::W<CPU_SPEC>;
#[doc = "Field `LPCORE_DBGM_UNAVALIABLE` reader - need_des"]
pub type LPCORE_DBGM_UNAVALIABLE_R = crate::BitReader;
#[doc = "Field `LPCORE_DBGM_UNAVALIABLE` writer - need_des"]
pub type LPCORE_DBGM_UNAVALIABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lpcore_dbgm_unavaliable(&self) -> LPCORE_DBGM_UNAVALIABLE_R {
        LPCORE_DBGM_UNAVALIABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU")
            .field(
                "lpcore_dbgm_unavaliable",
                &format_args!("{}", self.lpcore_dbgm_unavaliable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpcore_dbgm_unavaliable(&mut self) -> LPCORE_DBGM_UNAVALIABLE_W<CPU_SPEC, 31> {
        LPCORE_DBGM_UNAVALIABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_SPEC;
impl crate::RegisterSpec for CPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu::R`](R) reader structure"]
impl crate::Readable for CPU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu::W`](W) writer structure"]
impl crate::Writable for CPU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU to value 0x8000_0000"]
impl crate::Resettable for CPU_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
