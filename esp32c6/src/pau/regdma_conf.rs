///Register `REGDMA_CONF` reader
pub type R = crate::R<REGDMA_CONF_SPEC>;
///Register `REGDMA_CONF` writer
pub type W = crate::W<REGDMA_CONF_SPEC>;
///Field `FLOW_ERR` reader - backup error type
pub type FLOW_ERR_R = crate::FieldReader;
///Field `START` writer - backup start signal
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TO_MEM` reader - backup direction(reg to mem / mem to reg)
pub type TO_MEM_R = crate::BitReader;
///Field `TO_MEM` writer - backup direction(reg to mem / mem to reg)
pub type TO_MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINK_SEL` reader - Link select
pub type LINK_SEL_R = crate::FieldReader;
///Field `LINK_SEL` writer - Link select
pub type LINK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `START_MAC` writer - mac sw backup start signal
pub type START_MAC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TO_MEM_MAC` reader - mac sw backup direction(reg to mem / mem to reg)
pub type TO_MEM_MAC_R = crate::BitReader;
///Field `TO_MEM_MAC` writer - mac sw backup direction(reg to mem / mem to reg)
pub type TO_MEM_MAC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEL_MAC` reader - mac hw/sw select
pub type SEL_MAC_R = crate::BitReader;
///Field `SEL_MAC` writer - mac hw/sw select
pub type SEL_MAC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - backup error type
    #[inline(always)]
    pub fn flow_err(&self) -> FLOW_ERR_R {
        FLOW_ERR_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub fn to_mem(&self) -> TO_MEM_R {
        TO_MEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Link select
    #[inline(always)]
    pub fn link_sel(&self) -> LINK_SEL_R {
        LINK_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 8 - mac sw backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub fn to_mem_mac(&self) -> TO_MEM_MAC_R {
        TO_MEM_MAC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mac hw/sw select
    #[inline(always)]
    pub fn sel_mac(&self) -> SEL_MAC_R {
        SEL_MAC_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CONF")
            .field("flow_err", &self.flow_err())
            .field("to_mem", &self.to_mem())
            .field("link_sel", &self.link_sel())
            .field("to_mem_mac", &self.to_mem_mac())
            .field("sel_mac", &self.sel_mac())
            .finish()
    }
}
impl W {
    ///Bit 3 - backup start signal
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<REGDMA_CONF_SPEC> {
        START_W::new(self, 3)
    }
    ///Bit 4 - backup direction(reg to mem / mem to reg)
    #[inline(always)]
    #[must_use]
    pub fn to_mem(&mut self) -> TO_MEM_W<REGDMA_CONF_SPEC> {
        TO_MEM_W::new(self, 4)
    }
    ///Bits 5:6 - Link select
    #[inline(always)]
    #[must_use]
    pub fn link_sel(&mut self) -> LINK_SEL_W<REGDMA_CONF_SPEC> {
        LINK_SEL_W::new(self, 5)
    }
    ///Bit 7 - mac sw backup start signal
    #[inline(always)]
    #[must_use]
    pub fn start_mac(&mut self) -> START_MAC_W<REGDMA_CONF_SPEC> {
        START_MAC_W::new(self, 7)
    }
    ///Bit 8 - mac sw backup direction(reg to mem / mem to reg)
    #[inline(always)]
    #[must_use]
    pub fn to_mem_mac(&mut self) -> TO_MEM_MAC_W<REGDMA_CONF_SPEC> {
        TO_MEM_MAC_W::new(self, 8)
    }
    ///Bit 9 - mac hw/sw select
    #[inline(always)]
    #[must_use]
    pub fn sel_mac(&mut self) -> SEL_MAC_W<REGDMA_CONF_SPEC> {
        SEL_MAC_W::new(self, 9)
    }
}
/**Peri backup control register

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGDMA_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`regdma_conf::R`](R) reader structure
impl crate::Readable for REGDMA_CONF_SPEC {}
///`write(|w| ..)` method takes [`regdma_conf::W`](W) writer structure
impl crate::Writable for REGDMA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGDMA_CONF to value 0
impl crate::Resettable for REGDMA_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
