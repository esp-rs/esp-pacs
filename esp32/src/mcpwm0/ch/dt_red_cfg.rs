#[doc = "Register `DT_RED_CFG` reader"]
pub type R = crate::R<DT_RED_CFG_SPEC>;
#[doc = "Register `DT_RED_CFG` writer"]
pub type W = crate::W<DT_RED_CFG_SPEC>;
#[doc = "Field `RED` reader - "]
pub type RED_R = crate::FieldReader<u16>;
#[doc = "Field `RED` writer - "]
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT_RED_CFG")
            .field("red", &self.red())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<DT_RED_CFG_SPEC> {
        RED_W::new(self, 0)
    }
}
#[doc = "Shadow register for rising edge delay (RED).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_red_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_red_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_RED_CFG_SPEC;
impl crate::RegisterSpec for DT_RED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_red_cfg::R`](R) reader structure"]
impl crate::Readable for DT_RED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt_red_cfg::W`](W) writer structure"]
impl crate::Writable for DT_RED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT_RED_CFG to value 0"]
impl crate::Resettable for DT_RED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
