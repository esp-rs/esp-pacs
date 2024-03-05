#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TARGET0_INT_ENA` reader - interupt0 enable"]
pub type TARGET0_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET0_INT_ENA` writer - interupt0 enable"]
pub type TARGET0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET1_INT_ENA` reader - interupt1 enable"]
pub type TARGET1_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET1_INT_ENA` writer - interupt1 enable"]
pub type TARGET1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2_INT_ENA` reader - interupt2 enable"]
pub type TARGET2_INT_ENA_R = crate::BitReader;
#[doc = "Field `TARGET2_INT_ENA` writer - interupt2 enable"]
pub type TARGET2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    pub fn target0_int_ena(&self) -> TARGET0_INT_ENA_R {
        TARGET0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    pub fn target1_int_ena(&self) -> TARGET1_INT_ENA_R {
        TARGET1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    pub fn target2_int_ena(&self) -> TARGET2_INT_ENA_R {
        TARGET2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "target0_int_ena",
                &format_args!("{}", self.target0_int_ena().bit()),
            )
            .field(
                "target1_int_ena",
                &format_args!("{}", self.target1_int_ena().bit()),
            )
            .field(
                "target2_int_ena",
                &format_args!("{}", self.target2_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn target0_int_ena(&mut self) -> TARGET0_INT_ENA_W<INT_ENA_SPEC> {
        TARGET0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn target1_int_ena(&mut self) -> TARGET1_INT_ENA_W<INT_ENA_SPEC> {
        TARGET1_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn target2_int_ena(&mut self) -> TARGET2_INT_ENA_W<INT_ENA_SPEC> {
        TARGET2_INT_ENA_W::new(self, 2)
    }
}
#[doc = "SYSTIMER_INT_ENA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
