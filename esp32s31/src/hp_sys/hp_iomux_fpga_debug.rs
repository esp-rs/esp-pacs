#[doc = "Register `HP_IOMUX_FPGA_DEBUG` reader"]
pub type R = crate::R<HP_IOMUX_FPGA_DEBUG_SPEC>;
#[doc = "Register `HP_IOMUX_FPGA_DEBUG` writer"]
pub type W = crate::W<HP_IOMUX_FPGA_DEBUG_SPEC>;
#[doc = "Field `HP_REG_FPGA_DEBUG` reader - iomux fpga debug"]
pub type HP_REG_FPGA_DEBUG_R = crate::BitReader;
#[doc = "Field `HP_REG_FPGA_DEBUG` writer - iomux fpga debug"]
pub type HP_REG_FPGA_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - iomux fpga debug"]
    #[inline(always)]
    pub fn hp_reg_fpga_debug(&self) -> HP_REG_FPGA_DEBUG_R {
        HP_REG_FPGA_DEBUG_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_IOMUX_FPGA_DEBUG")
            .field("hp_reg_fpga_debug", &self.hp_reg_fpga_debug())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - iomux fpga debug"]
    #[inline(always)]
    pub fn hp_reg_fpga_debug(&mut self) -> HP_REG_FPGA_DEBUG_W<'_, HP_IOMUX_FPGA_DEBUG_SPEC> {
        HP_REG_FPGA_DEBUG_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_iomux_fpga_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_iomux_fpga_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_IOMUX_FPGA_DEBUG_SPEC;
impl crate::RegisterSpec for HP_IOMUX_FPGA_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_iomux_fpga_debug::R`](R) reader structure"]
impl crate::Readable for HP_IOMUX_FPGA_DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_iomux_fpga_debug::W`](W) writer structure"]
impl crate::Writable for HP_IOMUX_FPGA_DEBUG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_IOMUX_FPGA_DEBUG to value 0x01"]
impl crate::Resettable for HP_IOMUX_FPGA_DEBUG_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
