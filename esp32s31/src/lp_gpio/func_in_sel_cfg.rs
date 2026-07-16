#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Field `FUNC_IN_SEL` reader - Configures to select a pin from the 8 LP_GPIO pins to connect the input signal %s.\\\\ 0: Select LP_GPIO0\\\\ 1: Select LP_GPIO1\\\\ ......\\\\ 6: Select LP_GPIO6\\\\ 7: Select LP_GPIO7\\\\ Or\\\\ 0x8: A constantly high input\\\\ 0xc: A constantly low input\\\\"]
pub type FUNC_IN_SEL_R = crate::FieldReader;
#[doc = "Field `FUNC_IN_SEL` writer - Configures to select a pin from the 8 LP_GPIO pins to connect the input signal %s.\\\\ 0: Select LP_GPIO0\\\\ 1: Select LP_GPIO1\\\\ ......\\\\ 6: Select LP_GPIO6\\\\ 7: Select LP_GPIO7\\\\ Or\\\\ 0x8: A constantly high input\\\\ 0xc: A constantly low input\\\\"]
pub type FUNC_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FUNC_IN_INV_SEL` reader - Configures whether or not to invert the input value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC_IN_INV_SEL` writer - Configures whether or not to invert the input value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type FUNC_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG_IN_SEL` reader - Configures whether or not to route signals via LP_GPIO matrix.\\\\ 0: Bypass LP_GPIO matrix, i.e., connect signals directly to peripheral configured in IO MUX.\\\\ 1: Route signals via LP_GPIO matrix.\\\\"]
pub type SIG_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG_IN_SEL` writer - Configures whether or not to route signals via LP_GPIO matrix.\\\\ 0: Bypass LP_GPIO matrix, i.e., connect signals directly to peripheral configured in IO MUX.\\\\ 1: Route signals via LP_GPIO matrix.\\\\"]
pub type SIG_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Configures to select a pin from the 8 LP_GPIO pins to connect the input signal %s.\\\\ 0: Select LP_GPIO0\\\\ 1: Select LP_GPIO1\\\\ ......\\\\ 6: Select LP_GPIO6\\\\ 7: Select LP_GPIO7\\\\ Or\\\\ 0x8: A constantly high input\\\\ 0xc: A constantly low input\\\\"]
    #[inline(always)]
    pub fn func_in_sel(&self) -> FUNC_IN_SEL_R {
        FUNC_IN_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Configures whether or not to invert the input value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func_in_inv_sel(&self) -> FUNC_IN_INV_SEL_R {
        FUNC_IN_INV_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to route signals via LP_GPIO matrix.\\\\ 0: Bypass LP_GPIO matrix, i.e., connect signals directly to peripheral configured in IO MUX.\\\\ 1: Route signals via LP_GPIO matrix.\\\\"]
    #[inline(always)]
    pub fn sig_in_sel(&self) -> SIG_IN_SEL_R {
        SIG_IN_SEL_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bits 0:3 - Configures to select a pin from the 8 LP_GPIO pins to connect the input signal %s.\\\\ 0: Select LP_GPIO0\\\\ 1: Select LP_GPIO1\\\\ ......\\\\ 6: Select LP_GPIO6\\\\ 7: Select LP_GPIO7\\\\ Or\\\\ 0x8: A constantly high input\\\\ 0xc: A constantly low input\\\\"]
    #[inline(always)]
    pub fn func_in_sel(&mut self) -> FUNC_IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        FUNC_IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Configures whether or not to invert the input value.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn func_in_inv_sel(&mut self) -> FUNC_IN_INV_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        FUNC_IN_INV_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to route signals via LP_GPIO matrix.\\\\ 0: Bypass LP_GPIO matrix, i.e., connect signals directly to peripheral configured in IO MUX.\\\\ 1: Route signals via LP_GPIO matrix.\\\\"]
    #[inline(always)]
    pub fn sig_in_sel(&mut self) -> SIG_IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        SIG_IN_SEL_W::new(self, 5)
    }
}
#[doc = "Configuration register for input signal %s\n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0x08"]
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
