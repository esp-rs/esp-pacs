#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Field `IN_INV_SEL` reader - Reserved"]
pub type IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `IN_INV_SEL` writer - Reserved"]
pub type IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG_IN_SEL` reader - Reserved"]
pub type SIG_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG_IN_SEL` writer - Reserved"]
pub type SIG_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SEL` reader - reg_gpio_func_in_sel\\[5:4\\]==2'b11->constant 1,reg_gpio_func_in_sel\\[5:4\\]==2'b10->constant 0"]
pub type IN_SEL_R = crate::FieldReader;
#[doc = "Field `IN_SEL` writer - reg_gpio_func_in_sel\\[5:4\\]==2'b11->constant 1,reg_gpio_func_in_sel\\[5:4\\]==2'b10->constant 0"]
pub type IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn in_inv_sel(&self) -> IN_INV_SEL_R {
        IN_INV_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn sig_in_sel(&self) -> SIG_IN_SEL_R {
        SIG_IN_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - reg_gpio_func_in_sel\\[5:4\\]==2'b11->constant 1,reg_gpio_func_in_sel\\[5:4\\]==2'b10->constant 0"]
    #[inline(always)]
    pub fn in_sel(&self) -> IN_SEL_R {
        IN_SEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_IN_SEL_CFG")
            .field("in_inv_sel", &self.in_inv_sel())
            .field("sig_in_sel", &self.sig_in_sel())
            .field("in_sel", &self.in_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn in_inv_sel(&mut self) -> IN_INV_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        IN_INV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn sig_in_sel(&mut self) -> SIG_IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        SIG_IN_SEL_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - reg_gpio_func_in_sel\\[5:4\\]==2'b11->constant 1,reg_gpio_func_in_sel\\[5:4\\]==2'b10->constant 0"]
    #[inline(always)]
    pub fn in_sel(&mut self) -> IN_SEL_W<'_, FUNC_IN_SEL_CFG_SPEC> {
        IN_SEL_W::new(self, 2)
    }
}
#[doc = "LP GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
