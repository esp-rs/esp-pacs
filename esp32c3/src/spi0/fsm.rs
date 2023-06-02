#[doc = "Register `FSM` reader"]
pub struct R(crate::R<FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM` writer"]
pub struct W(crate::W<FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPI_ST` reader - The current status of SPI0 slave FSM: spi0_slv_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type CSPI_ST_R = crate::FieldReader;
#[doc = "Field `EM_ST` reader - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:EM_CACHE_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
pub type EM_ST_R = crate::FieldReader;
#[doc = "Field `CSPI_LOCK_DELAY_TIME` reader - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type CSPI_LOCK_DELAY_TIME_R = crate::FieldReader;
#[doc = "Field `CSPI_LOCK_DELAY_TIME` writer - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type CSPI_LOCK_DELAY_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI0 slave FSM: spi0_slv_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn cspi_st(&self) -> CSPI_ST_R {
        CSPI_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:EM_CACHE_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
    #[inline(always)]
    pub fn em_st(&self) -> EM_ST_R {
        EM_ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn cspi_lock_delay_time(&self) -> CSPI_LOCK_DELAY_TIME_R {
        CSPI_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("cspi_st", &format_args!("{}", self.cspi_st().bits()))
            .field("em_st", &format_args!("{}", self.em_st().bits()))
            .field(
                "cspi_lock_delay_time",
                &format_args!("{}", self.cspi_lock_delay_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    #[must_use]
    pub fn cspi_lock_delay_time(&mut self) -> CSPI_LOCK_DELAY_TIME_W<7> {
        CSPI_LOCK_DELAY_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 FSM status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm](index.html) module"]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm::R](R) reader structure"]
impl crate::Readable for FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm::W](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM to value 0x0200"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
