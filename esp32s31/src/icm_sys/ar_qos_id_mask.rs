#[doc = "Register `AR_QOS_ID_MASK%s` reader"]
pub type R = crate::R<AR_QOS_ID_MASK_SPEC>;
#[doc = "Register `AR_QOS_ID_MASK%s` writer"]
pub type W = crate::W<AR_QOS_ID_MASK_SPEC>;
#[doc = "Field `REG_AR_QOS_ID_MASK_0` reader - "]
pub type REG_AR_QOS_ID_MASK_0_R = crate::FieldReader;
#[doc = "Field `REG_AR_QOS_ID_MASK_0` writer - "]
pub type REG_AR_QOS_ID_MASK_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_ar_qos_id_mask_0(&self) -> REG_AR_QOS_ID_MASK_0_R {
        REG_AR_QOS_ID_MASK_0_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR_QOS_ID_MASK")
            .field("reg_ar_qos_id_mask_0", &self.reg_ar_qos_id_mask_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_ar_qos_id_mask_0(&mut self) -> REG_AR_QOS_ID_MASK_0_W<'_, AR_QOS_ID_MASK_SPEC> {
        REG_AR_QOS_ID_MASK_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_id_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_id_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AR_QOS_ID_MASK_SPEC;
impl crate::RegisterSpec for AR_QOS_ID_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar_qos_id_mask::R`](R) reader structure"]
impl crate::Readable for AR_QOS_ID_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ar_qos_id_mask::W`](W) writer structure"]
impl crate::Writable for AR_QOS_ID_MASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR_QOS_ID_MASK%s to value 0x0f"]
impl crate::Resettable for AR_QOS_ID_MASK_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
