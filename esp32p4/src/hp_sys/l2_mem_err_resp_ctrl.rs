#[doc = "Register `L2_MEM_ERR_RESP_CTRL` reader"]
pub type R = crate::R<L2_MEM_ERR_RESP_CTRL_SPEC>;
#[doc = "Register `L2_MEM_ERR_RESP_CTRL` writer"]
pub type W = crate::W<L2_MEM_ERR_RESP_CTRL_SPEC>;
#[doc = "Field `L2_MEM_ERR_RESP_EN` reader - Set 1 to turn on l2mem error response"]
pub type L2_MEM_ERR_RESP_EN_R = crate::BitReader;
#[doc = "Field `L2_MEM_ERR_RESP_EN` writer - Set 1 to turn on l2mem error response"]
pub type L2_MEM_ERR_RESP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to turn on l2mem error response"]
    #[inline(always)]
    pub fn l2_mem_err_resp_en(&self) -> L2_MEM_ERR_RESP_EN_R {
        L2_MEM_ERR_RESP_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_ERR_RESP_CTRL")
            .field("l2_mem_err_resp_en", &self.l2_mem_err_resp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to turn on l2mem error response"]
    #[inline(always)]
    #[must_use]
    pub fn l2_mem_err_resp_en(&mut self) -> L2_MEM_ERR_RESP_EN_W<L2_MEM_ERR_RESP_CTRL_SPEC> {
        L2_MEM_ERR_RESP_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_err_resp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_err_resp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_ERR_RESP_CTRL_SPEC;
impl crate::RegisterSpec for L2_MEM_ERR_RESP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_err_resp_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_MEM_ERR_RESP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_err_resp_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_MEM_ERR_RESP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_MEM_ERR_RESP_CTRL to value 0"]
impl crate::Resettable for L2_MEM_ERR_RESP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
