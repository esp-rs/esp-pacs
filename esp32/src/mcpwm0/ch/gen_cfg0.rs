#[doc = "Register `GEN_CFG0` reader"]
pub type R = crate::R<GEN_CFG0_SPEC>;
#[doc = "Register `GEN_CFG0` writer"]
pub type W = crate::W<GEN_CFG0_SPEC>;
#[doc = "Field `CFG_UPMETHOD` reader - "]
pub type CFG_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CFG_UPMETHOD` writer - "]
pub type CFG_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T0_SEL` reader - "]
pub type T0_SEL_R = crate::FieldReader;
#[doc = "Field `T0_SEL` writer - "]
pub type T0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `T1_SEL` reader - "]
pub type T1_SEL_R = crate::FieldReader;
#[doc = "Field `T1_SEL` writer - "]
pub type T1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cfg_upmethod(&self) -> CFG_UPMETHOD_R {
        CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn t0_sel(&self) -> T0_SEL_R {
        T0_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn t1_sel(&self) -> T1_SEL_R {
        T1_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_CFG0")
            .field("cfg_upmethod", &self.cfg_upmethod())
            .field("t0_sel", &self.t0_sel())
            .field("t1_sel", &self.t1_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_upmethod(&mut self) -> CFG_UPMETHOD_W<GEN_CFG0_SPEC> {
        CFG_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn t0_sel(&mut self) -> T0_SEL_W<GEN_CFG0_SPEC> {
        T0_SEL_W::new(self, 4)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn t1_sel(&mut self) -> T1_SEL_W<GEN_CFG0_SPEC> {
        T1_SEL_W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_CFG0_SPEC;
impl crate::RegisterSpec for GEN_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_cfg0::R`](R) reader structure"]
impl crate::Readable for GEN_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_cfg0::W`](W) writer structure"]
impl crate::Writable for GEN_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_CFG0 to value 0"]
impl crate::Resettable for GEN_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
