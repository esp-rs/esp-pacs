#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Field `IN_SEL` reader - Select a pin for input signal %s. 0-29: GPIO\\[s\\]. 0x20: high. 0x30: low."]
pub type IN_SEL_R = crate::FieldReader;
#[doc = "Field `IN_SEL` writer - Select a pin for input signal %s. 0-29: GPIO\\[s\\]. 0x20: high. 0x30: low."]
pub type IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IN_INV_SEL` reader - Invert the input value. 0: not invert. 1: invert."]
pub type IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `IN_INV_SEL` writer - Invert the input value. 0: not invert. 1: invert."]
pub type IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - Bypass GPIO matrix. 0: bypass. 1: route via GPIO matrix."]
pub type SEL_R = crate::BitReader;
#[doc = "Field `SEL` writer - Bypass GPIO matrix. 0: bypass. 1: route via GPIO matrix."]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Select a pin for input signal %s. 0-29: GPIO\\[s\\]. 0x20: high. 0x30: low."]
    #[inline(always)]
    pub fn in_sel(&self) -> IN_SEL_R {
        IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Invert the input value. 0: not invert. 1: invert."]
    #[inline(always)]
    pub fn in_inv_sel(&self) -> IN_INV_SEL_R {
        IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bypass GPIO matrix. 0: bypass. 1: route via GPIO matrix."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_IN_SEL_CFG")
            .field("in_sel", &self.in_sel())
            .field("in_inv_sel", &self.in_inv_sel())
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Select a pin for input signal %s. 0-29: GPIO\\[s\\]. 0x20: high. 0x30: low."]
    #[inline(always)]
    pub fn in_sel(&mut self) -> IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Invert the input value. 0: not invert. 1: invert."]
    #[inline(always)]
    pub fn in_inv_sel(&mut self) -> IN_INV_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        IN_INV_SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bypass GPIO matrix. 0: bypass. 1: route via GPIO matrix."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        SEL_W::new(self, 7)
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
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0x30"]
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
