#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Field `FUNC_IN_SEL` reader - Configures to select a pin from the 63 GPIO pins to connect the input signal %s."]
pub type FUNC_IN_SEL_R = crate::FieldReader;
#[doc = "Field `FUNC_IN_SEL` writer - Configures to select a pin from the 63 GPIO pins to connect the input signal %s."]
pub type FUNC_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FUNC_IN_INV_SEL` reader - Configures whether or not to invert the input value. 0: Not invert. 1: Invert."]
pub type FUNC_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_IN_INV_SEL` writer - Configures whether or not to invert the input value. 0: Not invert. 1: Invert."]
pub type FUNC_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG_IN_SEL` reader - Configures whether or not to route signals via GPIO matrix. 0: Bypass GPIO matrix. 1: Route via GPIO matrix."]
pub type SIG_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG_IN_SEL` writer - Configures whether or not to route signals via GPIO matrix. 0: Bypass GPIO matrix. 1: Route via GPIO matrix."]
pub type SIG_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configures to select a pin from the 63 GPIO pins to connect the input signal %s."]
    #[inline(always)]
    pub fn func_in_sel(&self) -> FUNC_IN_SEL_R {
        FUNC_IN_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Configures whether or not to invert the input value. 0: Not invert. 1: Invert."]
    #[inline(always)]
    pub fn func_in_inv_sel(&self) -> FUNC_IN_INV_SEL_R {
        FUNC_IN_INV_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to route signals via GPIO matrix. 0: Bypass GPIO matrix. 1: Route via GPIO matrix."]
    #[inline(always)]
    pub fn sig_in_sel(&self) -> SIG_IN_SEL_R {
        SIG_IN_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_IN_SEL_CFG")
            .field("func_in_sel", &self.func_in_sel())
            .field("func_in_inv_sel", &self.func_in_inv_sel())
            .field("sig_in_sel", &self.sig_in_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures to select a pin from the 63 GPIO pins to connect the input signal %s."]
    #[inline(always)]
    pub fn func_in_sel(&mut self) -> FUNC_IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        FUNC_IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Configures whether or not to invert the input value. 0: Not invert. 1: Invert."]
    #[inline(always)]
    pub fn func_in_inv_sel(&mut self) -> FUNC_IN_INV_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        FUNC_IN_INV_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to route signals via GPIO matrix. 0: Bypass GPIO matrix. 1: Route via GPIO matrix."]
    #[inline(always)]
    pub fn sig_in_sel(&mut self) -> SIG_IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        SIG_IN_SEL_W::new(self, 9)
    }
}
#[doc = "GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_IN_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {}
