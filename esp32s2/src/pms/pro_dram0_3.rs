#[doc = "Register `PRO_DRAM0_3` reader"]
pub type R = crate::R<PRO_DRAM0_3_SPEC>;
#[doc = "Register `PRO_DRAM0_3` writer"]
pub type W = crate::W<PRO_DRAM0_3_SPEC>;
#[doc = "Field `PRO_DRAM0_ILG_CLR` reader - The clear signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_CLR_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_ILG_CLR` writer - The clear signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_ILG_EN` reader - The enable signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_EN_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_ILG_EN` writer - The enable signal for DBUS0 access interrupt."]
pub type PRO_DRAM0_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("pro_dram0_ilg_clr", &self.pro_dram0_ilg_clr())
            .field("pro_dram0_ilg_en", &self.pro_dram0_ilg_en())
            .field("pro_dram0_ilg_intr", &self.pro_dram0_ilg_intr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for DBUS0 access interrupt."]
    #[inline(always)]
    pub fn pro_dram0_ilg_clr(&mut self) -> PRO_DRAM0_ILG_CLR_W<PRO_DRAM0_3_SPEC> {
        PRO_DRAM0_ILG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for DBUS0 access interrupt."]
    #[inline(always)]
    pub fn pro_dram0_ilg_en(&mut self) -> PRO_DRAM0_ILG_EN_W<PRO_DRAM0_3_SPEC> {
        PRO_DRAM0_ILG_EN_W::new(self, 1)
    }
}
#[doc = "DBUS permission control register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dram0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dram0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DRAM0_3_SPEC;
impl crate::RegisterSpec for PRO_DRAM0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dram0_3::R`](R) reader structure"]
impl crate::Readable for PRO_DRAM0_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dram0_3::W`](W) writer structure"]
impl crate::Writable for PRO_DRAM0_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DRAM0_3 to value 0"]
impl crate::Resettable for PRO_DRAM0_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
