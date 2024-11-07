#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `FLOW_ERR` reader - x"]
pub type FLOW_ERR_R = crate::FieldReader;
#[doc = "Field `ADDR_MAP_MODE` reader - x"]
pub type ADDR_MAP_MODE_R = crate::BitReader;
#[doc = "Field `ADDR_MAP_MODE` writer - x"]
pub type ADDR_MAP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURST_LIMIT` reader - x"]
pub type BURST_LIMIT_R = crate::FieldReader;
#[doc = "Field `BURST_LIMIT` writer - x"]
pub type BURST_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOUT_THRES` reader - x"]
pub type TOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUT_THRES` writer - x"]
pub type TOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SIZE` reader - x"]
pub type SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `SIZE` writer - x"]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `START` writer - x"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_MEM` reader - x"]
pub type TO_MEM_R = crate::BitReader;
#[doc = "Field `TO_MEM` writer - x"]
pub type TO_MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA` reader - x"]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - x"]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - x"]
    #[inline(always)]
    pub fn flow_err(&self) -> FLOW_ERR_R {
        FLOW_ERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - x"]
    #[inline(always)]
    pub fn addr_map_mode(&self) -> ADDR_MAP_MODE_R {
        ADDR_MAP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - x"]
    #[inline(always)]
    pub fn burst_limit(&self) -> BURST_LIMIT_R {
        BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:18 - x"]
    #[inline(always)]
    pub fn tout_thres(&self) -> TOUT_THRES_R {
        TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 19:28 - x"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - x"]
    #[inline(always)]
    pub fn to_mem(&self) -> TO_MEM_R {
        TO_MEM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - x"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("flow_err", &self.flow_err())
            .field("addr_map_mode", &self.addr_map_mode())
            .field("burst_limit", &self.burst_limit())
            .field("tout_thres", &self.tout_thres())
            .field("size", &self.size())
            .field("to_mem", &self.to_mem())
            .field("ena", &self.ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - x"]
    #[inline(always)]
    pub fn addr_map_mode(&mut self) -> ADDR_MAP_MODE_W<CONFIG_SPEC> {
        ADDR_MAP_MODE_W::new(self, 3)
    }
    #[doc = "Bits 4:8 - x"]
    #[inline(always)]
    pub fn burst_limit(&mut self) -> BURST_LIMIT_W<CONFIG_SPEC> {
        BURST_LIMIT_W::new(self, 4)
    }
    #[doc = "Bits 9:18 - x"]
    #[inline(always)]
    pub fn tout_thres(&mut self) -> TOUT_THRES_W<CONFIG_SPEC> {
        TOUT_THRES_W::new(self, 9)
    }
    #[doc = "Bits 19:28 - x"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<CONFIG_SPEC> {
        SIZE_W::new(self, 19)
    }
    #[doc = "Bit 29 - x"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CONFIG_SPEC> {
        START_W::new(self, 29)
    }
    #[doc = "Bit 30 - x"]
    #[inline(always)]
    pub fn to_mem(&mut self) -> TO_MEM_W<CONFIG_SPEC> {
        TO_MEM_W::new(self, 30)
    }
    #[doc = "Bit 31 - x"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<CONFIG_SPEC> {
        ENA_W::new(self, 31)
    }
}
#[doc = "x\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x6480"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x6480;
}
