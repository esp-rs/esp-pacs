#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SLV_EVENT` reader - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
pub type SLV_EVENT_R = crate::FieldReader;
#[doc = "Field `SLV_EVENT` writer - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
pub type SLV_EVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTDATA` reader - reserved"]
pub type EXTDATA_R = crate::BitReader;
#[doc = "Field `EXTDATA` writer - reserved"]
pub type EXTDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAPIDX` reader - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
pub type MAPIDX_R = crate::FieldReader;
#[doc = "Field `MAPIDX` writer - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
pub type MAPIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IBIDATA` reader - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
pub type IBIDATA_R = crate::FieldReader;
#[doc = "Field `IBIDATA` writer - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
pub type IBIDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PENDINT` reader - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
pub type PENDINT_R = crate::FieldReader;
#[doc = "Field `PENDINT` writer - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
pub type PENDINT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACTSTATE` reader - NA"]
pub type ACTSTATE_R = crate::FieldReader;
#[doc = "Field `ACTSTATE` writer - NA"]
pub type ACTSTATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VENDINFO` reader - NA"]
pub type VENDINFO_R = crate::FieldReader;
#[doc = "Field `VENDINFO` writer - NA"]
pub type VENDINFO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
    #[inline(always)]
    pub fn slv_event(&self) -> SLV_EVENT_R {
        SLV_EVENT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn extdata(&self) -> EXTDATA_R {
        EXTDATA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
    #[inline(always)]
    pub fn mapidx(&self) -> MAPIDX_R {
        MAPIDX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
    #[inline(always)]
    pub fn ibidata(&self) -> IBIDATA_R {
        IBIDATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
    #[inline(always)]
    pub fn pendint(&self) -> PENDINT_R {
        PENDINT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - NA"]
    #[inline(always)]
    pub fn actstate(&self) -> ACTSTATE_R {
        ACTSTATE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn vendinfo(&self) -> VENDINFO_R {
        VENDINFO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("slv_event", &self.slv_event())
            .field("extdata", &self.extdata())
            .field("mapidx", &self.mapidx())
            .field("ibidata", &self.ibidata())
            .field("pendint", &self.pendint())
            .field("actstate", &self.actstate())
            .field("vendinfo", &self.vendinfo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - If set to non-0, will request an event. Once requested, STATUS.EVENT and EVDET will show the status as it progresses. Once completed, the field will automatically return to 0. Once non-0, only 0 can be written(to cancel) until done. 0: Normal mode. If set to 0 after was a non-0 value, will cancel if not already in flight. 1: start an IBI. This will try to push through an IBI on the bus. If data associate with the IBI, it will be drawn from the IBIDATA field. Note that if Time control is enabled, this will include anytime control related bytes further, the IBIDATA byte will have bit7 set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn slv_event(&mut self) -> SLV_EVENT_W<CTRL_SPEC> {
        SLV_EVENT_W::new(self, 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn extdata(&mut self) -> EXTDATA_W<CTRL_SPEC> {
        EXTDATA_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Index of Dynamic Address that IBI is for. This is 0 for the main or base Dynamic Address, or can be any valid index."]
    #[inline(always)]
    #[must_use]
    pub fn mapidx(&mut self) -> MAPIDX_W<CTRL_SPEC> {
        MAPIDX_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data byte to go with an IBI, if enabled for it. If enabled (was in BCR), then it is required."]
    #[inline(always)]
    #[must_use]
    pub fn ibidata(&mut self) -> IBIDATA_W<CTRL_SPEC> {
        IBIDATA_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Should be set to the pending interrupt that GETSTATUS CCC will return. This should be maintained by the application if used and configured, as the Master will read this. If not configured, the GETSTATUS field will return 1 if an IBI is pending, and 0 otherwise."]
    #[inline(always)]
    #[must_use]
    pub fn pendint(&mut self) -> PENDINT_W<CTRL_SPEC> {
        PENDINT_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn actstate(&mut self) -> ACTSTATE_W<CTRL_SPEC> {
        ACTSTATE_W::new(self, 20)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vendinfo(&mut self) -> VENDINFO_W<CTRL_SPEC> {
        VENDINFO_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
