#[doc = "Register `REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER` reader"]
pub type R = crate::R<REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC>;
#[doc = "Register `REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER` writer"]
pub type W = crate::W<REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC>;
#[doc = "Field `WKUPFRM_FILTER` reader - This is the address through which the application writes or reads the remote wake-up frame filter registers.The reg_wkupfrm_filter register is a pointer to eight reg_wkupfrm_filter registers.The reg_wkupfrm_filter register is loaded by sequentially loading the eight register values.Eight sequential writes to this address(0x0028)write all reg_wkupfrm_filter registers.Similarly, eight sequential reads from this address(0x0028) read all reg_wkupfrm_filter registers. This register is present only when you select the PMT module Remote Wake-Up feature in coreConsultant."]
pub type WKUPFRM_FILTER_R = crate::FieldReader<u32>;
#[doc = "Field `WKUPFRM_FILTER` writer - This is the address through which the application writes or reads the remote wake-up frame filter registers.The reg_wkupfrm_filter register is a pointer to eight reg_wkupfrm_filter registers.The reg_wkupfrm_filter register is loaded by sequentially loading the eight register values.Eight sequential writes to this address(0x0028)write all reg_wkupfrm_filter registers.Similarly, eight sequential reads from this address(0x0028) read all reg_wkupfrm_filter registers. This register is present only when you select the PMT module Remote Wake-Up feature in coreConsultant."]
pub type WKUPFRM_FILTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the address through which the application writes or reads the remote wake-up frame filter registers.The reg_wkupfrm_filter register is a pointer to eight reg_wkupfrm_filter registers.The reg_wkupfrm_filter register is loaded by sequentially loading the eight register values.Eight sequential writes to this address(0x0028)write all reg_wkupfrm_filter registers.Similarly, eight sequential reads from this address(0x0028) read all reg_wkupfrm_filter registers. This register is present only when you select the PMT module Remote Wake-Up feature in coreConsultant."]
    #[inline(always)]
    pub fn wkupfrm_filter(&self) -> WKUPFRM_FILTER_R {
        WKUPFRM_FILTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER")
            .field("wkupfrm_filter", &self.wkupfrm_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the address through which the application writes or reads the remote wake-up frame filter registers.The reg_wkupfrm_filter register is a pointer to eight reg_wkupfrm_filter registers.The reg_wkupfrm_filter register is loaded by sequentially loading the eight register values.Eight sequential writes to this address(0x0028)write all reg_wkupfrm_filter registers.Similarly, eight sequential reads from this address(0x0028) read all reg_wkupfrm_filter registers. This register is present only when you select the PMT module Remote Wake-Up feature in coreConsultant."]
    #[inline(always)]
    pub fn wkupfrm_filter(
        &mut self,
    ) -> WKUPFRM_FILTER_W<'_, REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC> {
        WKUPFRM_FILTER_W::new(self, 0)
    }
}
#[doc = "Remote Wake-Up Frame Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register10_remotewakeupframefilterregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register10_remotewakeupframefilterregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register10_remotewakeupframefilterregister::R`](R) reader structure"]
impl crate::Readable for REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register10_remotewakeupframefilterregister::W`](W) writer structure"]
impl crate::Writable for REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER to value 0"]
impl crate::Resettable for REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC {}
