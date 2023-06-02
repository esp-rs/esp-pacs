#[doc = "Register `LPBUS` reader"]
pub struct R(crate::R<LPBUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPBUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPBUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPBUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPBUS` writer"]
pub struct W(crate::W<LPBUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPBUS_SPEC>;
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
impl From<crate::W<LPBUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPBUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAST_MEM_WPULSE` reader - This field controls fast memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
pub type FAST_MEM_WPULSE_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_WPULSE` writer - This field controls fast memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
pub type FAST_MEM_WPULSE_W<'a, const O: u8> = crate::FieldWriter<'a, LPBUS_SPEC, 3, O>;
#[doc = "Field `FAST_MEM_WA` reader - This field controls fast memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
pub type FAST_MEM_WA_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_WA` writer - This field controls fast memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
pub type FAST_MEM_WA_W<'a, const O: u8> = crate::FieldWriter<'a, LPBUS_SPEC, 3, O>;
#[doc = "Field `FAST_MEM_RA` reader - This field controls fast memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
pub type FAST_MEM_RA_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_RA` writer - This field controls fast memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
pub type FAST_MEM_RA_W<'a, const O: u8> = crate::FieldWriter<'a, LPBUS_SPEC, 2, O>;
#[doc = "Field `FAST_MEM_RM` reader - This field controls fast memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
pub type FAST_MEM_RM_R = crate::FieldReader;
#[doc = "Field `FAST_MEM_RM` writer - This field controls fast memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
pub type FAST_MEM_RM_W<'a, const O: u8> = crate::FieldWriter<'a, LPBUS_SPEC, 4, O>;
#[doc = "Field `FAST_MEM_MUX_FSM_IDLE` reader - reserved"]
pub type FAST_MEM_MUX_FSM_IDLE_R = crate::BitReader;
#[doc = "Field `FAST_MEM_MUX_SEL_STATUS` reader - reserved"]
pub type FAST_MEM_MUX_SEL_STATUS_R = crate::BitReader;
#[doc = "Field `FAST_MEM_MUX_SEL_UPDATE` reader - reserved"]
pub type FAST_MEM_MUX_SEL_UPDATE_R = crate::BitReader;
#[doc = "Field `FAST_MEM_MUX_SEL` reader - reserved"]
pub type FAST_MEM_MUX_SEL_R = crate::BitReader;
impl R {
    #[doc = "Bits 16:18 - This field controls fast memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
    #[inline(always)]
    pub fn fast_mem_wpulse(&self) -> FAST_MEM_WPULSE_R {
        FAST_MEM_WPULSE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - This field controls fast memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
    #[inline(always)]
    pub fn fast_mem_wa(&self) -> FAST_MEM_WA_R {
        FAST_MEM_WA_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - This field controls fast memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
    #[inline(always)]
    pub fn fast_mem_ra(&self) -> FAST_MEM_RA_R {
        FAST_MEM_RA_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - This field controls fast memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
    #[inline(always)]
    pub fn fast_mem_rm(&self) -> FAST_MEM_RM_R {
        FAST_MEM_RM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn fast_mem_mux_fsm_idle(&self) -> FAST_MEM_MUX_FSM_IDLE_R {
        FAST_MEM_MUX_FSM_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn fast_mem_mux_sel_status(&self) -> FAST_MEM_MUX_SEL_STATUS_R {
        FAST_MEM_MUX_SEL_STATUS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn fast_mem_mux_sel_update(&self) -> FAST_MEM_MUX_SEL_UPDATE_R {
        FAST_MEM_MUX_SEL_UPDATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn fast_mem_mux_sel(&self) -> FAST_MEM_MUX_SEL_R {
        FAST_MEM_MUX_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPBUS")
            .field(
                "fast_mem_wpulse",
                &format_args!("{}", self.fast_mem_wpulse().bits()),
            )
            .field(
                "fast_mem_wa",
                &format_args!("{}", self.fast_mem_wa().bits()),
            )
            .field(
                "fast_mem_ra",
                &format_args!("{}", self.fast_mem_ra().bits()),
            )
            .field(
                "fast_mem_rm",
                &format_args!("{}", self.fast_mem_rm().bits()),
            )
            .field(
                "fast_mem_mux_fsm_idle",
                &format_args!("{}", self.fast_mem_mux_fsm_idle().bit()),
            )
            .field(
                "fast_mem_mux_sel_status",
                &format_args!("{}", self.fast_mem_mux_sel_status().bit()),
            )
            .field(
                "fast_mem_mux_sel_update",
                &format_args!("{}", self.fast_mem_mux_sel_update().bit()),
            )
            .field(
                "fast_mem_mux_sel",
                &format_args!("{}", self.fast_mem_mux_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPBUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:18 - This field controls fast memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
    #[inline(always)]
    #[must_use]
    pub fn fast_mem_wpulse(&mut self) -> FAST_MEM_WPULSE_W<16> {
        FAST_MEM_WPULSE_W::new(self)
    }
    #[doc = "Bits 19:21 - This field controls fast memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
    #[inline(always)]
    #[must_use]
    pub fn fast_mem_wa(&mut self) -> FAST_MEM_WA_W<19> {
        FAST_MEM_WA_W::new(self)
    }
    #[doc = "Bits 22:23 - This field controls fast memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
    #[inline(always)]
    #[must_use]
    pub fn fast_mem_ra(&mut self) -> FAST_MEM_RA_W<22> {
        FAST_MEM_RA_W::new(self)
    }
    #[doc = "Bits 24:27 - This field controls fast memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
    #[inline(always)]
    #[must_use]
    pub fn fast_mem_rm(&mut self) -> FAST_MEM_RM_W<24> {
        FAST_MEM_RM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpbus](index.html) module"]
pub struct LPBUS_SPEC;
impl crate::RegisterSpec for LPBUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpbus::R](R) reader structure"]
impl crate::Readable for LPBUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpbus::W](W) writer structure"]
impl crate::Writable for LPBUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPBUS to value 0x0228_0000"]
impl crate::Resettable for LPBUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0228_0000;
}
