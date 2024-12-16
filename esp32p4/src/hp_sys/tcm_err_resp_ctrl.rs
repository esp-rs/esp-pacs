#[doc = "Register `TCM_ERR_RESP_CTRL` reader"]
pub type R = crate::R<TCM_ERR_RESP_CTRL_SPEC>;
#[doc = "Register `TCM_ERR_RESP_CTRL` writer"]
pub type W = crate::W<TCM_ERR_RESP_CTRL_SPEC>;
#[doc = "Field `TCM_ERR_RESP_EN` reader - Set 1 to turn on tcm error response"]
pub type TCM_ERR_RESP_EN_R = crate::BitReader;
#[doc = "Field `TCM_ERR_RESP_EN` writer - Set 1 to turn on tcm error response"]
pub type TCM_ERR_RESP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to turn on tcm error response"]
    #[inline(always)]
    pub fn tcm_err_resp_en(&self) -> TCM_ERR_RESP_EN_R {
        TCM_ERR_RESP_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_ERR_RESP_CTRL")
            .field("tcm_err_resp_en", &self.tcm_err_resp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to turn on tcm error response"]
    #[inline(always)]
    pub fn tcm_err_resp_en(&mut self) -> TCM_ERR_RESP_EN_W<TCM_ERR_RESP_CTRL_SPEC> {
        TCM_ERR_RESP_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_err_resp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_err_resp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_ERR_RESP_CTRL_SPEC;
impl crate::RegisterSpec for TCM_ERR_RESP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_err_resp_ctrl::R`](R) reader structure"]
impl crate::Readable for TCM_ERR_RESP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_err_resp_ctrl::W`](W) writer structure"]
impl crate::Writable for TCM_ERR_RESP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCM_ERR_RESP_CTRL to value 0"]
impl crate::Resettable for TCM_ERR_RESP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
