///Register `CPU_SW_STALL` reader
pub type R = crate::R<CPU_SW_STALL_SPEC>;
///Register `CPU_SW_STALL` writer
pub type W = crate::W<CPU_SW_STALL_SPEC>;
///Field `HPCORE1_SW_STALL_CODE` reader - need_des
pub type HPCORE1_SW_STALL_CODE_R = crate::FieldReader;
///Field `HPCORE1_SW_STALL_CODE` writer - need_des
pub type HPCORE1_SW_STALL_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HPCORE0_SW_STALL_CODE` reader - need_des
pub type HPCORE0_SW_STALL_CODE_R = crate::FieldReader;
///Field `HPCORE0_SW_STALL_CODE` writer - need_des
pub type HPCORE0_SW_STALL_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 16:23 - need_des
    #[inline(always)]
    pub fn hpcore1_sw_stall_code(&self) -> HPCORE1_SW_STALL_CODE_R {
        HPCORE1_SW_STALL_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - need_des
    #[inline(always)]
    pub fn hpcore0_sw_stall_code(&self) -> HPCORE0_SW_STALL_CODE_R {
        HPCORE0_SW_STALL_CODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_SW_STALL")
            .field("hpcore1_sw_stall_code", &self.hpcore1_sw_stall_code())
            .field("hpcore0_sw_stall_code", &self.hpcore0_sw_stall_code())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hpcore1_sw_stall_code(&mut self) -> HPCORE1_SW_STALL_CODE_W<CPU_SW_STALL_SPEC> {
        HPCORE1_SW_STALL_CODE_W::new(self, 16)
    }
    ///Bits 24:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hpcore0_sw_stall_code(&mut self) -> HPCORE0_SW_STALL_CODE_W<CPU_SW_STALL_SPEC> {
        HPCORE0_SW_STALL_CODE_W::new(self, 24)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_sw_stall::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_sw_stall::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_SW_STALL_SPEC;
impl crate::RegisterSpec for CPU_SW_STALL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_sw_stall::R`](R) reader structure
impl crate::Readable for CPU_SW_STALL_SPEC {}
///`write(|w| ..)` method takes [`cpu_sw_stall::W`](W) writer structure
impl crate::Writable for CPU_SW_STALL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_SW_STALL to value 0
impl crate::Resettable for CPU_SW_STALL_SPEC {
    const RESET_VALUE: u32 = 0;
}
