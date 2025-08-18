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
            .field("dsi_mem_aux_ctrl", &self.dsi_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures dsi_bridge fifo memory aux ctrl"]
    #[inline(always)]
    pub fn dsi_mem_aux_ctrl(&mut self) -> DSI_MEM_AUX_CTRL_W<'_, MEM_AUX_CTRL_SPEC> {
        DSI_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "dsi_bridge mem aux control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_aux_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_aux_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for MEM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_AUX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL to value 0x1320"]
impl crate::Resettable for MEM_AUX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320;
}
