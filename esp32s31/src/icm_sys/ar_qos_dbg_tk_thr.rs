#[doc = "Register `AR_QOS_DBG_TK_THR%s` reader"]
pub type R = crate::R<AR_QOS_DBG_TK_THR_SPEC>;
#[doc = "Register `AR_QOS_DBG_TK_THR%s` writer"]
pub type W = crate::W<AR_QOS_DBG_TK_THR_SPEC>;
#[doc = "Field `REG_AR_QOS_DBG_TK_THR_0` reader - "]
pub type REG_AR_QOS_DBG_TK_THR_0_R = crate::FieldReader;
#[doc = "Field `REG_AR_QOS_DBG_TK_THR_0` writer - "]
pub type REG_AR_QOS_DBG_TK_THR_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_ar_qos_dbg_tk_thr_0(&self) -> REG_AR_QOS_DBG_TK_THR_0_R {
        REG_AR_QOS_DBG_TK_THR_0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AR_QOS_DBG_TK_THR")
            .field("reg_ar_qos_dbg_tk_thr_0", &self.reg_ar_qos_dbg_tk_thr_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_ar_qos_dbg_tk_thr_0(
        &mut self,
    ) -> REG_AR_QOS_DBG_TK_THR_0_W<'_, AR_QOS_DBG_TK_THR_SPEC> {
        REG_AR_QOS_DBG_TK_THR_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_dbg_tk_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_dbg_tk_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AR_QOS_DBG_TK_THR_SPEC;
impl crate::RegisterSpec for AR_QOS_DBG_TK_THR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar_qos_dbg_tk_thr::R`](R) reader structure"]
impl crate::Readable for AR_QOS_DBG_TK_THR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ar_qos_dbg_tk_thr::W`](W) writer structure"]
impl crate::Writable for AR_QOS_DBG_TK_THR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR_QOS_DBG_TK_THR%s to value 0x10"]
impl crate::Resettable for AR_QOS_DBG_TK_THR_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
