#[doc = "Register `DEBUG_SEL_CFG0` reader"]
pub type R = crate::R<DEBUG_SEL_CFG0_SPEC>;
#[doc = "Register `DEBUG_SEL_CFG0` writer"]
pub type W = crate::W<DEBUG_SEL_CFG0_SPEC>;
#[doc = "Field `DEBUG_FIELD0_SEL` reader - "]
pub type DEBUG_FIELD0_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD0_SEL` writer - "]
pub type DEBUG_FIELD0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_FIELD1_SEL` reader - "]
pub type DEBUG_FIELD1_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD1_SEL` writer - "]
pub type DEBUG_FIELD1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_FIELD2_SEL` reader - "]
pub type DEBUG_FIELD2_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD2_SEL` writer - "]
pub type DEBUG_FIELD2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_FIELD3_SEL` reader - "]
pub type DEBUG_FIELD3_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD3_SEL` writer - "]
pub type DEBUG_FIELD3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn debug_field0_sel(&self) -> DEBUG_FIELD0_SEL_R {
        DEBUG_FIELD0_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn debug_field1_sel(&self) -> DEBUG_FIELD1_SEL_R {
        DEBUG_FIELD1_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn debug_field2_sel(&self) -> DEBUG_FIELD2_SEL_R {
        DEBUG_FIELD2_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn debug_field3_sel(&self) -> DEBUG_FIELD3_SEL_R {
        DEBUG_FIELD3_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL_CFG0")
            .field("debug_field0_sel", &self.debug_field0_sel())
            .field("debug_field1_sel", &self.debug_field1_sel())
            .field("debug_field2_sel", &self.debug_field2_sel())
            .field("debug_field3_sel", &self.debug_field3_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn debug_field0_sel(&mut self) -> DEBUG_FIELD0_SEL_W<'_, DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD0_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn debug_field1_sel(&mut self) -> DEBUG_FIELD1_SEL_W<'_, DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD1_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn debug_field2_sel(&mut self) -> DEBUG_FIELD2_SEL_W<'_, DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD2_SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn debug_field3_sel(&mut self) -> DEBUG_FIELD3_SEL_W<'_, DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD3_SEL_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_sel_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SEL_CFG0_SPEC;
impl crate::RegisterSpec for DEBUG_SEL_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_sel_cfg0::R`](R) reader structure"]
impl crate::Readable for DEBUG_SEL_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_sel_cfg0::W`](W) writer structure"]
impl crate::Writable for DEBUG_SEL_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_SEL_CFG0 to value 0"]
impl crate::Resettable for DEBUG_SEL_CFG0_SPEC {}
