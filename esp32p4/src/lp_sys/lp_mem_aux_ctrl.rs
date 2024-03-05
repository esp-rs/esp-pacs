#[doc = "Register `LP_MEM_AUX_CTRL` reader"]
pub type R = crate::R<LP_MEM_AUX_CTRL_SPEC>;
#[doc = "Register `LP_MEM_AUX_CTRL` writer"]
pub type W = crate::W<LP_MEM_AUX_CTRL_SPEC>;
#[doc = "Field `LP_MEM_AUX_CTRL` reader - need_des"]
pub type LP_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `LP_MEM_AUX_CTRL` writer - need_des"]
pub type LP_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_mem_aux_ctrl(&self) -> LP_MEM_AUX_CTRL_R {
        LP_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_MEM_AUX_CTRL")
            .field(
                "lp_mem_aux_ctrl",
                &format_args!("{}", self.lp_mem_aux_ctrl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_MEM_AUX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mem_aux_ctrl(&mut self) -> LP_MEM_AUX_CTRL_W<LP_MEM_AUX_CTRL_SPEC> {
        LP_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_mem_aux_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_mem_aux_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_MEM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for LP_MEM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_mem_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_MEM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_mem_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_MEM_AUX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_MEM_AUX_CTRL to value 0x2070"]
impl crate::Resettable for LP_MEM_AUX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2070;
}
