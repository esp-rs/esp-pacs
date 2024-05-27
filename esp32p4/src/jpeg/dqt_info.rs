#[doc = "Register `DQT_INFO` reader"]
pub type R = crate::R<DQT_INFO_SPEC>;
#[doc = "Register `DQT_INFO` writer"]
pub type W = crate::W<DQT_INFO_SPEC>;
#[doc = "Field `T0_DQT_INFO` reader - Configure dqt table0's quantization coefficient precision in bit\\[7:4\\], configure dqt table0's table id in bit\\[3:0\\]"]
pub type T0_DQT_INFO_R = crate::FieldReader;
#[doc = "Field `T0_DQT_INFO` writer - Configure dqt table0's quantization coefficient precision in bit\\[7:4\\], configure dqt table0's table id in bit\\[3:0\\]"]
pub type T0_DQT_INFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T1_DQT_INFO` reader - Configure dqt table1's quantization coefficient precision in bit\\[7:4\\], configure dqt table1's table id in bit\\[3:0\\]"]
pub type T1_DQT_INFO_R = crate::FieldReader;
#[doc = "Field `T1_DQT_INFO` writer - Configure dqt table1's quantization coefficient precision in bit\\[7:4\\], configure dqt table1's table id in bit\\[3:0\\]"]
pub type T1_DQT_INFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T2_DQT_INFO` reader - Configure dqt table2's quantization coefficient precision in bit\\[7:4\\], configure dqt table2's table id in bit\\[3:0\\]"]
pub type T2_DQT_INFO_R = crate::FieldReader;
#[doc = "Field `T2_DQT_INFO` writer - Configure dqt table2's quantization coefficient precision in bit\\[7:4\\], configure dqt table2's table id in bit\\[3:0\\]"]
pub type T2_DQT_INFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T3_DQT_INFO` reader - Configure dqt table3's quantization coefficient precision in bit\\[7:4\\], configure dqt table3's table id in bit\\[3:0\\]"]
pub type T3_DQT_INFO_R = crate::FieldReader;
#[doc = "Field `T3_DQT_INFO` writer - Configure dqt table3's quantization coefficient precision in bit\\[7:4\\], configure dqt table3's table id in bit\\[3:0\\]"]
pub type T3_DQT_INFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configure dqt table0's quantization coefficient precision in bit\\[7:4\\], configure dqt table0's table id in bit\\[3:0\\]"]
    #[inline(always)]
    pub fn t0_dqt_info(&self) -> T0_DQT_INFO_R {
        T0_DQT_INFO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configure dqt table1's quantization coefficient precision in bit\\[7:4\\], configure dqt table1's table id in bit\\[3:0\\]"]
    #[inline(always)]
    pub fn t1_dqt_info(&self) -> T1_DQT_INFO_R {
        T1_DQT_INFO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configure dqt table2's quantization coefficient precision in bit\\[7:4\\], configure dqt table2's table id in bit\\[3:0\\]"]
    #[inline(always)]
    pub fn t2_dqt_info(&self) -> T2_DQT_INFO_R {
        T2_DQT_INFO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Configure dqt table3's quantization coefficient precision in bit\\[7:4\\], configure dqt table3's table id in bit\\[3:0\\]"]
    #[inline(always)]
    pub fn t3_dqt_info(&self) -> T3_DQT_INFO_R {
        T3_DQT_INFO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DQT_INFO")
            .field("t0_dqt_info", &self.t0_dqt_info())
            .field("t1_dqt_info", &self.t1_dqt_info())
            .field("t2_dqt_info", &self.t2_dqt_info())
            .field("t3_dqt_info", &self.t3_dqt_info())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure dqt table0's quantization coefficient precision in bit\\[7:4\\], configure dqt table0's table id in bit\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn t0_dqt_info(&mut self) -> T0_DQT_INFO_W<DQT_INFO_SPEC> {
        T0_DQT_INFO_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configure dqt table1's quantization coefficient precision in bit\\[7:4\\], configure dqt table1's table id in bit\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn t1_dqt_info(&mut self) -> T1_DQT_INFO_W<DQT_INFO_SPEC> {
        T1_DQT_INFO_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configure dqt table2's quantization coefficient precision in bit\\[7:4\\], configure dqt table2's table id in bit\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn t2_dqt_info(&mut self) -> T2_DQT_INFO_W<DQT_INFO_SPEC> {
        T2_DQT_INFO_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Configure dqt table3's quantization coefficient precision in bit\\[7:4\\], configure dqt table3's table id in bit\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn t3_dqt_info(&mut self) -> T3_DQT_INFO_W<DQT_INFO_SPEC> {
        T3_DQT_INFO_W::new(self, 24)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dqt_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dqt_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DQT_INFO_SPEC;
impl crate::RegisterSpec for DQT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dqt_info::R`](R) reader structure"]
impl crate::Readable for DQT_INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dqt_info::W`](W) writer structure"]
impl crate::Writable for DQT_INFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DQT_INFO to value 0x0302_0100"]
impl crate::Resettable for DQT_INFO_SPEC {
    const RESET_VALUE: u32 = 0x0302_0100;
}
