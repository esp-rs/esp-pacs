///Register `LP_CPU_PWR0` reader
pub type R = crate::R<LP_CPU_PWR0_SPEC>;
///Register `LP_CPU_PWR0` writer
pub type W = crate::W<LP_CPU_PWR0_SPEC>;
///Field `LP_CPU_WAITI_RDY` reader - need_des
pub type LP_CPU_WAITI_RDY_R = crate::BitReader;
///Field `LP_CPU_STALL_RDY` reader - need_des
pub type LP_CPU_STALL_RDY_R = crate::BitReader;
///Field `LP_CPU_FORCE_STALL` reader - need_des
pub type LP_CPU_FORCE_STALL_R = crate::BitReader;
///Field `LP_CPU_FORCE_STALL` writer - need_des
pub type LP_CPU_FORCE_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_CPU_SLP_WAITI_FLAG_EN` reader - need_des
pub type LP_CPU_SLP_WAITI_FLAG_EN_R = crate::BitReader;
///Field `LP_CPU_SLP_WAITI_FLAG_EN` writer - need_des
pub type LP_CPU_SLP_WAITI_FLAG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_CPU_SLP_STALL_FLAG_EN` reader - need_des
pub type LP_CPU_SLP_STALL_FLAG_EN_R = crate::BitReader;
///Field `LP_CPU_SLP_STALL_FLAG_EN` writer - need_des
pub type LP_CPU_SLP_STALL_FLAG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_CPU_SLP_STALL_WAIT` reader - need_des
pub type LP_CPU_SLP_STALL_WAIT_R = crate::FieldReader;
///Field `LP_CPU_SLP_STALL_WAIT` writer - need_des
pub type LP_CPU_SLP_STALL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LP_CPU_SLP_STALL_EN` reader - need_des
pub type LP_CPU_SLP_STALL_EN_R = crate::BitReader;
///Field `LP_CPU_SLP_STALL_EN` writer - need_des
pub type LP_CPU_SLP_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_CPU_SLP_RESET_EN` reader - need_des
pub type LP_CPU_SLP_RESET_EN_R = crate::BitReader;
///Field `LP_CPU_SLP_RESET_EN` writer - need_des
pub type LP_CPU_SLP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_CPU_SLP_BYPASS_INTR_EN` reader - need_des
pub type LP_CPU_SLP_BYPASS_INTR_EN_R = crate::BitReader;
///Field `LP_CPU_SLP_BYPASS_INTR_EN` writer - need_des
pub type LP_CPU_SLP_BYPASS_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn lp_cpu_waiti_rdy(&self) -> LP_CPU_WAITI_RDY_R {
        LP_CPU_WAITI_RDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn lp_cpu_stall_rdy(&self) -> LP_CPU_STALL_RDY_R {
        LP_CPU_STALL_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 18 - need_des
    #[inline(always)]
    pub fn lp_cpu_force_stall(&self) -> LP_CPU_FORCE_STALL_R {
        LP_CPU_FORCE_STALL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - need_des
    #[inline(always)]
    pub fn lp_cpu_slp_waiti_flag_en(&self) -> LP_CPU_SLP_WAITI_FLAG_EN_R {
        LP_CPU_SLP_WAITI_FLAG_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - need_des
    #[inline(always)]
    pub fn lp_cpu_slp_stall_flag_en(&self) -> LP_CPU_SLP_STALL_FLAG_EN_R {
        LP_CPU_SLP_STALL_FLAG_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:28 - need_des
    #[inline(always)]
    pub fn lp_cpu_slp_stall_wait(&self) -> LP_CPU_SLP_STALL_WAIT_R {
        LP_CPU_SLP_STALL_WAIT_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn lp_cpu_slp_stall_en(&self) -> LP_CPU_SLP_STALL_EN_R {
        LP_CPU_SLP_STALL_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn lp_cpu_slp_reset_en(&self) -> LP_CPU_SLP_RESET_EN_R {
        LP_CPU_SLP_RESET_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_cpu_slp_bypass_intr_en(&self) -> LP_CPU_SLP_BYPASS_INTR_EN_R {
        LP_CPU_SLP_BYPASS_INTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_PWR0")
            .field("lp_cpu_waiti_rdy", &self.lp_cpu_waiti_rdy())
            .field("lp_cpu_stall_rdy", &self.lp_cpu_stall_rdy())
            .field("lp_cpu_force_stall", &self.lp_cpu_force_stall())
            .field("lp_cpu_slp_waiti_flag_en", &self.lp_cpu_slp_waiti_flag_en())
            .field("lp_cpu_slp_stall_flag_en", &self.lp_cpu_slp_stall_flag_en())
            .field("lp_cpu_slp_stall_wait", &self.lp_cpu_slp_stall_wait())
            .field("lp_cpu_slp_stall_en", &self.lp_cpu_slp_stall_en())
            .field("lp_cpu_slp_reset_en", &self.lp_cpu_slp_reset_en())
            .field(
                "lp_cpu_slp_bypass_intr_en",
                &self.lp_cpu_slp_bypass_intr_en(),
            )
            .finish()
    }
}
impl W {
    ///Bit 18 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_force_stall(&mut self) -> LP_CPU_FORCE_STALL_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_FORCE_STALL_W::new(self, 18)
    }
    ///Bit 19 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_slp_waiti_flag_en(&mut self) -> LP_CPU_SLP_WAITI_FLAG_EN_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_SLP_WAITI_FLAG_EN_W::new(self, 19)
    }
    ///Bit 20 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_slp_stall_flag_en(&mut self) -> LP_CPU_SLP_STALL_FLAG_EN_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_SLP_STALL_FLAG_EN_W::new(self, 20)
    }
    ///Bits 21:28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_slp_stall_wait(&mut self) -> LP_CPU_SLP_STALL_WAIT_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_SLP_STALL_WAIT_W::new(self, 21)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_slp_stall_en(&mut self) -> LP_CPU_SLP_STALL_EN_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_SLP_STALL_EN_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_slp_reset_en(&mut self) -> LP_CPU_SLP_RESET_EN_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_SLP_RESET_EN_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_slp_bypass_intr_en(&mut self) -> LP_CPU_SLP_BYPASS_INTR_EN_W<LP_CPU_PWR0_SPEC> {
        LP_CPU_SLP_BYPASS_INTR_EN_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CPU_PWR0_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_cpu_pwr0::R`](R) reader structure
impl crate::Readable for LP_CPU_PWR0_SPEC {}
///`write(|w| ..)` method takes [`lp_cpu_pwr0::W`](W) writer structure
impl crate::Writable for LP_CPU_PWR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_CPU_PWR0 to value 0x1ff0_0000
impl crate::Resettable for LP_CPU_PWR0_SPEC {
    const RESET_VALUE: u32 = 0x1ff0_0000;
}
