///Register `PRO_CPU_RECORD_CTRL` reader
pub type R = crate::R<PRO_CPU_RECORD_CTRL_SPEC>;
///Register `PRO_CPU_RECORD_CTRL` writer
pub type W = crate::W<PRO_CPU_RECORD_CTRL_SPEC>;
///Field `PRO_CPU_RECORD_ENABLE` reader -
pub type PRO_CPU_RECORD_ENABLE_R = crate::BitReader;
///Field `PRO_CPU_RECORD_ENABLE` writer -
pub type PRO_CPU_RECORD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CPU_RECORD_DISABLE` reader -
pub type PRO_CPU_RECORD_DISABLE_R = crate::BitReader;
///Field `PRO_CPU_RECORD_DISABLE` writer -
pub type PRO_CPU_RECORD_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_CPU_PDEBUG_ENABLE` reader -
pub type PRO_CPU_PDEBUG_ENABLE_R = crate::BitReader;
///Field `PRO_CPU_PDEBUG_ENABLE` writer -
pub type PRO_CPU_PDEBUG_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn pro_cpu_record_enable(&self) -> PRO_CPU_RECORD_ENABLE_R {
        PRO_CPU_RECORD_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn pro_cpu_record_disable(&self) -> PRO_CPU_RECORD_DISABLE_R {
        PRO_CPU_RECORD_DISABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn pro_cpu_pdebug_enable(&self) -> PRO_CPU_PDEBUG_ENABLE_R {
        PRO_CPU_PDEBUG_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_CTRL")
            .field("pro_cpu_record_enable", &self.pro_cpu_record_enable())
            .field("pro_cpu_record_disable", &self.pro_cpu_record_disable())
            .field("pro_cpu_pdebug_enable", &self.pro_cpu_pdebug_enable())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_record_enable(&mut self) -> PRO_CPU_RECORD_ENABLE_W<PRO_CPU_RECORD_CTRL_SPEC> {
        PRO_CPU_RECORD_ENABLE_W::new(self, 0)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_record_disable(&mut self) -> PRO_CPU_RECORD_DISABLE_W<PRO_CPU_RECORD_CTRL_SPEC> {
        PRO_CPU_RECORD_DISABLE_W::new(self, 4)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn pro_cpu_pdebug_enable(&mut self) -> PRO_CPU_PDEBUG_ENABLE_W<PRO_CPU_RECORD_CTRL_SPEC> {
        PRO_CPU_PDEBUG_ENABLE_W::new(self, 8)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_CPU_RECORD_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_cpu_record_ctrl::R`](R) reader structure
impl crate::Readable for PRO_CPU_RECORD_CTRL_SPEC {}
///`write(|w| ..)` method takes [`pro_cpu_record_ctrl::W`](W) writer structure
impl crate::Writable for PRO_CPU_RECORD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_CPU_RECORD_CTRL to value 0x0100
impl crate::Resettable for PRO_CPU_RECORD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
