///Register `CLOCK_GATE` reader
pub type R = crate::R<CLOCK_GATE_SPEC>;
///Register `CLOCK_GATE` writer
pub type W = crate::W<CLOCK_GATE_SPEC>;
///Field `CLK_EN` reader - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock.
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock.
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_NMI_MASK_HW` reader - This bit is used to disable all NMI interrupt signals to CPU.
pub type PRO_NMI_MASK_HW_R = crate::BitReader;
///Field `PRO_NMI_MASK_HW` writer - This bit is used to disable all NMI interrupt signals to CPU.
pub type PRO_NMI_MASK_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock.
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This bit is used to disable all NMI interrupt signals to CPU.
    #[inline(always)]
    pub fn pro_nmi_mask_hw(&self) -> PRO_NMI_MASK_HW_R {
        PRO_NMI_MASK_HW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_GATE")
            .field("clk_en", &self.clk_en())
            .field("pro_nmi_mask_hw", &self.pro_nmi_mask_hw())
            .finish()
    }
}
impl W {
    ///Bit 0 - This bit is used to enable or disable the clock of interrupt matrix. 1: enable the clock. 0: disable the clock.
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLOCK_GATE_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - This bit is used to disable all NMI interrupt signals to CPU.
    #[inline(always)]
    #[must_use]
    pub fn pro_nmi_mask_hw(&mut self) -> PRO_NMI_MASK_HW_W<CLOCK_GATE_SPEC> {
        PRO_NMI_MASK_HW_W::new(self, 1)
    }
}
/**NMI interrupt signals mask register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clock_gate::R`](R) reader structure
impl crate::Readable for CLOCK_GATE_SPEC {}
///`write(|w| ..)` method takes [`clock_gate::W`](W) writer structure
impl crate::Writable for CLOCK_GATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLOCK_GATE to value 0x01
impl crate::Resettable for CLOCK_GATE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
