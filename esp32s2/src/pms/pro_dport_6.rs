#[doc = "Register `PRO_DPORT_6` reader"]
pub type R = crate::R<PRO_DPORT_6_SPEC>;
#[doc = "Register `PRO_DPORT_6` writer"]
pub type W = crate::W<PRO_DPORT_6_SPEC>;
#[doc = "Field `PRO_DPORT_ILG_CLR` reader - The clear signal for PeriBus1 access interrupt."]
pub type PRO_DPORT_ILG_CLR_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_ILG_CLR` writer - The clear signal for PeriBus1 access interrupt."]
pub type PRO_DPORT_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_ILG_EN` reader - The enable signal for PeriBus1 access interrupt."]
pub type PRO_DPORT_ILG_EN_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_ILG_EN` writer - The enable signal for PeriBus1 access interrupt."]
pub type PRO_DPORT_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_ILG_INTR` reader - PeriBus1 access interrupt signal."]
pub type PRO_DPORT_ILG_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for PeriBus1 access interrupt."]
    #[inline(always)]
    pub fn pro_dport_ilg_clr(&self) -> PRO_DPORT_ILG_CLR_R {
        PRO_DPORT_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for PeriBus1 access interrupt."]
    #[inline(always)]
    pub fn pro_dport_ilg_en(&self) -> PRO_DPORT_ILG_EN_R {
        PRO_DPORT_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PeriBus1 access interrupt signal."]
    #[inline(always)]
    pub fn pro_dport_ilg_intr(&self) -> PRO_DPORT_ILG_INTR_R {
        PRO_DPORT_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_6")
            .field(
                "pro_dport_ilg_clr",
                &format_args!("{}", self.pro_dport_ilg_clr().bit()),
            )
            .field(
                "pro_dport_ilg_en",
                &format_args!("{}", self.pro_dport_ilg_en().bit()),
            )
            .field(
                "pro_dport_ilg_intr",
                &format_args!("{}", self.pro_dport_ilg_intr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for PeriBus1 access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_ilg_clr(&mut self) -> PRO_DPORT_ILG_CLR_W<PRO_DPORT_6_SPEC> {
        PRO_DPORT_ILG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for PeriBus1 access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_ilg_en(&mut self) -> PRO_DPORT_ILG_EN_W<PRO_DPORT_6_SPEC> {
        PRO_DPORT_ILG_EN_W::new(self, 1)
    }
}
#[doc = "PeriBus1 permission control register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_6_SPEC;
impl crate::RegisterSpec for PRO_DPORT_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_6::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_6::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_6 to value 0"]
impl crate::Resettable for PRO_DPORT_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
