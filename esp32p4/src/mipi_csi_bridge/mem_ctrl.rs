///Register `MEM_CTRL` reader
pub type R = crate::R<MEM_CTRL_SPEC>;
///Register `MEM_CTRL` writer
pub type W = crate::W<MEM_CTRL_SPEC>;
///Field `CSI_BRIDGE_MEM_CLK_FORCE_ON` reader - csi bridge memory clock gating force on.
pub type CSI_BRIDGE_MEM_CLK_FORCE_ON_R = crate::BitReader;
///Field `CSI_BRIDGE_MEM_CLK_FORCE_ON` writer - csi bridge memory clock gating force on.
pub type CSI_BRIDGE_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSI_MEM_AUX_CTRL` reader - N/A
pub type CSI_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
///Field `CSI_MEM_AUX_CTRL` writer - N/A
pub type CSI_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0 - csi bridge memory clock gating force on.
    #[inline(always)]
    pub fn csi_bridge_mem_clk_force_on(&self) -> CSI_BRIDGE_MEM_CLK_FORCE_ON_R {
        CSI_BRIDGE_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:14 - N/A
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
                &self.csi_bridge_mem_clk_force_on(),
            )
            .field("csi_mem_aux_ctrl", &self.csi_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    ///Bit 0 - csi bridge memory clock gating force on.
    #[inline(always)]
    #[must_use]
    pub fn csi_bridge_mem_clk_force_on(&mut self) -> CSI_BRIDGE_MEM_CLK_FORCE_ON_W<MEM_CTRL_SPEC> {
        CSI_BRIDGE_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
    ///Bits 1:14 - N/A
    #[inline(always)]
    #[must_use]
    pub fn csi_mem_aux_ctrl(&mut self) -> CSI_MEM_AUX_CTRL_W<MEM_CTRL_SPEC> {
        CSI_MEM_AUX_CTRL_W::new(self, 1)
    }
}
/**csi bridge buffer control.

You can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_ctrl::R`](R) reader structure
impl crate::Readable for MEM_CTRL_SPEC {}
///`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure
impl crate::Writable for MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_CTRL to value 0x2640
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2640;
}
