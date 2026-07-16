#[doc = "Register `QOS_DEBUG_MST_SEL` reader"]
pub type R = crate::R<QOS_DEBUG_MST_SEL_SPEC>;
#[doc = "Register `QOS_DEBUG_MST_SEL` writer"]
pub type W = crate::W<QOS_DEBUG_MST_SEL_SPEC>;
#[doc = "Field `REG_QOS_DEBUG_MST_SEL` reader - "]
pub type REG_QOS_DEBUG_MST_SEL_R = crate::FieldReader;
#[doc = "Field `REG_QOS_DEBUG_MST_SEL` writer - "]
pub type REG_QOS_DEBUG_MST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_qos_debug_mst_sel(&self) -> REG_QOS_DEBUG_MST_SEL_R {
        REG_QOS_DEBUG_MST_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QOS_DEBUG_MST_SEL")
            .field("reg_qos_debug_mst_sel", &self.reg_qos_debug_mst_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_qos_debug_mst_sel(&mut self) -> REG_QOS_DEBUG_MST_SEL_W<'_, QOS_DEBUG_MST_SEL_SPEC> {
        REG_QOS_DEBUG_MST_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`qos_debug_mst_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qos_debug_mst_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QOS_DEBUG_MST_SEL_SPEC;
impl crate::RegisterSpec for QOS_DEBUG_MST_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_debug_mst_sel::R`](R) reader structure"]
impl crate::Readable for QOS_DEBUG_MST_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qos_debug_mst_sel::W`](W) writer structure"]
impl crate::Writable for QOS_DEBUG_MST_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QOS_DEBUG_MST_SEL to value 0"]
impl crate::Resettable for QOS_DEBUG_MST_SEL_SPEC {}
