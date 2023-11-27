#[doc = "Register `MEM_AUX_CTRL` reader"]
pub type R = crate::R<MEM_AUX_CTRL_SPEC>;
#[doc = "Register `MEM_AUX_CTRL` writer"]
pub type W = crate::W<MEM_AUX_CTRL_SPEC>;
#[doc = "Field `DSI_MEM_AUX_CTRL` reader - this field configures dsi_bridge fifo memory aux ctrl"]
pub type DSI_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `DSI_MEM_AUX_CTRL` writer - this field configures dsi_bridge fifo memory aux ctrl"]
pub type DSI_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures dsi_bridge fifo memory aux ctrl"]
    #[inline(always)]
    pub fn dsi_mem_aux_ctrl(&self) -> DSI_MEM_AUX_CTRL_R {
        DSI_MEM_AUX_CTRL_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_AUX_CTRL")
            .field(
                "dsi_mem_aux_ctrl",
                &format_args!("{}", self.dsi_mem_aux_ctrl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_AUX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures dsi_bridge fifo memory aux ctrl"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_mem_aux_ctrl(&mut self) -> DSI_MEM_AUX_CTRL_W<MEM_AUX_CTRL_SPEC> {
        DSI_MEM_AUX_CTRL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "dsi_bridge mem aux control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for MEM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_AUX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL to value 0x1320"]
impl crate::Resettable for MEM_AUX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1320;
}
