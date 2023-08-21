#[doc = "Register `PRO_DRAM0_3` reader"]
pub type R = crate::R<PRO_DRAM0_3_SPEC>;
#[doc = "Register `PRO_DRAM0_3` writer"]
pub type W = crate::W<PRO_DRAM0_3_SPEC>;
#[doc = "Field `PRO_DRAM0_ILG_CLR` reader - The clear signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_CLR_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_ILG_CLR` writer - The clear signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_DRAM0_ILG_EN` reader - The enable signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_EN_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_ILG_EN` writer - The enable signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_DRAM0_ILG_INTR` reader - DBUS0 access interrupt signal."]
pub type PRO_DRAM0_ILG_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for DBUS0 access interrupt."]
    #[inline(always)]
    pub fn pro_dram0_ilg_clr(&self) -> PRO_DRAM0_ILG_CLR_R {
        PRO_DRAM0_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for DBUS0 access interrupt."]
    #[inline(always)]
    pub fn pro_dram0_ilg_en(&self) -> PRO_DRAM0_ILG_EN_R {
        PRO_DRAM0_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBUS0 access interrupt signal."]
    #[inline(always)]
    pub fn pro_dram0_ilg_intr(&self) -> PRO_DRAM0_ILG_INTR_R {
        PRO_DRAM0_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DRAM0_3")
            .field(
                "pro_dram0_ilg_clr",
                &format_args!("{}", self.pro_dram0_ilg_clr().bit()),
            )
            .field(
                "pro_dram0_ilg_en",
                &format_args!("{}", self.pro_dram0_ilg_en().bit()),
            )
            .field(
                "pro_dram0_ilg_intr",
                &format_args!("{}", self.pro_dram0_ilg_intr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DRAM0_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for DBUS0 access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_ilg_clr(&mut self) -> PRO_DRAM0_ILG_CLR_W<PRO_DRAM0_3_SPEC, 0> {
        PRO_DRAM0_ILG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The enable signal for DBUS0 access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_ilg_en(&mut self) -> PRO_DRAM0_ILG_EN_W<PRO_DRAM0_3_SPEC, 1> {
        PRO_DRAM0_ILG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DBUS permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dram0_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dram0_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DRAM0_3_SPEC;
impl crate::RegisterSpec for PRO_DRAM0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dram0_3::R`](R) reader structure"]
impl crate::Readable for PRO_DRAM0_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dram0_3::W`](W) writer structure"]
impl crate::Writable for PRO_DRAM0_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DRAM0_3 to value 0"]
impl crate::Resettable for PRO_DRAM0_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
