#[doc = "Register `DPC_CONF` reader"]
pub type R = crate::R<DPC_CONF_SPEC>;
#[doc = "Register `DPC_CONF` writer"]
pub type W = crate::W<DPC_CONF_SPEC>;
#[doc = "Field `DPC_THRESHOLD_L` reader - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DPC_THRESHOLD_L_R = crate::FieldReader;
#[doc = "Field `DPC_THRESHOLD_L` writer - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DPC_THRESHOLD_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_THRESHOLD_H` reader - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DPC_THRESHOLD_H_R = crate::FieldReader;
#[doc = "Field `DPC_THRESHOLD_H` writer - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DPC_THRESHOLD_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_FACTOR_DARK` reader - this field configures the dynamic correction method 1 dark factor"]
pub type DPC_FACTOR_DARK_R = crate::FieldReader;
#[doc = "Field `DPC_FACTOR_DARK` writer - this field configures the dynamic correction method 1 dark factor"]
pub type DPC_FACTOR_DARK_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DPC_FACTOR_BRIG` reader - this field configures the dynamic correction method 1 bright factor"]
pub type DPC_FACTOR_BRIG_R = crate::FieldReader;
#[doc = "Field `DPC_FACTOR_BRIG` writer - this field configures the dynamic correction method 1 bright factor"]
pub type DPC_FACTOR_BRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_l(&self) -> DPC_THRESHOLD_L_R {
        DPC_THRESHOLD_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_h(&self) -> DPC_THRESHOLD_H_R {
        DPC_THRESHOLD_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - this field configures the dynamic correction method 1 dark factor"]
    #[inline(always)]
    pub fn dpc_factor_dark(&self) -> DPC_FACTOR_DARK_R {
        DPC_FACTOR_DARK_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - this field configures the dynamic correction method 1 bright factor"]
    #[inline(always)]
    pub fn dpc_factor_brig(&self) -> DPC_FACTOR_BRIG_R {
        DPC_FACTOR_BRIG_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPC_CONF")
            .field("dpc_threshold_l", &self.dpc_threshold_l())
            .field("dpc_threshold_h", &self.dpc_threshold_h())
            .field("dpc_factor_dark", &self.dpc_factor_dark())
            .field("dpc_factor_brig", &self.dpc_factor_brig())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_l(&mut self) -> DPC_THRESHOLD_L_W<'_, DPC_CONF_SPEC> {
        DPC_THRESHOLD_L_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_h(&mut self) -> DPC_THRESHOLD_H_W<'_, DPC_CONF_SPEC> {
        DPC_THRESHOLD_H_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - this field configures the dynamic correction method 1 dark factor"]
    #[inline(always)]
    pub fn dpc_factor_dark(&mut self) -> DPC_FACTOR_DARK_W<'_, DPC_CONF_SPEC> {
        DPC_FACTOR_DARK_W::new(self, 16)
    }
    #[doc = "Bits 22:27 - this field configures the dynamic correction method 1 bright factor"]
    #[inline(always)]
    pub fn dpc_factor_brig(&mut self) -> DPC_FACTOR_BRIG_W<'_, DPC_CONF_SPEC> {
        DPC_FACTOR_BRIG_W::new(self, 22)
    }
}
#[doc = "DPC parameter config register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPC_CONF_SPEC;
impl crate::RegisterSpec for DPC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc_conf::R`](R) reader structure"]
impl crate::Readable for DPC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpc_conf::W`](W) writer structure"]
impl crate::Writable for DPC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPC_CONF to value 0x0410_3030"]
impl crate::Resettable for DPC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0410_3030;
}
