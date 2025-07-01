#[doc = "Register `TCM_INT_RAW` reader"]
pub type R = crate::R<TCM_INT_RAW_SPEC>;
#[doc = "Register `TCM_INT_RAW` writer"]
pub type W = crate::W<TCM_INT_RAW_SPEC>;
#[doc = "Field `TCM_PARITY_ERR_INT_RAW` reader - need_des"]
pub type TCM_PARITY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `TCM_PARITY_ERR_INT_RAW` writer - need_des"]
pub type TCM_PARITY_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_raw(&self) -> TCM_PARITY_ERR_INT_RAW_R {
        TCM_PARITY_ERR_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_INT_RAW")
            .field("tcm_parity_err_int_raw", &self.tcm_parity_err_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_raw(&mut self) -> TCM_PARITY_ERR_INT_RAW_W<TCM_INT_RAW_SPEC> {
        TCM_PARITY_ERR_INT_RAW_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_INT_RAW_SPEC;
impl crate::RegisterSpec for TCM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_int_raw::R`](R) reader structure"]
impl crate::Readable for TCM_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_int_raw::W`](W) writer structure"]
impl crate::Writable for TCM_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_INT_RAW to value 0"]
impl crate::Resettable for TCM_INT_RAW_SPEC {}
