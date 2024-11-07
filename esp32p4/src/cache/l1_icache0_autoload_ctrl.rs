#[doc = "Register `L1_ICACHE0_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<L1_ICACHE0_AUTOLOAD_CTRL_SPEC>;
#[doc = "Register `L1_ICACHE0_AUTOLOAD_CTRL` writer"]
pub type W = crate::W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-ICache0. 1: enable, 0: disable."]
pub type L1_ICACHE0_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation on L1-ICache0. 1: enable, 0: disable."]
pub type L1_ICACHE0_AUTOLOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-ICache0 is finished or not. 0: not finished. 1: finished."]
pub type L1_ICACHE0_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-ICache0. 0: ascending. 1: descending."]
pub type L1_ICACHE0_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_ORDER` writer - The bit is used to configure the direction of autoload operation on L1-ICache0. 0: ascending. 1: descending."]
pub type L1_ICACHE0_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-ICache0. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_ICACHE0_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_TRIGGER_MODE` writer - The field is used to configure trigger mode of autoload operation on L1-ICache0. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_ICACHE0_AUTOLOAD_TRIGGER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-ICache0."]
pub type L1_ICACHE0_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT0_ENA` writer - The bit is used to enable the first section for autoload operation on L1-ICache0."]
pub type L1_ICACHE0_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-ICache0."]
pub type L1_ICACHE0_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT1_ENA` writer - The bit is used to enable the second section for autoload operation on L1-ICache0."]
pub type L1_ICACHE0_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 icache0 autoload."]
pub type L1_ICACHE0_AUTOLOAD_RGID_R = crate::FieldReader;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_RGID` writer - The bit is used to set the gid of l1 icache0 autoload."]
pub type L1_ICACHE0_AUTOLOAD_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-ICache0. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l1_icache0_autoload_ena(&self) -> L1_ICACHE0_AUTOLOAD_ENA_R {
        L1_ICACHE0_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-ICache0 is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_icache0_autoload_done(&self) -> L1_ICACHE0_AUTOLOAD_DONE_R {
        L1_ICACHE0_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-ICache0. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l1_icache0_autoload_order(&self) -> L1_ICACHE0_AUTOLOAD_ORDER_R {
        L1_ICACHE0_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-ICache0. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l1_icache0_autoload_trigger_mode(&self) -> L1_ICACHE0_AUTOLOAD_TRIGGER_MODE_R {
        L1_ICACHE0_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct0_ena(&self) -> L1_ICACHE0_AUTOLOAD_SCT0_ENA_R {
        L1_ICACHE0_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct1_ena(&self) -> L1_ICACHE0_AUTOLOAD_SCT1_ENA_R {
        L1_ICACHE0_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - The bit is used to set the gid of l1 icache0 autoload."]
    #[inline(always)]
    pub fn l1_icache0_autoload_rgid(&self) -> L1_ICACHE0_AUTOLOAD_RGID_R {
        L1_ICACHE0_AUTOLOAD_RGID_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE0_AUTOLOAD_CTRL")
            .field("l1_icache0_autoload_ena", &self.l1_icache0_autoload_ena())
            .field("l1_icache0_autoload_done", &self.l1_icache0_autoload_done())
            .field(
                "l1_icache0_autoload_order",
                &self.l1_icache0_autoload_order(),
            )
            .field(
                "l1_icache0_autoload_trigger_mode",
                &self.l1_icache0_autoload_trigger_mode(),
            )
            .field(
                "l1_icache0_autoload_sct0_ena",
                &self.l1_icache0_autoload_sct0_ena(),
            )
            .field(
                "l1_icache0_autoload_sct1_ena",
                &self.l1_icache0_autoload_sct1_ena(),
            )
            .field("l1_icache0_autoload_rgid", &self.l1_icache0_autoload_rgid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-ICache0. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l1_icache0_autoload_ena(
        &mut self,
    ) -> L1_ICACHE0_AUTOLOAD_ENA_W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC> {
        L1_ICACHE0_AUTOLOAD_ENA_W::new(self, 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-ICache0. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l1_icache0_autoload_order(
        &mut self,
    ) -> L1_ICACHE0_AUTOLOAD_ORDER_W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC> {
        L1_ICACHE0_AUTOLOAD_ORDER_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-ICache0. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l1_icache0_autoload_trigger_mode(
        &mut self,
    ) -> L1_ICACHE0_AUTOLOAD_TRIGGER_MODE_W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC> {
        L1_ICACHE0_AUTOLOAD_TRIGGER_MODE_W::new(self, 3)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct0_ena(
        &mut self,
    ) -> L1_ICACHE0_AUTOLOAD_SCT0_ENA_W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC> {
        L1_ICACHE0_AUTOLOAD_SCT0_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct1_ena(
        &mut self,
    ) -> L1_ICACHE0_AUTOLOAD_SCT1_ENA_W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC> {
        L1_ICACHE0_AUTOLOAD_SCT1_ENA_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - The bit is used to set the gid of l1 icache0 autoload."]
    #[inline(always)]
    pub fn l1_icache0_autoload_rgid(
        &mut self,
    ) -> L1_ICACHE0_AUTOLOAD_RGID_W<L1_ICACHE0_AUTOLOAD_CTRL_SPEC> {
        L1_ICACHE0_AUTOLOAD_RGID_W::new(self, 10)
    }
}
#[doc = "L1 instruction Cache 0 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache0_autoload_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_icache0_autoload_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_ICACHE0_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_ICACHE0_AUTOLOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_icache0_autoload_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_ICACHE0_AUTOLOAD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_ICACHE0_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for L1_ICACHE0_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
