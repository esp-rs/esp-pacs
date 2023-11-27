#[doc = "Register `MEM_CTRL` reader"]
pub type R = crate::R<MEM_CTRL_SPEC>;
#[doc = "Register `MEM_CTRL` writer"]
pub type W = crate::W<MEM_CTRL_SPEC>;
#[doc = "Field `CSI_BRIDGE_MEM_CLK_FORCE_ON` reader - csi bridge memory clock gating force on."]
pub type CSI_BRIDGE_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CSI_BRIDGE_MEM_CLK_FORCE_ON` writer - csi bridge memory clock gating force on."]
pub type CSI_BRIDGE_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_MEM_AUX_CTRL` reader - N/A"]
pub type CSI_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `CSI_MEM_AUX_CTRL` writer - N/A"]
pub type CSI_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - csi bridge memory clock gating force on."]
    #[inline(always)]
    pub fn csi_bridge_mem_clk_force_on(&self) -> CSI_BRIDGE_MEM_CLK_FORCE_ON_R {
        CSI_BRIDGE_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - N/A"]
    #[inline(always)]
    pub fn csi_mem_aux_ctrl(&self) -> CSI_MEM_AUX_CTRL_R {
        CSI_MEM_AUX_CTRL_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CTRL")
            .field(
                "csi_bridge_mem_clk_force_on",
                &format_args!("{}", self.csi_bridge_mem_clk_force_on().bit()),
            )
            .field(
                "csi_mem_aux_ctrl",
                &format_args!("{}", self.csi_mem_aux_ctrl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - csi bridge memory clock gating force on."]
    #[inline(always)]
    #[must_use]
    pub fn csi_bridge_mem_clk_force_on(&mut self) -> CSI_BRIDGE_MEM_CLK_FORCE_ON_W<MEM_CTRL_SPEC> {
        CSI_BRIDGE_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bits 1:14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn csi_mem_aux_ctrl(&mut self) -> CSI_MEM_AUX_CTRL_W<MEM_CTRL_SPEC> {
        CSI_MEM_AUX_CTRL_W::new(self, 1)
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
#[doc = "csi bridge buffer control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0x2640"]
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2640;
}
