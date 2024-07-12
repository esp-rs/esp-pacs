#[doc = "Register `A_SYS_CONF` reader"]
pub type R = crate::R<A_SYS_CONF_SPEC>;
#[doc = "Register `A_SYS_CONF` writer"]
pub type W = crate::W<A_SYS_CONF_SPEC>;
#[doc = "Field `A_DB_TMP_READY_TRIGGER_MB_NUM` reader - Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
pub type A_DB_TMP_READY_TRIGGER_MB_NUM_R = crate::FieldReader;
#[doc = "Field `A_DB_TMP_READY_TRIGGER_MB_NUM` writer - Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
pub type A_DB_TMP_READY_TRIGGER_MB_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_REC_READY_TRIGGER_MB_LINES` reader - Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
pub type A_REC_READY_TRIGGER_MB_LINES_R = crate::FieldReader;
#[doc = "Field `A_REC_READY_TRIGGER_MB_LINES` writer - Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
pub type A_REC_READY_TRIGGER_MB_LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_INTRA_COST_CMP_OFFSET` reader - Configures video A intra cost offset when I MB compared with P MB."]
pub type A_INTRA_COST_CMP_OFFSET_R = crate::FieldReader<u16>;
#[doc = "Field `A_INTRA_COST_CMP_OFFSET` writer - Configures video A intra cost offset when I MB compared with P MB."]
pub type A_INTRA_COST_CMP_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
    #[inline(always)]
    pub fn a_db_tmp_ready_trigger_mb_num(&self) -> A_DB_TMP_READY_TRIGGER_MB_NUM_R {
        A_DB_TMP_READY_TRIGGER_MB_NUM_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
    #[inline(always)]
    pub fn a_rec_ready_trigger_mb_lines(&self) -> A_REC_READY_TRIGGER_MB_LINES_R {
        A_REC_READY_TRIGGER_MB_LINES_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:29 - Configures video A intra cost offset when I MB compared with P MB."]
    #[inline(always)]
    pub fn a_intra_cost_cmp_offset(&self) -> A_INTRA_COST_CMP_OFFSET_R {
        A_INTRA_COST_CMP_OFFSET_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_SYS_CONF")
            .field(
                "a_db_tmp_ready_trigger_mb_num",
                &self.a_db_tmp_ready_trigger_mb_num(),
            )
            .field(
                "a_rec_ready_trigger_mb_lines",
                &self.a_rec_ready_trigger_mb_lines(),
            )
            .field("a_intra_cost_cmp_offset", &self.a_intra_cost_cmp_offset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
    #[inline(always)]
    #[must_use]
    pub fn a_db_tmp_ready_trigger_mb_num(
        &mut self,
    ) -> A_DB_TMP_READY_TRIGGER_MB_NUM_W<A_SYS_CONF_SPEC> {
        A_DB_TMP_READY_TRIGGER_MB_NUM_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
    #[inline(always)]
    #[must_use]
    pub fn a_rec_ready_trigger_mb_lines(
        &mut self,
    ) -> A_REC_READY_TRIGGER_MB_LINES_W<A_SYS_CONF_SPEC> {
        A_REC_READY_TRIGGER_MB_LINES_W::new(self, 7)
    }
    #[doc = "Bits 14:29 - Configures video A intra cost offset when I MB compared with P MB."]
    #[inline(always)]
    #[must_use]
    pub fn a_intra_cost_cmp_offset(&mut self) -> A_INTRA_COST_CMP_OFFSET_W<A_SYS_CONF_SPEC> {
        A_INTRA_COST_CMP_OFFSET_W::new(self, 14)
    }
}
#[doc = "Video A system level configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_sys_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_sys_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_SYS_CONF_SPEC;
impl crate::RegisterSpec for A_SYS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_sys_conf::R`](R) reader structure"]
impl crate::Readable for A_SYS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_sys_conf::W`](W) writer structure"]
impl crate::Writable for A_SYS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A_SYS_CONF to value 0x0203"]
impl crate::Resettable for A_SYS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0203;
}
