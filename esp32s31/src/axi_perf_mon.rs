#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    agent_select: AGENT_SELECT,
    sel_ag0_counter0: SEL_AG0_COUNTER0,
    sel_ag1_counter0: SEL_AG1_COUNTER0,
    sel_ag2_counter0: SEL_AG2_COUNTER0,
    sel_ag3_counter0: SEL_AG3_COUNTER0,
    sel_ag4_counter0: SEL_AG4_COUNTER0,
    sel_ag5_counter0: SEL_AG5_COUNTER0,
    sel_ag6_counter0: SEL_AG6_COUNTER0,
    sel_ag0_counter1: SEL_AG0_COUNTER1,
    sel_ag1_counter1: SEL_AG1_COUNTER1,
    sel_ag2_counter1: SEL_AG2_COUNTER1,
    sel_ag3_counter1: SEL_AG3_COUNTER1,
    sel_ag4_counter1: SEL_AG4_COUNTER1,
    sel_ag5_counter1: SEL_AG5_COUNTER1,
    sel_ag6_counter1: SEL_AG6_COUNTER1,
    sel_ag0_counter2: SEL_AG0_COUNTER2,
    sel_ag1_counter2: SEL_AG1_COUNTER2,
    sel_ag2_counter2: SEL_AG2_COUNTER2,
    sel_ag3_counter2: SEL_AG3_COUNTER2,
    sel_ag4_counter2: SEL_AG4_COUNTER2,
    sel_ag5_counter2: SEL_AG5_COUNTER2,
    sel_ag6_counter2: SEL_AG6_COUNTER2,
    sel_ag0_counter3: SEL_AG0_COUNTER3,
    sel_ag1_counter3: SEL_AG1_COUNTER3,
    sel_ag2_counter3: SEL_AG2_COUNTER3,
    sel_ag3_counter3: SEL_AG3_COUNTER3,
    sel_ag4_counter3: SEL_AG4_COUNTER3,
    sel_ag5_counter3: SEL_AG5_COUNTER3,
    sel_ag6_counter3: SEL_AG6_COUNTER3,
    sel_ag0_counter4: SEL_AG0_COUNTER4,
    sel_ag1_counter4: SEL_AG1_COUNTER4,
    sel_ag2_counter4: SEL_AG2_COUNTER4,
    sel_ag3_counter4: SEL_AG3_COUNTER4,
    sel_ag4_counter4: SEL_AG4_COUNTER4,
    sel_ag5_counter4: SEL_AG5_COUNTER4,
    sel_ag6_counter4: SEL_AG6_COUNTER4,
    sel_ag0_counter5: SEL_AG0_COUNTER5,
    sel_ag1_counter5: SEL_AG1_COUNTER5,
    sel_ag2_counter5: SEL_AG2_COUNTER5,
    sel_ag3_counter5: SEL_AG3_COUNTER5,
    sel_ag4_counter5: SEL_AG4_COUNTER5,
    sel_ag5_counter5: SEL_AG5_COUNTER5,
    sel_ag6_counter5: SEL_AG6_COUNTER5,
    sel_ag0_counter6: SEL_AG0_COUNTER6,
    sel_ag1_counter6: SEL_AG1_COUNTER6,
    sel_ag2_counter6: SEL_AG2_COUNTER6,
    sel_ag3_counter6: SEL_AG3_COUNTER6,
    sel_ag4_counter6: SEL_AG4_COUNTER6,
    sel_ag5_counter6: SEL_AG5_COUNTER6,
    sel_ag6_counter6: SEL_AG6_COUNTER6,
    sel_ag0_counter7: SEL_AG0_COUNTER7,
    sel_ag1_counter7: SEL_AG1_COUNTER7,
    sel_ag2_counter7: SEL_AG2_COUNTER7,
    sel_ag3_counter7: SEL_AG3_COUNTER7,
    sel_ag4_counter7: SEL_AG4_COUNTER7,
    sel_ag5_counter7: SEL_AG5_COUNTER7,
    sel_ag6_counter7: SEL_AG6_COUNTER7,
    sel_ag0_range0: SEL_AG0_RANGE0,
    sel_ag1_range0: SEL_AG1_RANGE0,
    sel_ag2_range0: SEL_AG2_RANGE0,
    sel_ag3_range0: SEL_AG3_RANGE0,
    sel_ag4_range0: SEL_AG4_RANGE0,
    sel_ag5_range0: SEL_AG5_RANGE0,
    sel_ag6_range0: SEL_AG6_RANGE0,
    sel_ag0_range1: SEL_AG0_RANGE1,
    sel_ag1_range1: SEL_AG1_RANGE1,
    sel_ag2_range1: SEL_AG2_RANGE1,
    sel_ag3_range1: SEL_AG3_RANGE1,
    sel_ag4_range1: SEL_AG4_RANGE1,
    sel_ag5_range1: SEL_AG5_RANGE1,
    sel_ag6_range1: SEL_AG6_RANGE1,
    sel_ag0_range2: SEL_AG0_RANGE2,
    sel_ag1_range2: SEL_AG1_RANGE2,
    sel_ag2_range2: SEL_AG2_RANGE2,
    sel_ag3_range2: SEL_AG3_RANGE2,
    sel_ag4_range2: SEL_AG4_RANGE2,
    sel_ag5_range2: SEL_AG5_RANGE2,
    sel_ag6_range2: SEL_AG6_RANGE2,
    sel_ag0_range3: SEL_AG0_RANGE3,
    sel_ag1_range3: SEL_AG1_RANGE3,
    sel_ag2_range3: SEL_AG2_RANGE3,
    sel_ag3_range3: SEL_AG3_RANGE3,
    sel_ag4_range3: SEL_AG4_RANGE3,
    sel_ag5_range3: SEL_AG5_RANGE3,
    sel_ag6_range3: SEL_AG6_RANGE3,
    sel_ag0_range4: SEL_AG0_RANGE4,
    sel_ag1_range4: SEL_AG1_RANGE4,
    sel_ag2_range4: SEL_AG2_RANGE4,
    sel_ag3_range4: SEL_AG3_RANGE4,
    sel_ag4_range4: SEL_AG4_RANGE4,
    sel_ag5_range4: SEL_AG5_RANGE4,
    sel_ag6_range4: SEL_AG6_RANGE4,
    sel_ag0_range5: SEL_AG0_RANGE5,
    sel_ag1_range5: SEL_AG1_RANGE5,
    sel_ag2_range5: SEL_AG2_RANGE5,
    sel_ag3_range5: SEL_AG3_RANGE5,
    sel_ag4_range5: SEL_AG4_RANGE5,
    sel_ag5_range5: SEL_AG5_RANGE5,
    sel_ag6_range5: SEL_AG6_RANGE5,
    sel_ag0_range6: SEL_AG0_RANGE6,
    sel_ag1_range6: SEL_AG1_RANGE6,
    sel_ag2_range6: SEL_AG2_RANGE6,
    sel_ag3_range6: SEL_AG3_RANGE6,
    sel_ag4_range6: SEL_AG4_RANGE6,
    sel_ag5_range6: SEL_AG5_RANGE6,
    sel_ag6_range6: SEL_AG6_RANGE6,
    sel_ag0_range7: SEL_AG0_RANGE7,
    sel_ag1_range7: SEL_AG1_RANGE7,
    sel_ag2_range7: SEL_AG2_RANGE7,
    sel_ag3_range7: SEL_AG3_RANGE7,
    sel_ag4_range7: SEL_AG4_RANGE7,
    sel_ag5_range7: SEL_AG5_RANGE7,
    sel_ag6_range7: SEL_AG6_RANGE7,
    sel_ag0_rd_axi_info_record0: SEL_AG0_RD_AXI_INFO_RECORD0,
    sel_ag1_rd_axi_info_record0: SEL_AG1_RD_AXI_INFO_RECORD0,
    sel_ag2_rd_axi_info_record0: SEL_AG2_RD_AXI_INFO_RECORD0,
    sel_ag3_rd_axi_info_record0: SEL_AG3_RD_AXI_INFO_RECORD0,
    sel_ag4_rd_axi_info_record0: SEL_AG4_RD_AXI_INFO_RECORD0,
    sel_ag5_rd_axi_info_record0: SEL_AG5_RD_AXI_INFO_RECORD0,
    sel_ag6_rd_axi_info_record0: SEL_AG6_RD_AXI_INFO_RECORD0,
    sel_ag0_rd_axi_info_record1: SEL_AG0_RD_AXI_INFO_RECORD1,
    sel_ag1_rd_axi_info_record1: SEL_AG1_RD_AXI_INFO_RECORD1,
    sel_ag2_rd_axi_info_record1: SEL_AG2_RD_AXI_INFO_RECORD1,
    sel_ag3_rd_axi_info_record1: SEL_AG3_RD_AXI_INFO_RECORD1,
    sel_ag4_rd_axi_info_record1: SEL_AG4_RD_AXI_INFO_RECORD1,
    sel_ag5_rd_axi_info_record1: SEL_AG5_RD_AXI_INFO_RECORD1,
    sel_ag6_rd_axi_info_record1: SEL_AG6_RD_AXI_INFO_RECORD1,
    sel_ag0_rd_axi_info_record2: SEL_AG0_RD_AXI_INFO_RECORD2,
    sel_ag1_rd_axi_info_record2: SEL_AG1_RD_AXI_INFO_RECORD2,
    sel_ag2_rd_axi_info_record2: SEL_AG2_RD_AXI_INFO_RECORD2,
    sel_ag3_rd_axi_info_record2: SEL_AG3_RD_AXI_INFO_RECORD2,
    sel_ag4_rd_axi_info_record2: SEL_AG4_RD_AXI_INFO_RECORD2,
    sel_ag5_rd_axi_info_record2: SEL_AG5_RD_AXI_INFO_RECORD2,
    sel_ag6_rd_axi_info_record2: SEL_AG6_RD_AXI_INFO_RECORD2,
    sel_ag0_rd_axi_info_record3: SEL_AG0_RD_AXI_INFO_RECORD3,
    sel_ag1_rd_axi_info_record3: SEL_AG1_RD_AXI_INFO_RECORD3,
    sel_ag2_rd_axi_info_record3: SEL_AG2_RD_AXI_INFO_RECORD3,
    sel_ag3_rd_axi_info_record3: SEL_AG3_RD_AXI_INFO_RECORD3,
    sel_ag4_rd_axi_info_record3: SEL_AG4_RD_AXI_INFO_RECORD3,
    sel_ag5_rd_axi_info_record3: SEL_AG5_RD_AXI_INFO_RECORD3,
    sel_ag6_rd_axi_info_record3: SEL_AG6_RD_AXI_INFO_RECORD3,
    sel_ag0_wr_axi_info_record0: SEL_AG0_WR_AXI_INFO_RECORD0,
    sel_ag1_wr_axi_info_record0: SEL_AG1_WR_AXI_INFO_RECORD0,
    sel_ag2_wr_axi_info_record0: SEL_AG2_WR_AXI_INFO_RECORD0,
    sel_ag3_wr_axi_info_record0: SEL_AG3_WR_AXI_INFO_RECORD0,
    sel_ag4_wr_axi_info_record0: SEL_AG4_WR_AXI_INFO_RECORD0,
    sel_ag5_wr_axi_info_record0: SEL_AG5_WR_AXI_INFO_RECORD0,
    sel_ag6_wr_axi_info_record0: SEL_AG6_WR_AXI_INFO_RECORD0,
    sel_ag0_wr_axi_info_record1: SEL_AG0_WR_AXI_INFO_RECORD1,
    sel_ag1_wr_axi_info_record1: SEL_AG1_WR_AXI_INFO_RECORD1,
    sel_ag2_wr_axi_info_record1: SEL_AG2_WR_AXI_INFO_RECORD1,
    sel_ag3_wr_axi_info_record1: SEL_AG3_WR_AXI_INFO_RECORD1,
    sel_ag4_wr_axi_info_record1: SEL_AG4_WR_AXI_INFO_RECORD1,
    sel_ag5_wr_axi_info_record1: SEL_AG5_WR_AXI_INFO_RECORD1,
    sel_ag6_wr_axi_info_record1: SEL_AG6_WR_AXI_INFO_RECORD1,
    sel_ag0_wr_axi_info_record2: SEL_AG0_WR_AXI_INFO_RECORD2,
    sel_ag1_wr_axi_info_record2: SEL_AG1_WR_AXI_INFO_RECORD2,
    sel_ag2_wr_axi_info_record2: SEL_AG2_WR_AXI_INFO_RECORD2,
    sel_ag3_wr_axi_info_record2: SEL_AG3_WR_AXI_INFO_RECORD2,
    sel_ag4_wr_axi_info_record2: SEL_AG4_WR_AXI_INFO_RECORD2,
    sel_ag5_wr_axi_info_record2: SEL_AG5_WR_AXI_INFO_RECORD2,
    sel_ag6_wr_axi_info_record2: SEL_AG6_WR_AXI_INFO_RECORD2,
    sel_ag0_wr_axi_info_record3: SEL_AG0_WR_AXI_INFO_RECORD3,
    sel_ag1_wr_axi_info_record3: SEL_AG1_WR_AXI_INFO_RECORD3,
    sel_ag2_wr_axi_info_record3: SEL_AG2_WR_AXI_INFO_RECORD3,
    sel_ag3_wr_axi_info_record3: SEL_AG3_WR_AXI_INFO_RECORD3,
    sel_ag4_wr_axi_info_record3: SEL_AG4_WR_AXI_INFO_RECORD3,
    sel_ag5_wr_axi_info_record3: SEL_AG5_WR_AXI_INFO_RECORD3,
    sel_ag6_wr_axi_info_record3: SEL_AG6_WR_AXI_INFO_RECORD3,
    sel_ag0_ins_bandw_data0: SEL_AG0_INS_BANDW_DATA0,
    sel_ag1_ins_bandw_data0: SEL_AG1_INS_BANDW_DATA0,
    sel_ag2_ins_bandw_data0: SEL_AG2_INS_BANDW_DATA0,
    sel_ag3_ins_bandw_data0: SEL_AG3_INS_BANDW_DATA0,
    sel_ag4_ins_bandw_data0: SEL_AG4_INS_BANDW_DATA0,
    sel_ag5_ins_bandw_data0: SEL_AG5_INS_BANDW_DATA0,
    sel_ag6_ins_bandw_data0: SEL_AG6_INS_BANDW_DATA0,
    sel_ag0_ins_bandw_data1: SEL_AG0_INS_BANDW_DATA1,
    sel_ag1_ins_bandw_data1: SEL_AG1_INS_BANDW_DATA1,
    sel_ag2_ins_bandw_data1: SEL_AG2_INS_BANDW_DATA1,
    sel_ag3_ins_bandw_data1: SEL_AG3_INS_BANDW_DATA1,
    sel_ag4_ins_bandw_data1: SEL_AG4_INS_BANDW_DATA1,
    sel_ag5_ins_bandw_data1: SEL_AG5_INS_BANDW_DATA1,
    sel_ag6_ins_bandw_data1: SEL_AG6_INS_BANDW_DATA1,
    sel_ag0_ins_bandw_data2: SEL_AG0_INS_BANDW_DATA2,
    sel_ag1_ins_bandw_data2: SEL_AG1_INS_BANDW_DATA2,
    sel_ag2_ins_bandw_data2: SEL_AG2_INS_BANDW_DATA2,
    sel_ag3_ins_bandw_data2: SEL_AG3_INS_BANDW_DATA2,
    sel_ag4_ins_bandw_data2: SEL_AG4_INS_BANDW_DATA2,
    sel_ag5_ins_bandw_data2: SEL_AG5_INS_BANDW_DATA2,
    sel_ag6_ins_bandw_data2: SEL_AG6_INS_BANDW_DATA2,
    sel_ag0_ins_bandw_data3: SEL_AG0_INS_BANDW_DATA3,
    sel_ag1_ins_bandw_data3: SEL_AG1_INS_BANDW_DATA3,
    sel_ag2_ins_bandw_data3: SEL_AG2_INS_BANDW_DATA3,
    sel_ag3_ins_bandw_data3: SEL_AG3_INS_BANDW_DATA3,
    sel_ag4_ins_bandw_data3: SEL_AG4_INS_BANDW_DATA3,
    sel_ag5_ins_bandw_data3: SEL_AG5_INS_BANDW_DATA3,
    sel_ag6_ins_bandw_data3: SEL_AG6_INS_BANDW_DATA3,
    sel_ag0_metric_range0: SEL_AG0_METRIC_RANGE0,
    sel_ag1_metric_range0: SEL_AG1_METRIC_RANGE0,
    sel_ag2_metric_range0: SEL_AG2_METRIC_RANGE0,
    sel_ag3_metric_range0: SEL_AG3_METRIC_RANGE0,
    sel_ag4_metric_range0: SEL_AG4_METRIC_RANGE0,
    sel_ag5_metric_range0: SEL_AG5_METRIC_RANGE0,
    sel_ag6_metric_range0: SEL_AG6_METRIC_RANGE0,
    sel_ag0_metric_range1: SEL_AG0_METRIC_RANGE1,
    sel_ag1_metric_range1: SEL_AG1_METRIC_RANGE1,
    sel_ag2_metric_range1: SEL_AG2_METRIC_RANGE1,
    sel_ag3_metric_range1: SEL_AG3_METRIC_RANGE1,
    sel_ag4_metric_range1: SEL_AG4_METRIC_RANGE1,
    sel_ag5_metric_range1: SEL_AG5_METRIC_RANGE1,
    sel_ag6_metric_range1: SEL_AG6_METRIC_RANGE1,
    sel_ag0_metric_range2: SEL_AG0_METRIC_RANGE2,
    sel_ag1_metric_range2: SEL_AG1_METRIC_RANGE2,
    sel_ag2_metric_range2: SEL_AG2_METRIC_RANGE2,
    sel_ag3_metric_range2: SEL_AG3_METRIC_RANGE2,
    sel_ag4_metric_range2: SEL_AG4_METRIC_RANGE2,
    sel_ag5_metric_range2: SEL_AG5_METRIC_RANGE2,
    sel_ag6_metric_range2: SEL_AG6_METRIC_RANGE2,
    sel_ag0_metric_range3: SEL_AG0_METRIC_RANGE3,
    sel_ag1_metric_range3: SEL_AG1_METRIC_RANGE3,
    sel_ag2_metric_range3: SEL_AG2_METRIC_RANGE3,
    sel_ag3_metric_range3: SEL_AG3_METRIC_RANGE3,
    sel_ag4_metric_range3: SEL_AG4_METRIC_RANGE3,
    sel_ag5_metric_range3: SEL_AG5_METRIC_RANGE3,
    sel_ag6_metric_range3: SEL_AG6_METRIC_RANGE3,
    sel_ag0_metric_range4: SEL_AG0_METRIC_RANGE4,
    sel_ag1_metric_range4: SEL_AG1_METRIC_RANGE4,
    sel_ag2_metric_range4: SEL_AG2_METRIC_RANGE4,
    sel_ag3_metric_range4: SEL_AG3_METRIC_RANGE4,
    sel_ag4_metric_range4: SEL_AG4_METRIC_RANGE4,
    sel_ag5_metric_range4: SEL_AG5_METRIC_RANGE4,
    sel_ag6_metric_range4: SEL_AG6_METRIC_RANGE4,
    sel_ag0_metric_range5: SEL_AG0_METRIC_RANGE5,
    sel_ag1_metric_range5: SEL_AG1_METRIC_RANGE5,
    sel_ag2_metric_range5: SEL_AG2_METRIC_RANGE5,
    sel_ag3_metric_range5: SEL_AG3_METRIC_RANGE5,
    sel_ag4_metric_range5: SEL_AG4_METRIC_RANGE5,
    sel_ag5_metric_range5: SEL_AG5_METRIC_RANGE5,
    sel_ag6_metric_range5: SEL_AG6_METRIC_RANGE5,
    sel_ag0_metric_range6: SEL_AG0_METRIC_RANGE6,
    sel_ag1_metric_range6: SEL_AG1_METRIC_RANGE6,
    sel_ag2_metric_range6: SEL_AG2_METRIC_RANGE6,
    sel_ag3_metric_range6: SEL_AG3_METRIC_RANGE6,
    sel_ag4_metric_range6: SEL_AG4_METRIC_RANGE6,
    sel_ag5_metric_range6: SEL_AG5_METRIC_RANGE6,
    sel_ag6_metric_range6: SEL_AG6_METRIC_RANGE6,
    sel_ag0_metric_range7: SEL_AG0_METRIC_RANGE7,
    sel_ag1_metric_range7: SEL_AG1_METRIC_RANGE7,
    sel_ag2_metric_range7: SEL_AG2_METRIC_RANGE7,
    sel_ag3_metric_range7: SEL_AG3_METRIC_RANGE7,
    sel_ag4_metric_range7: SEL_AG4_METRIC_RANGE7,
    sel_ag5_metric_range7: SEL_AG5_METRIC_RANGE7,
    sel_ag6_metric_range7: SEL_AG6_METRIC_RANGE7,
    sel_ag0_rd_addr_mask0: SEL_AG0_RD_ADDR_MASK0,
    sel_ag1_rd_addr_mask0: SEL_AG1_RD_ADDR_MASK0,
    sel_ag2_rd_addr_mask0: SEL_AG2_RD_ADDR_MASK0,
    sel_ag3_rd_addr_mask0: SEL_AG3_RD_ADDR_MASK0,
    sel_ag4_rd_addr_mask0: SEL_AG4_RD_ADDR_MASK0,
    sel_ag5_rd_addr_mask0: SEL_AG5_RD_ADDR_MASK0,
    sel_ag6_rd_addr_mask0: SEL_AG6_RD_ADDR_MASK0,
    sel_ag0_rd_addr_mask1: SEL_AG0_RD_ADDR_MASK1,
    sel_ag1_rd_addr_mask1: SEL_AG1_RD_ADDR_MASK1,
    sel_ag2_rd_addr_mask1: SEL_AG2_RD_ADDR_MASK1,
    sel_ag3_rd_addr_mask1: SEL_AG3_RD_ADDR_MASK1,
    sel_ag4_rd_addr_mask1: SEL_AG4_RD_ADDR_MASK1,
    sel_ag5_rd_addr_mask1: SEL_AG5_RD_ADDR_MASK1,
    sel_ag6_rd_addr_mask1: SEL_AG6_RD_ADDR_MASK1,
    sel_ag0_rd_addr_mask2: SEL_AG0_RD_ADDR_MASK2,
    sel_ag1_rd_addr_mask2: SEL_AG1_RD_ADDR_MASK2,
    sel_ag2_rd_addr_mask2: SEL_AG2_RD_ADDR_MASK2,
    sel_ag3_rd_addr_mask2: SEL_AG3_RD_ADDR_MASK2,
    sel_ag4_rd_addr_mask2: SEL_AG4_RD_ADDR_MASK2,
    sel_ag5_rd_addr_mask2: SEL_AG5_RD_ADDR_MASK2,
    sel_ag6_rd_addr_mask2: SEL_AG6_RD_ADDR_MASK2,
    sel_ag0_wr_addr_mask0: SEL_AG0_WR_ADDR_MASK0,
    sel_ag1_wr_addr_mask0: SEL_AG1_WR_ADDR_MASK0,
    sel_ag2_wr_addr_mask0: SEL_AG2_WR_ADDR_MASK0,
    sel_ag3_wr_addr_mask0: SEL_AG3_WR_ADDR_MASK0,
    sel_ag4_wr_addr_mask0: SEL_AG4_WR_ADDR_MASK0,
    sel_ag5_wr_addr_mask0: SEL_AG5_WR_ADDR_MASK0,
    sel_ag6_wr_addr_mask0: SEL_AG6_WR_ADDR_MASK0,
    sel_ag0_wr_addr_mask1: SEL_AG0_WR_ADDR_MASK1,
    sel_ag1_wr_addr_mask1: SEL_AG1_WR_ADDR_MASK1,
    sel_ag2_wr_addr_mask1: SEL_AG2_WR_ADDR_MASK1,
    sel_ag3_wr_addr_mask1: SEL_AG3_WR_ADDR_MASK1,
    sel_ag4_wr_addr_mask1: SEL_AG4_WR_ADDR_MASK1,
    sel_ag5_wr_addr_mask1: SEL_AG5_WR_ADDR_MASK1,
    sel_ag6_wr_addr_mask1: SEL_AG6_WR_ADDR_MASK1,
    sel_ag0_wr_addr_mask2: SEL_AG0_WR_ADDR_MASK2,
    sel_ag1_wr_addr_mask2: SEL_AG1_WR_ADDR_MASK2,
    sel_ag2_wr_addr_mask2: SEL_AG2_WR_ADDR_MASK2,
    sel_ag3_wr_addr_mask2: SEL_AG3_WR_ADDR_MASK2,
    sel_ag4_wr_addr_mask2: SEL_AG4_WR_ADDR_MASK2,
    sel_ag5_wr_addr_mask2: SEL_AG5_WR_ADDR_MASK2,
    sel_ag6_wr_addr_mask2: SEL_AG6_WR_ADDR_MASK2,
    sel_ag0_rd_addr_filter0: SEL_AG0_RD_ADDR_FILTER0,
    sel_ag1_rd_addr_filter0: SEL_AG1_RD_ADDR_FILTER0,
    sel_ag2_rd_addr_filter0: SEL_AG2_RD_ADDR_FILTER0,
    sel_ag3_rd_addr_filter0: SEL_AG3_RD_ADDR_FILTER0,
    sel_ag4_rd_addr_filter0: SEL_AG4_RD_ADDR_FILTER0,
    sel_ag5_rd_addr_filter0: SEL_AG5_RD_ADDR_FILTER0,
    sel_ag6_rd_addr_filter0: SEL_AG6_RD_ADDR_FILTER0,
    sel_ag0_rd_addr_filter1: SEL_AG0_RD_ADDR_FILTER1,
    sel_ag1_rd_addr_filter1: SEL_AG1_RD_ADDR_FILTER1,
    sel_ag2_rd_addr_filter1: SEL_AG2_RD_ADDR_FILTER1,
    sel_ag3_rd_addr_filter1: SEL_AG3_RD_ADDR_FILTER1,
    sel_ag4_rd_addr_filter1: SEL_AG4_RD_ADDR_FILTER1,
    sel_ag5_rd_addr_filter1: SEL_AG5_RD_ADDR_FILTER1,
    sel_ag6_rd_addr_filter1: SEL_AG6_RD_ADDR_FILTER1,
    sel_ag0_rd_addr_filter2: SEL_AG0_RD_ADDR_FILTER2,
    sel_ag1_rd_addr_filter2: SEL_AG1_RD_ADDR_FILTER2,
    sel_ag2_rd_addr_filter2: SEL_AG2_RD_ADDR_FILTER2,
    sel_ag3_rd_addr_filter2: SEL_AG3_RD_ADDR_FILTER2,
    sel_ag4_rd_addr_filter2: SEL_AG4_RD_ADDR_FILTER2,
    sel_ag5_rd_addr_filter2: SEL_AG5_RD_ADDR_FILTER2,
    sel_ag6_rd_addr_filter2: SEL_AG6_RD_ADDR_FILTER2,
    sel_ag0_wr_addr_filter0: SEL_AG0_WR_ADDR_FILTER0,
    sel_ag1_wr_addr_filter0: SEL_AG1_WR_ADDR_FILTER0,
    sel_ag2_wr_addr_filter0: SEL_AG2_WR_ADDR_FILTER0,
    sel_ag3_wr_addr_filter0: SEL_AG3_WR_ADDR_FILTER0,
    sel_ag4_wr_addr_filter0: SEL_AG4_WR_ADDR_FILTER0,
    sel_ag5_wr_addr_filter0: SEL_AG5_WR_ADDR_FILTER0,
    sel_ag6_wr_addr_filter0: SEL_AG6_WR_ADDR_FILTER0,
    sel_ag0_wr_addr_filter1: SEL_AG0_WR_ADDR_FILTER1,
    sel_ag1_wr_addr_filter1: SEL_AG1_WR_ADDR_FILTER1,
    sel_ag2_wr_addr_filter1: SEL_AG2_WR_ADDR_FILTER1,
    sel_ag3_wr_addr_filter1: SEL_AG3_WR_ADDR_FILTER1,
    sel_ag4_wr_addr_filter1: SEL_AG4_WR_ADDR_FILTER1,
    sel_ag5_wr_addr_filter1: SEL_AG5_WR_ADDR_FILTER1,
    sel_ag6_wr_addr_filter1: SEL_AG6_WR_ADDR_FILTER1,
    sel_ag0_wr_addr_filter2: SEL_AG0_WR_ADDR_FILTER2,
    sel_ag1_wr_addr_filter2: SEL_AG1_WR_ADDR_FILTER2,
    sel_ag2_wr_addr_filter2: SEL_AG2_WR_ADDR_FILTER2,
    sel_ag3_wr_addr_filter2: SEL_AG3_WR_ADDR_FILTER2,
    sel_ag4_wr_addr_filter2: SEL_AG4_WR_ADDR_FILTER2,
    sel_ag5_wr_addr_filter2: SEL_AG5_WR_ADDR_FILTER2,
    sel_ag6_wr_addr_filter2: SEL_AG6_WR_ADDR_FILTER2,
    sel_ag0_metric_select1: SEL_AG0_METRIC_SELECT1,
    sel_ag1_metric_select1: SEL_AG1_METRIC_SELECT1,
    sel_ag2_metric_select1: SEL_AG2_METRIC_SELECT1,
    sel_ag3_metric_select1: SEL_AG3_METRIC_SELECT1,
    sel_ag4_metric_select1: SEL_AG4_METRIC_SELECT1,
    sel_ag5_metric_select1: SEL_AG5_METRIC_SELECT1,
    sel_ag6_metric_select1: SEL_AG6_METRIC_SELECT1,
    sel_ag0_metric_select2: SEL_AG0_METRIC_SELECT2,
    sel_ag1_metric_select2: SEL_AG1_METRIC_SELECT2,
    sel_ag2_metric_select2: SEL_AG2_METRIC_SELECT2,
    sel_ag3_metric_select2: SEL_AG3_METRIC_SELECT2,
    sel_ag4_metric_select2: SEL_AG4_METRIC_SELECT2,
    sel_ag5_metric_select2: SEL_AG5_METRIC_SELECT2,
    sel_ag6_metric_select2: SEL_AG6_METRIC_SELECT2,
    sel_ag_rd_addr_region_sel: SEL_AG_RD_ADDR_REGION_SEL,
    sel_ag_wr_addr_region_sel: SEL_AG_WR_ADDR_REGION_SEL,
    sel_ag_addr_filter_en: SEL_AG_ADDR_FILTER_EN,
    sel_ag_sw_record_stop_en: SEL_AG_SW_RECORD_STOP_EN,
    sel_ag_sw_record_stop_clr: SEL_AG_SW_RECORD_STOP_CLR,
    sel_ag_ins_bandw_test_en: SEL_AG_INS_BANDW_TEST_EN,
    sel_ag0_sw_record_stop_data_limit: SEL_AG0_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag1_sw_record_stop_data_limit: SEL_AG1_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag2_sw_record_stop_data_limit: SEL_AG2_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag3_sw_record_stop_data_limit: SEL_AG3_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag4_sw_record_stop_data_limit: SEL_AG4_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag5_sw_record_stop_data_limit: SEL_AG5_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag6_sw_record_stop_data_limit: SEL_AG6_SW_RECORD_STOP_DATA_LIMIT,
    sel_ag0_id_mask: SEL_AG0_ID_MASK,
    sel_ag1_id_mask: SEL_AG1_ID_MASK,
    sel_ag2_id_mask: SEL_AG2_ID_MASK,
    sel_ag3_id_mask: SEL_AG3_ID_MASK,
    sel_ag4_id_mask: SEL_AG4_ID_MASK,
    sel_ag5_id_mask: SEL_AG5_ID_MASK,
    sel_ag6_id_mask: SEL_AG6_ID_MASK,
    sel_ag0_id_filter: SEL_AG0_ID_FILTER,
    sel_ag1_id_filter: SEL_AG1_ID_FILTER,
    sel_ag2_id_filter: SEL_AG2_ID_FILTER,
    sel_ag3_id_filter: SEL_AG3_ID_FILTER,
    sel_ag4_id_filter: SEL_AG4_ID_FILTER,
    sel_ag5_id_filter: SEL_AG5_ID_FILTER,
    sel_ag6_id_filter: SEL_AG6_ID_FILTER,
    sel_ag_bandw_test_en: SEL_AG_BANDW_TEST_EN,
    sel_ag_bandw_test_stop: SEL_AG_BANDW_TEST_STOP,
    sel_ag0_bandw_trigger_in_sel: SEL_AG0_BANDW_TRIGGER_IN_SEL,
    sel_ag1_bandw_trigger_in_sel: SEL_AG1_BANDW_TRIGGER_IN_SEL,
    sel_ag2_bandw_trigger_in_sel: SEL_AG2_BANDW_TRIGGER_IN_SEL,
    sel_ag3_bandw_trigger_in_sel: SEL_AG3_BANDW_TRIGGER_IN_SEL,
    sel_ag4_bandw_trigger_in_sel: SEL_AG4_BANDW_TRIGGER_IN_SEL,
    sel_ag5_bandw_trigger_in_sel: SEL_AG5_BANDW_TRIGGER_IN_SEL,
    sel_ag6_bandw_trigger_in_sel: SEL_AG6_BANDW_TRIGGER_IN_SEL,
    sel_ag0_wr_bandw_cnt_valid_strobe_num: SEL_AG0_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag1_wr_bandw_cnt_valid_strobe_num: SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag2_wr_bandw_cnt_valid_strobe_num: SEL_AG2_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag3_wr_bandw_cnt_valid_strobe_num: SEL_AG3_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag4_wr_bandw_cnt_valid_strobe_num: SEL_AG4_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag5_wr_bandw_cnt_valid_strobe_num: SEL_AG5_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag6_wr_bandw_cnt_valid_strobe_num: SEL_AG6_WR_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag0_rd_bandw_cnt_valid_strobe_num: SEL_AG0_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag1_rd_bandw_cnt_valid_strobe_num: SEL_AG1_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag2_rd_bandw_cnt_valid_strobe_num: SEL_AG2_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag3_rd_bandw_cnt_valid_strobe_num: SEL_AG3_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag4_rd_bandw_cnt_valid_strobe_num: SEL_AG4_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag5_rd_bandw_cnt_valid_strobe_num: SEL_AG5_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag6_rd_bandw_cnt_valid_strobe_num: SEL_AG6_RD_BANDW_CNT_VALID_STROBE_NUM,
    sel_ag0_ins_bandw_time_thr: SEL_AG0_INS_BANDW_TIME_THR,
    sel_ag1_ins_bandw_time_thr: SEL_AG1_INS_BANDW_TIME_THR,
    sel_ag2_ins_bandw_time_thr: SEL_AG2_INS_BANDW_TIME_THR,
    sel_ag3_ins_bandw_time_thr: SEL_AG3_INS_BANDW_TIME_THR,
    sel_ag4_ins_bandw_time_thr: SEL_AG4_INS_BANDW_TIME_THR,
    sel_ag5_ins_bandw_time_thr: SEL_AG5_INS_BANDW_TIME_THR,
    sel_ag6_ins_bandw_time_thr: SEL_AG6_INS_BANDW_TIME_THR,
    sel_ag_int_raw: SEL_AG_INT_RAW,
    sel_ag_int_st: SEL_AG_INT_ST,
    sel_ag_int_ena: SEL_AG_INT_ENA,
    sel_ag_int_clr: SEL_AG_INT_CLR,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - reserved"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x04 - reserved"]
    #[inline(always)]
    pub const fn agent_select(&self) -> &AGENT_SELECT {
        &self.agent_select
    }
    #[doc = "0x08 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter0(&self) -> &SEL_AG0_COUNTER0 {
        &self.sel_ag0_counter0
    }
    #[doc = "0x0c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter0(&self) -> &SEL_AG1_COUNTER0 {
        &self.sel_ag1_counter0
    }
    #[doc = "0x10 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter0(&self) -> &SEL_AG2_COUNTER0 {
        &self.sel_ag2_counter0
    }
    #[doc = "0x14 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter0(&self) -> &SEL_AG3_COUNTER0 {
        &self.sel_ag3_counter0
    }
    #[doc = "0x18 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter0(&self) -> &SEL_AG4_COUNTER0 {
        &self.sel_ag4_counter0
    }
    #[doc = "0x1c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter0(&self) -> &SEL_AG5_COUNTER0 {
        &self.sel_ag5_counter0
    }
    #[doc = "0x20 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter0(&self) -> &SEL_AG6_COUNTER0 {
        &self.sel_ag6_counter0
    }
    #[doc = "0x24 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter1(&self) -> &SEL_AG0_COUNTER1 {
        &self.sel_ag0_counter1
    }
    #[doc = "0x28 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter1(&self) -> &SEL_AG1_COUNTER1 {
        &self.sel_ag1_counter1
    }
    #[doc = "0x2c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter1(&self) -> &SEL_AG2_COUNTER1 {
        &self.sel_ag2_counter1
    }
    #[doc = "0x30 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter1(&self) -> &SEL_AG3_COUNTER1 {
        &self.sel_ag3_counter1
    }
    #[doc = "0x34 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter1(&self) -> &SEL_AG4_COUNTER1 {
        &self.sel_ag4_counter1
    }
    #[doc = "0x38 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter1(&self) -> &SEL_AG5_COUNTER1 {
        &self.sel_ag5_counter1
    }
    #[doc = "0x3c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter1(&self) -> &SEL_AG6_COUNTER1 {
        &self.sel_ag6_counter1
    }
    #[doc = "0x40 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter2(&self) -> &SEL_AG0_COUNTER2 {
        &self.sel_ag0_counter2
    }
    #[doc = "0x44 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter2(&self) -> &SEL_AG1_COUNTER2 {
        &self.sel_ag1_counter2
    }
    #[doc = "0x48 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter2(&self) -> &SEL_AG2_COUNTER2 {
        &self.sel_ag2_counter2
    }
    #[doc = "0x4c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter2(&self) -> &SEL_AG3_COUNTER2 {
        &self.sel_ag3_counter2
    }
    #[doc = "0x50 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter2(&self) -> &SEL_AG4_COUNTER2 {
        &self.sel_ag4_counter2
    }
    #[doc = "0x54 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter2(&self) -> &SEL_AG5_COUNTER2 {
        &self.sel_ag5_counter2
    }
    #[doc = "0x58 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter2(&self) -> &SEL_AG6_COUNTER2 {
        &self.sel_ag6_counter2
    }
    #[doc = "0x5c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter3(&self) -> &SEL_AG0_COUNTER3 {
        &self.sel_ag0_counter3
    }
    #[doc = "0x60 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter3(&self) -> &SEL_AG1_COUNTER3 {
        &self.sel_ag1_counter3
    }
    #[doc = "0x64 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter3(&self) -> &SEL_AG2_COUNTER3 {
        &self.sel_ag2_counter3
    }
    #[doc = "0x68 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter3(&self) -> &SEL_AG3_COUNTER3 {
        &self.sel_ag3_counter3
    }
    #[doc = "0x6c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter3(&self) -> &SEL_AG4_COUNTER3 {
        &self.sel_ag4_counter3
    }
    #[doc = "0x70 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter3(&self) -> &SEL_AG5_COUNTER3 {
        &self.sel_ag5_counter3
    }
    #[doc = "0x74 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter3(&self) -> &SEL_AG6_COUNTER3 {
        &self.sel_ag6_counter3
    }
    #[doc = "0x78 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter4(&self) -> &SEL_AG0_COUNTER4 {
        &self.sel_ag0_counter4
    }
    #[doc = "0x7c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter4(&self) -> &SEL_AG1_COUNTER4 {
        &self.sel_ag1_counter4
    }
    #[doc = "0x80 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter4(&self) -> &SEL_AG2_COUNTER4 {
        &self.sel_ag2_counter4
    }
    #[doc = "0x84 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter4(&self) -> &SEL_AG3_COUNTER4 {
        &self.sel_ag3_counter4
    }
    #[doc = "0x88 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter4(&self) -> &SEL_AG4_COUNTER4 {
        &self.sel_ag4_counter4
    }
    #[doc = "0x8c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter4(&self) -> &SEL_AG5_COUNTER4 {
        &self.sel_ag5_counter4
    }
    #[doc = "0x90 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter4(&self) -> &SEL_AG6_COUNTER4 {
        &self.sel_ag6_counter4
    }
    #[doc = "0x94 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter5(&self) -> &SEL_AG0_COUNTER5 {
        &self.sel_ag0_counter5
    }
    #[doc = "0x98 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter5(&self) -> &SEL_AG1_COUNTER5 {
        &self.sel_ag1_counter5
    }
    #[doc = "0x9c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter5(&self) -> &SEL_AG2_COUNTER5 {
        &self.sel_ag2_counter5
    }
    #[doc = "0xa0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter5(&self) -> &SEL_AG3_COUNTER5 {
        &self.sel_ag3_counter5
    }
    #[doc = "0xa4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter5(&self) -> &SEL_AG4_COUNTER5 {
        &self.sel_ag4_counter5
    }
    #[doc = "0xa8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter5(&self) -> &SEL_AG5_COUNTER5 {
        &self.sel_ag5_counter5
    }
    #[doc = "0xac - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter5(&self) -> &SEL_AG6_COUNTER5 {
        &self.sel_ag6_counter5
    }
    #[doc = "0xb0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter6(&self) -> &SEL_AG0_COUNTER6 {
        &self.sel_ag0_counter6
    }
    #[doc = "0xb4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter6(&self) -> &SEL_AG1_COUNTER6 {
        &self.sel_ag1_counter6
    }
    #[doc = "0xb8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter6(&self) -> &SEL_AG2_COUNTER6 {
        &self.sel_ag2_counter6
    }
    #[doc = "0xbc - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter6(&self) -> &SEL_AG3_COUNTER6 {
        &self.sel_ag3_counter6
    }
    #[doc = "0xc0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter6(&self) -> &SEL_AG4_COUNTER6 {
        &self.sel_ag4_counter6
    }
    #[doc = "0xc4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter6(&self) -> &SEL_AG5_COUNTER6 {
        &self.sel_ag5_counter6
    }
    #[doc = "0xc8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter6(&self) -> &SEL_AG6_COUNTER6 {
        &self.sel_ag6_counter6
    }
    #[doc = "0xcc - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_counter7(&self) -> &SEL_AG0_COUNTER7 {
        &self.sel_ag0_counter7
    }
    #[doc = "0xd0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_counter7(&self) -> &SEL_AG1_COUNTER7 {
        &self.sel_ag1_counter7
    }
    #[doc = "0xd4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_counter7(&self) -> &SEL_AG2_COUNTER7 {
        &self.sel_ag2_counter7
    }
    #[doc = "0xd8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_counter7(&self) -> &SEL_AG3_COUNTER7 {
        &self.sel_ag3_counter7
    }
    #[doc = "0xdc - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_counter7(&self) -> &SEL_AG4_COUNTER7 {
        &self.sel_ag4_counter7
    }
    #[doc = "0xe0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_counter7(&self) -> &SEL_AG5_COUNTER7 {
        &self.sel_ag5_counter7
    }
    #[doc = "0xe4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_counter7(&self) -> &SEL_AG6_COUNTER7 {
        &self.sel_ag6_counter7
    }
    #[doc = "0xe8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range0(&self) -> &SEL_AG0_RANGE0 {
        &self.sel_ag0_range0
    }
    #[doc = "0xec - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range0(&self) -> &SEL_AG1_RANGE0 {
        &self.sel_ag1_range0
    }
    #[doc = "0xf0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range0(&self) -> &SEL_AG2_RANGE0 {
        &self.sel_ag2_range0
    }
    #[doc = "0xf4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range0(&self) -> &SEL_AG3_RANGE0 {
        &self.sel_ag3_range0
    }
    #[doc = "0xf8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range0(&self) -> &SEL_AG4_RANGE0 {
        &self.sel_ag4_range0
    }
    #[doc = "0xfc - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range0(&self) -> &SEL_AG5_RANGE0 {
        &self.sel_ag5_range0
    }
    #[doc = "0x100 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range0(&self) -> &SEL_AG6_RANGE0 {
        &self.sel_ag6_range0
    }
    #[doc = "0x104 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range1(&self) -> &SEL_AG0_RANGE1 {
        &self.sel_ag0_range1
    }
    #[doc = "0x108 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range1(&self) -> &SEL_AG1_RANGE1 {
        &self.sel_ag1_range1
    }
    #[doc = "0x10c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range1(&self) -> &SEL_AG2_RANGE1 {
        &self.sel_ag2_range1
    }
    #[doc = "0x110 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range1(&self) -> &SEL_AG3_RANGE1 {
        &self.sel_ag3_range1
    }
    #[doc = "0x114 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range1(&self) -> &SEL_AG4_RANGE1 {
        &self.sel_ag4_range1
    }
    #[doc = "0x118 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range1(&self) -> &SEL_AG5_RANGE1 {
        &self.sel_ag5_range1
    }
    #[doc = "0x11c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range1(&self) -> &SEL_AG6_RANGE1 {
        &self.sel_ag6_range1
    }
    #[doc = "0x120 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range2(&self) -> &SEL_AG0_RANGE2 {
        &self.sel_ag0_range2
    }
    #[doc = "0x124 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range2(&self) -> &SEL_AG1_RANGE2 {
        &self.sel_ag1_range2
    }
    #[doc = "0x128 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range2(&self) -> &SEL_AG2_RANGE2 {
        &self.sel_ag2_range2
    }
    #[doc = "0x12c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range2(&self) -> &SEL_AG3_RANGE2 {
        &self.sel_ag3_range2
    }
    #[doc = "0x130 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range2(&self) -> &SEL_AG4_RANGE2 {
        &self.sel_ag4_range2
    }
    #[doc = "0x134 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range2(&self) -> &SEL_AG5_RANGE2 {
        &self.sel_ag5_range2
    }
    #[doc = "0x138 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range2(&self) -> &SEL_AG6_RANGE2 {
        &self.sel_ag6_range2
    }
    #[doc = "0x13c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range3(&self) -> &SEL_AG0_RANGE3 {
        &self.sel_ag0_range3
    }
    #[doc = "0x140 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range3(&self) -> &SEL_AG1_RANGE3 {
        &self.sel_ag1_range3
    }
    #[doc = "0x144 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range3(&self) -> &SEL_AG2_RANGE3 {
        &self.sel_ag2_range3
    }
    #[doc = "0x148 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range3(&self) -> &SEL_AG3_RANGE3 {
        &self.sel_ag3_range3
    }
    #[doc = "0x14c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range3(&self) -> &SEL_AG4_RANGE3 {
        &self.sel_ag4_range3
    }
    #[doc = "0x150 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range3(&self) -> &SEL_AG5_RANGE3 {
        &self.sel_ag5_range3
    }
    #[doc = "0x154 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range3(&self) -> &SEL_AG6_RANGE3 {
        &self.sel_ag6_range3
    }
    #[doc = "0x158 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range4(&self) -> &SEL_AG0_RANGE4 {
        &self.sel_ag0_range4
    }
    #[doc = "0x15c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range4(&self) -> &SEL_AG1_RANGE4 {
        &self.sel_ag1_range4
    }
    #[doc = "0x160 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range4(&self) -> &SEL_AG2_RANGE4 {
        &self.sel_ag2_range4
    }
    #[doc = "0x164 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range4(&self) -> &SEL_AG3_RANGE4 {
        &self.sel_ag3_range4
    }
    #[doc = "0x168 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range4(&self) -> &SEL_AG4_RANGE4 {
        &self.sel_ag4_range4
    }
    #[doc = "0x16c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range4(&self) -> &SEL_AG5_RANGE4 {
        &self.sel_ag5_range4
    }
    #[doc = "0x170 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range4(&self) -> &SEL_AG6_RANGE4 {
        &self.sel_ag6_range4
    }
    #[doc = "0x174 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range5(&self) -> &SEL_AG0_RANGE5 {
        &self.sel_ag0_range5
    }
    #[doc = "0x178 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range5(&self) -> &SEL_AG1_RANGE5 {
        &self.sel_ag1_range5
    }
    #[doc = "0x17c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range5(&self) -> &SEL_AG2_RANGE5 {
        &self.sel_ag2_range5
    }
    #[doc = "0x180 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range5(&self) -> &SEL_AG3_RANGE5 {
        &self.sel_ag3_range5
    }
    #[doc = "0x184 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range5(&self) -> &SEL_AG4_RANGE5 {
        &self.sel_ag4_range5
    }
    #[doc = "0x188 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range5(&self) -> &SEL_AG5_RANGE5 {
        &self.sel_ag5_range5
    }
    #[doc = "0x18c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range5(&self) -> &SEL_AG6_RANGE5 {
        &self.sel_ag6_range5
    }
    #[doc = "0x190 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range6(&self) -> &SEL_AG0_RANGE6 {
        &self.sel_ag0_range6
    }
    #[doc = "0x194 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range6(&self) -> &SEL_AG1_RANGE6 {
        &self.sel_ag1_range6
    }
    #[doc = "0x198 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range6(&self) -> &SEL_AG2_RANGE6 {
        &self.sel_ag2_range6
    }
    #[doc = "0x19c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range6(&self) -> &SEL_AG3_RANGE6 {
        &self.sel_ag3_range6
    }
    #[doc = "0x1a0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range6(&self) -> &SEL_AG4_RANGE6 {
        &self.sel_ag4_range6
    }
    #[doc = "0x1a4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range6(&self) -> &SEL_AG5_RANGE6 {
        &self.sel_ag5_range6
    }
    #[doc = "0x1a8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range6(&self) -> &SEL_AG6_RANGE6 {
        &self.sel_ag6_range6
    }
    #[doc = "0x1ac - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_range7(&self) -> &SEL_AG0_RANGE7 {
        &self.sel_ag0_range7
    }
    #[doc = "0x1b0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_range7(&self) -> &SEL_AG1_RANGE7 {
        &self.sel_ag1_range7
    }
    #[doc = "0x1b4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_range7(&self) -> &SEL_AG2_RANGE7 {
        &self.sel_ag2_range7
    }
    #[doc = "0x1b8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_range7(&self) -> &SEL_AG3_RANGE7 {
        &self.sel_ag3_range7
    }
    #[doc = "0x1bc - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_range7(&self) -> &SEL_AG4_RANGE7 {
        &self.sel_ag4_range7
    }
    #[doc = "0x1c0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_range7(&self) -> &SEL_AG5_RANGE7 {
        &self.sel_ag5_range7
    }
    #[doc = "0x1c4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_range7(&self) -> &SEL_AG6_RANGE7 {
        &self.sel_ag6_range7
    }
    #[doc = "0x1c8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_axi_info_record0(&self) -> &SEL_AG0_RD_AXI_INFO_RECORD0 {
        &self.sel_ag0_rd_axi_info_record0
    }
    #[doc = "0x1cc - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_axi_info_record0(&self) -> &SEL_AG1_RD_AXI_INFO_RECORD0 {
        &self.sel_ag1_rd_axi_info_record0
    }
    #[doc = "0x1d0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_axi_info_record0(&self) -> &SEL_AG2_RD_AXI_INFO_RECORD0 {
        &self.sel_ag2_rd_axi_info_record0
    }
    #[doc = "0x1d4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_axi_info_record0(&self) -> &SEL_AG3_RD_AXI_INFO_RECORD0 {
        &self.sel_ag3_rd_axi_info_record0
    }
    #[doc = "0x1d8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_axi_info_record0(&self) -> &SEL_AG4_RD_AXI_INFO_RECORD0 {
        &self.sel_ag4_rd_axi_info_record0
    }
    #[doc = "0x1dc - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_axi_info_record0(&self) -> &SEL_AG5_RD_AXI_INFO_RECORD0 {
        &self.sel_ag5_rd_axi_info_record0
    }
    #[doc = "0x1e0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_axi_info_record0(&self) -> &SEL_AG6_RD_AXI_INFO_RECORD0 {
        &self.sel_ag6_rd_axi_info_record0
    }
    #[doc = "0x1e4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_axi_info_record1(&self) -> &SEL_AG0_RD_AXI_INFO_RECORD1 {
        &self.sel_ag0_rd_axi_info_record1
    }
    #[doc = "0x1e8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_axi_info_record1(&self) -> &SEL_AG1_RD_AXI_INFO_RECORD1 {
        &self.sel_ag1_rd_axi_info_record1
    }
    #[doc = "0x1ec - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_axi_info_record1(&self) -> &SEL_AG2_RD_AXI_INFO_RECORD1 {
        &self.sel_ag2_rd_axi_info_record1
    }
    #[doc = "0x1f0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_axi_info_record1(&self) -> &SEL_AG3_RD_AXI_INFO_RECORD1 {
        &self.sel_ag3_rd_axi_info_record1
    }
    #[doc = "0x1f4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_axi_info_record1(&self) -> &SEL_AG4_RD_AXI_INFO_RECORD1 {
        &self.sel_ag4_rd_axi_info_record1
    }
    #[doc = "0x1f8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_axi_info_record1(&self) -> &SEL_AG5_RD_AXI_INFO_RECORD1 {
        &self.sel_ag5_rd_axi_info_record1
    }
    #[doc = "0x1fc - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_axi_info_record1(&self) -> &SEL_AG6_RD_AXI_INFO_RECORD1 {
        &self.sel_ag6_rd_axi_info_record1
    }
    #[doc = "0x200 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_axi_info_record2(&self) -> &SEL_AG0_RD_AXI_INFO_RECORD2 {
        &self.sel_ag0_rd_axi_info_record2
    }
    #[doc = "0x204 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_axi_info_record2(&self) -> &SEL_AG1_RD_AXI_INFO_RECORD2 {
        &self.sel_ag1_rd_axi_info_record2
    }
    #[doc = "0x208 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_axi_info_record2(&self) -> &SEL_AG2_RD_AXI_INFO_RECORD2 {
        &self.sel_ag2_rd_axi_info_record2
    }
    #[doc = "0x20c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_axi_info_record2(&self) -> &SEL_AG3_RD_AXI_INFO_RECORD2 {
        &self.sel_ag3_rd_axi_info_record2
    }
    #[doc = "0x210 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_axi_info_record2(&self) -> &SEL_AG4_RD_AXI_INFO_RECORD2 {
        &self.sel_ag4_rd_axi_info_record2
    }
    #[doc = "0x214 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_axi_info_record2(&self) -> &SEL_AG5_RD_AXI_INFO_RECORD2 {
        &self.sel_ag5_rd_axi_info_record2
    }
    #[doc = "0x218 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_axi_info_record2(&self) -> &SEL_AG6_RD_AXI_INFO_RECORD2 {
        &self.sel_ag6_rd_axi_info_record2
    }
    #[doc = "0x21c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_axi_info_record3(&self) -> &SEL_AG0_RD_AXI_INFO_RECORD3 {
        &self.sel_ag0_rd_axi_info_record3
    }
    #[doc = "0x220 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_axi_info_record3(&self) -> &SEL_AG1_RD_AXI_INFO_RECORD3 {
        &self.sel_ag1_rd_axi_info_record3
    }
    #[doc = "0x224 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_axi_info_record3(&self) -> &SEL_AG2_RD_AXI_INFO_RECORD3 {
        &self.sel_ag2_rd_axi_info_record3
    }
    #[doc = "0x228 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_axi_info_record3(&self) -> &SEL_AG3_RD_AXI_INFO_RECORD3 {
        &self.sel_ag3_rd_axi_info_record3
    }
    #[doc = "0x22c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_axi_info_record3(&self) -> &SEL_AG4_RD_AXI_INFO_RECORD3 {
        &self.sel_ag4_rd_axi_info_record3
    }
    #[doc = "0x230 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_axi_info_record3(&self) -> &SEL_AG5_RD_AXI_INFO_RECORD3 {
        &self.sel_ag5_rd_axi_info_record3
    }
    #[doc = "0x234 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_axi_info_record3(&self) -> &SEL_AG6_RD_AXI_INFO_RECORD3 {
        &self.sel_ag6_rd_axi_info_record3
    }
    #[doc = "0x238 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_axi_info_record0(&self) -> &SEL_AG0_WR_AXI_INFO_RECORD0 {
        &self.sel_ag0_wr_axi_info_record0
    }
    #[doc = "0x23c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_axi_info_record0(&self) -> &SEL_AG1_WR_AXI_INFO_RECORD0 {
        &self.sel_ag1_wr_axi_info_record0
    }
    #[doc = "0x240 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_axi_info_record0(&self) -> &SEL_AG2_WR_AXI_INFO_RECORD0 {
        &self.sel_ag2_wr_axi_info_record0
    }
    #[doc = "0x244 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_axi_info_record0(&self) -> &SEL_AG3_WR_AXI_INFO_RECORD0 {
        &self.sel_ag3_wr_axi_info_record0
    }
    #[doc = "0x248 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_axi_info_record0(&self) -> &SEL_AG4_WR_AXI_INFO_RECORD0 {
        &self.sel_ag4_wr_axi_info_record0
    }
    #[doc = "0x24c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_axi_info_record0(&self) -> &SEL_AG5_WR_AXI_INFO_RECORD0 {
        &self.sel_ag5_wr_axi_info_record0
    }
    #[doc = "0x250 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_axi_info_record0(&self) -> &SEL_AG6_WR_AXI_INFO_RECORD0 {
        &self.sel_ag6_wr_axi_info_record0
    }
    #[doc = "0x254 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_axi_info_record1(&self) -> &SEL_AG0_WR_AXI_INFO_RECORD1 {
        &self.sel_ag0_wr_axi_info_record1
    }
    #[doc = "0x258 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_axi_info_record1(&self) -> &SEL_AG1_WR_AXI_INFO_RECORD1 {
        &self.sel_ag1_wr_axi_info_record1
    }
    #[doc = "0x25c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_axi_info_record1(&self) -> &SEL_AG2_WR_AXI_INFO_RECORD1 {
        &self.sel_ag2_wr_axi_info_record1
    }
    #[doc = "0x260 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_axi_info_record1(&self) -> &SEL_AG3_WR_AXI_INFO_RECORD1 {
        &self.sel_ag3_wr_axi_info_record1
    }
    #[doc = "0x264 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_axi_info_record1(&self) -> &SEL_AG4_WR_AXI_INFO_RECORD1 {
        &self.sel_ag4_wr_axi_info_record1
    }
    #[doc = "0x268 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_axi_info_record1(&self) -> &SEL_AG5_WR_AXI_INFO_RECORD1 {
        &self.sel_ag5_wr_axi_info_record1
    }
    #[doc = "0x26c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_axi_info_record1(&self) -> &SEL_AG6_WR_AXI_INFO_RECORD1 {
        &self.sel_ag6_wr_axi_info_record1
    }
    #[doc = "0x270 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_axi_info_record2(&self) -> &SEL_AG0_WR_AXI_INFO_RECORD2 {
        &self.sel_ag0_wr_axi_info_record2
    }
    #[doc = "0x274 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_axi_info_record2(&self) -> &SEL_AG1_WR_AXI_INFO_RECORD2 {
        &self.sel_ag1_wr_axi_info_record2
    }
    #[doc = "0x278 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_axi_info_record2(&self) -> &SEL_AG2_WR_AXI_INFO_RECORD2 {
        &self.sel_ag2_wr_axi_info_record2
    }
    #[doc = "0x27c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_axi_info_record2(&self) -> &SEL_AG3_WR_AXI_INFO_RECORD2 {
        &self.sel_ag3_wr_axi_info_record2
    }
    #[doc = "0x280 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_axi_info_record2(&self) -> &SEL_AG4_WR_AXI_INFO_RECORD2 {
        &self.sel_ag4_wr_axi_info_record2
    }
    #[doc = "0x284 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_axi_info_record2(&self) -> &SEL_AG5_WR_AXI_INFO_RECORD2 {
        &self.sel_ag5_wr_axi_info_record2
    }
    #[doc = "0x288 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_axi_info_record2(&self) -> &SEL_AG6_WR_AXI_INFO_RECORD2 {
        &self.sel_ag6_wr_axi_info_record2
    }
    #[doc = "0x28c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_axi_info_record3(&self) -> &SEL_AG0_WR_AXI_INFO_RECORD3 {
        &self.sel_ag0_wr_axi_info_record3
    }
    #[doc = "0x290 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_axi_info_record3(&self) -> &SEL_AG1_WR_AXI_INFO_RECORD3 {
        &self.sel_ag1_wr_axi_info_record3
    }
    #[doc = "0x294 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_axi_info_record3(&self) -> &SEL_AG2_WR_AXI_INFO_RECORD3 {
        &self.sel_ag2_wr_axi_info_record3
    }
    #[doc = "0x298 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_axi_info_record3(&self) -> &SEL_AG3_WR_AXI_INFO_RECORD3 {
        &self.sel_ag3_wr_axi_info_record3
    }
    #[doc = "0x29c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_axi_info_record3(&self) -> &SEL_AG4_WR_AXI_INFO_RECORD3 {
        &self.sel_ag4_wr_axi_info_record3
    }
    #[doc = "0x2a0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_axi_info_record3(&self) -> &SEL_AG5_WR_AXI_INFO_RECORD3 {
        &self.sel_ag5_wr_axi_info_record3
    }
    #[doc = "0x2a4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_axi_info_record3(&self) -> &SEL_AG6_WR_AXI_INFO_RECORD3 {
        &self.sel_ag6_wr_axi_info_record3
    }
    #[doc = "0x2a8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_ins_bandw_data0(&self) -> &SEL_AG0_INS_BANDW_DATA0 {
        &self.sel_ag0_ins_bandw_data0
    }
    #[doc = "0x2ac - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_ins_bandw_data0(&self) -> &SEL_AG1_INS_BANDW_DATA0 {
        &self.sel_ag1_ins_bandw_data0
    }
    #[doc = "0x2b0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_ins_bandw_data0(&self) -> &SEL_AG2_INS_BANDW_DATA0 {
        &self.sel_ag2_ins_bandw_data0
    }
    #[doc = "0x2b4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_ins_bandw_data0(&self) -> &SEL_AG3_INS_BANDW_DATA0 {
        &self.sel_ag3_ins_bandw_data0
    }
    #[doc = "0x2b8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_ins_bandw_data0(&self) -> &SEL_AG4_INS_BANDW_DATA0 {
        &self.sel_ag4_ins_bandw_data0
    }
    #[doc = "0x2bc - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_ins_bandw_data0(&self) -> &SEL_AG5_INS_BANDW_DATA0 {
        &self.sel_ag5_ins_bandw_data0
    }
    #[doc = "0x2c0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_ins_bandw_data0(&self) -> &SEL_AG6_INS_BANDW_DATA0 {
        &self.sel_ag6_ins_bandw_data0
    }
    #[doc = "0x2c4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_ins_bandw_data1(&self) -> &SEL_AG0_INS_BANDW_DATA1 {
        &self.sel_ag0_ins_bandw_data1
    }
    #[doc = "0x2c8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_ins_bandw_data1(&self) -> &SEL_AG1_INS_BANDW_DATA1 {
        &self.sel_ag1_ins_bandw_data1
    }
    #[doc = "0x2cc - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_ins_bandw_data1(&self) -> &SEL_AG2_INS_BANDW_DATA1 {
        &self.sel_ag2_ins_bandw_data1
    }
    #[doc = "0x2d0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_ins_bandw_data1(&self) -> &SEL_AG3_INS_BANDW_DATA1 {
        &self.sel_ag3_ins_bandw_data1
    }
    #[doc = "0x2d4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_ins_bandw_data1(&self) -> &SEL_AG4_INS_BANDW_DATA1 {
        &self.sel_ag4_ins_bandw_data1
    }
    #[doc = "0x2d8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_ins_bandw_data1(&self) -> &SEL_AG5_INS_BANDW_DATA1 {
        &self.sel_ag5_ins_bandw_data1
    }
    #[doc = "0x2dc - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_ins_bandw_data1(&self) -> &SEL_AG6_INS_BANDW_DATA1 {
        &self.sel_ag6_ins_bandw_data1
    }
    #[doc = "0x2e0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_ins_bandw_data2(&self) -> &SEL_AG0_INS_BANDW_DATA2 {
        &self.sel_ag0_ins_bandw_data2
    }
    #[doc = "0x2e4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_ins_bandw_data2(&self) -> &SEL_AG1_INS_BANDW_DATA2 {
        &self.sel_ag1_ins_bandw_data2
    }
    #[doc = "0x2e8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_ins_bandw_data2(&self) -> &SEL_AG2_INS_BANDW_DATA2 {
        &self.sel_ag2_ins_bandw_data2
    }
    #[doc = "0x2ec - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_ins_bandw_data2(&self) -> &SEL_AG3_INS_BANDW_DATA2 {
        &self.sel_ag3_ins_bandw_data2
    }
    #[doc = "0x2f0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_ins_bandw_data2(&self) -> &SEL_AG4_INS_BANDW_DATA2 {
        &self.sel_ag4_ins_bandw_data2
    }
    #[doc = "0x2f4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_ins_bandw_data2(&self) -> &SEL_AG5_INS_BANDW_DATA2 {
        &self.sel_ag5_ins_bandw_data2
    }
    #[doc = "0x2f8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_ins_bandw_data2(&self) -> &SEL_AG6_INS_BANDW_DATA2 {
        &self.sel_ag6_ins_bandw_data2
    }
    #[doc = "0x2fc - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_ins_bandw_data3(&self) -> &SEL_AG0_INS_BANDW_DATA3 {
        &self.sel_ag0_ins_bandw_data3
    }
    #[doc = "0x300 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_ins_bandw_data3(&self) -> &SEL_AG1_INS_BANDW_DATA3 {
        &self.sel_ag1_ins_bandw_data3
    }
    #[doc = "0x304 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_ins_bandw_data3(&self) -> &SEL_AG2_INS_BANDW_DATA3 {
        &self.sel_ag2_ins_bandw_data3
    }
    #[doc = "0x308 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_ins_bandw_data3(&self) -> &SEL_AG3_INS_BANDW_DATA3 {
        &self.sel_ag3_ins_bandw_data3
    }
    #[doc = "0x30c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_ins_bandw_data3(&self) -> &SEL_AG4_INS_BANDW_DATA3 {
        &self.sel_ag4_ins_bandw_data3
    }
    #[doc = "0x310 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_ins_bandw_data3(&self) -> &SEL_AG5_INS_BANDW_DATA3 {
        &self.sel_ag5_ins_bandw_data3
    }
    #[doc = "0x314 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_ins_bandw_data3(&self) -> &SEL_AG6_INS_BANDW_DATA3 {
        &self.sel_ag6_ins_bandw_data3
    }
    #[doc = "0x318 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range0(&self) -> &SEL_AG0_METRIC_RANGE0 {
        &self.sel_ag0_metric_range0
    }
    #[doc = "0x31c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range0(&self) -> &SEL_AG1_METRIC_RANGE0 {
        &self.sel_ag1_metric_range0
    }
    #[doc = "0x320 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range0(&self) -> &SEL_AG2_METRIC_RANGE0 {
        &self.sel_ag2_metric_range0
    }
    #[doc = "0x324 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range0(&self) -> &SEL_AG3_METRIC_RANGE0 {
        &self.sel_ag3_metric_range0
    }
    #[doc = "0x328 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range0(&self) -> &SEL_AG4_METRIC_RANGE0 {
        &self.sel_ag4_metric_range0
    }
    #[doc = "0x32c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range0(&self) -> &SEL_AG5_METRIC_RANGE0 {
        &self.sel_ag5_metric_range0
    }
    #[doc = "0x330 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range0(&self) -> &SEL_AG6_METRIC_RANGE0 {
        &self.sel_ag6_metric_range0
    }
    #[doc = "0x334 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range1(&self) -> &SEL_AG0_METRIC_RANGE1 {
        &self.sel_ag0_metric_range1
    }
    #[doc = "0x338 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range1(&self) -> &SEL_AG1_METRIC_RANGE1 {
        &self.sel_ag1_metric_range1
    }
    #[doc = "0x33c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range1(&self) -> &SEL_AG2_METRIC_RANGE1 {
        &self.sel_ag2_metric_range1
    }
    #[doc = "0x340 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range1(&self) -> &SEL_AG3_METRIC_RANGE1 {
        &self.sel_ag3_metric_range1
    }
    #[doc = "0x344 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range1(&self) -> &SEL_AG4_METRIC_RANGE1 {
        &self.sel_ag4_metric_range1
    }
    #[doc = "0x348 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range1(&self) -> &SEL_AG5_METRIC_RANGE1 {
        &self.sel_ag5_metric_range1
    }
    #[doc = "0x34c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range1(&self) -> &SEL_AG6_METRIC_RANGE1 {
        &self.sel_ag6_metric_range1
    }
    #[doc = "0x350 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range2(&self) -> &SEL_AG0_METRIC_RANGE2 {
        &self.sel_ag0_metric_range2
    }
    #[doc = "0x354 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range2(&self) -> &SEL_AG1_METRIC_RANGE2 {
        &self.sel_ag1_metric_range2
    }
    #[doc = "0x358 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range2(&self) -> &SEL_AG2_METRIC_RANGE2 {
        &self.sel_ag2_metric_range2
    }
    #[doc = "0x35c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range2(&self) -> &SEL_AG3_METRIC_RANGE2 {
        &self.sel_ag3_metric_range2
    }
    #[doc = "0x360 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range2(&self) -> &SEL_AG4_METRIC_RANGE2 {
        &self.sel_ag4_metric_range2
    }
    #[doc = "0x364 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range2(&self) -> &SEL_AG5_METRIC_RANGE2 {
        &self.sel_ag5_metric_range2
    }
    #[doc = "0x368 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range2(&self) -> &SEL_AG6_METRIC_RANGE2 {
        &self.sel_ag6_metric_range2
    }
    #[doc = "0x36c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range3(&self) -> &SEL_AG0_METRIC_RANGE3 {
        &self.sel_ag0_metric_range3
    }
    #[doc = "0x370 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range3(&self) -> &SEL_AG1_METRIC_RANGE3 {
        &self.sel_ag1_metric_range3
    }
    #[doc = "0x374 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range3(&self) -> &SEL_AG2_METRIC_RANGE3 {
        &self.sel_ag2_metric_range3
    }
    #[doc = "0x378 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range3(&self) -> &SEL_AG3_METRIC_RANGE3 {
        &self.sel_ag3_metric_range3
    }
    #[doc = "0x37c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range3(&self) -> &SEL_AG4_METRIC_RANGE3 {
        &self.sel_ag4_metric_range3
    }
    #[doc = "0x380 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range3(&self) -> &SEL_AG5_METRIC_RANGE3 {
        &self.sel_ag5_metric_range3
    }
    #[doc = "0x384 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range3(&self) -> &SEL_AG6_METRIC_RANGE3 {
        &self.sel_ag6_metric_range3
    }
    #[doc = "0x388 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range4(&self) -> &SEL_AG0_METRIC_RANGE4 {
        &self.sel_ag0_metric_range4
    }
    #[doc = "0x38c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range4(&self) -> &SEL_AG1_METRIC_RANGE4 {
        &self.sel_ag1_metric_range4
    }
    #[doc = "0x390 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range4(&self) -> &SEL_AG2_METRIC_RANGE4 {
        &self.sel_ag2_metric_range4
    }
    #[doc = "0x394 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range4(&self) -> &SEL_AG3_METRIC_RANGE4 {
        &self.sel_ag3_metric_range4
    }
    #[doc = "0x398 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range4(&self) -> &SEL_AG4_METRIC_RANGE4 {
        &self.sel_ag4_metric_range4
    }
    #[doc = "0x39c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range4(&self) -> &SEL_AG5_METRIC_RANGE4 {
        &self.sel_ag5_metric_range4
    }
    #[doc = "0x3a0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range4(&self) -> &SEL_AG6_METRIC_RANGE4 {
        &self.sel_ag6_metric_range4
    }
    #[doc = "0x3a4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range5(&self) -> &SEL_AG0_METRIC_RANGE5 {
        &self.sel_ag0_metric_range5
    }
    #[doc = "0x3a8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range5(&self) -> &SEL_AG1_METRIC_RANGE5 {
        &self.sel_ag1_metric_range5
    }
    #[doc = "0x3ac - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range5(&self) -> &SEL_AG2_METRIC_RANGE5 {
        &self.sel_ag2_metric_range5
    }
    #[doc = "0x3b0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range5(&self) -> &SEL_AG3_METRIC_RANGE5 {
        &self.sel_ag3_metric_range5
    }
    #[doc = "0x3b4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range5(&self) -> &SEL_AG4_METRIC_RANGE5 {
        &self.sel_ag4_metric_range5
    }
    #[doc = "0x3b8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range5(&self) -> &SEL_AG5_METRIC_RANGE5 {
        &self.sel_ag5_metric_range5
    }
    #[doc = "0x3bc - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range5(&self) -> &SEL_AG6_METRIC_RANGE5 {
        &self.sel_ag6_metric_range5
    }
    #[doc = "0x3c0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range6(&self) -> &SEL_AG0_METRIC_RANGE6 {
        &self.sel_ag0_metric_range6
    }
    #[doc = "0x3c4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range6(&self) -> &SEL_AG1_METRIC_RANGE6 {
        &self.sel_ag1_metric_range6
    }
    #[doc = "0x3c8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range6(&self) -> &SEL_AG2_METRIC_RANGE6 {
        &self.sel_ag2_metric_range6
    }
    #[doc = "0x3cc - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range6(&self) -> &SEL_AG3_METRIC_RANGE6 {
        &self.sel_ag3_metric_range6
    }
    #[doc = "0x3d0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range6(&self) -> &SEL_AG4_METRIC_RANGE6 {
        &self.sel_ag4_metric_range6
    }
    #[doc = "0x3d4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range6(&self) -> &SEL_AG5_METRIC_RANGE6 {
        &self.sel_ag5_metric_range6
    }
    #[doc = "0x3d8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range6(&self) -> &SEL_AG6_METRIC_RANGE6 {
        &self.sel_ag6_metric_range6
    }
    #[doc = "0x3dc - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_range7(&self) -> &SEL_AG0_METRIC_RANGE7 {
        &self.sel_ag0_metric_range7
    }
    #[doc = "0x3e0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_range7(&self) -> &SEL_AG1_METRIC_RANGE7 {
        &self.sel_ag1_metric_range7
    }
    #[doc = "0x3e4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_range7(&self) -> &SEL_AG2_METRIC_RANGE7 {
        &self.sel_ag2_metric_range7
    }
    #[doc = "0x3e8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_range7(&self) -> &SEL_AG3_METRIC_RANGE7 {
        &self.sel_ag3_metric_range7
    }
    #[doc = "0x3ec - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_range7(&self) -> &SEL_AG4_METRIC_RANGE7 {
        &self.sel_ag4_metric_range7
    }
    #[doc = "0x3f0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_range7(&self) -> &SEL_AG5_METRIC_RANGE7 {
        &self.sel_ag5_metric_range7
    }
    #[doc = "0x3f4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_range7(&self) -> &SEL_AG6_METRIC_RANGE7 {
        &self.sel_ag6_metric_range7
    }
    #[doc = "0x3f8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_addr_mask0(&self) -> &SEL_AG0_RD_ADDR_MASK0 {
        &self.sel_ag0_rd_addr_mask0
    }
    #[doc = "0x3fc - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_addr_mask0(&self) -> &SEL_AG1_RD_ADDR_MASK0 {
        &self.sel_ag1_rd_addr_mask0
    }
    #[doc = "0x400 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_addr_mask0(&self) -> &SEL_AG2_RD_ADDR_MASK0 {
        &self.sel_ag2_rd_addr_mask0
    }
    #[doc = "0x404 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_addr_mask0(&self) -> &SEL_AG3_RD_ADDR_MASK0 {
        &self.sel_ag3_rd_addr_mask0
    }
    #[doc = "0x408 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_addr_mask0(&self) -> &SEL_AG4_RD_ADDR_MASK0 {
        &self.sel_ag4_rd_addr_mask0
    }
    #[doc = "0x40c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_addr_mask0(&self) -> &SEL_AG5_RD_ADDR_MASK0 {
        &self.sel_ag5_rd_addr_mask0
    }
    #[doc = "0x410 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_addr_mask0(&self) -> &SEL_AG6_RD_ADDR_MASK0 {
        &self.sel_ag6_rd_addr_mask0
    }
    #[doc = "0x414 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_addr_mask1(&self) -> &SEL_AG0_RD_ADDR_MASK1 {
        &self.sel_ag0_rd_addr_mask1
    }
    #[doc = "0x418 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_addr_mask1(&self) -> &SEL_AG1_RD_ADDR_MASK1 {
        &self.sel_ag1_rd_addr_mask1
    }
    #[doc = "0x41c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_addr_mask1(&self) -> &SEL_AG2_RD_ADDR_MASK1 {
        &self.sel_ag2_rd_addr_mask1
    }
    #[doc = "0x420 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_addr_mask1(&self) -> &SEL_AG3_RD_ADDR_MASK1 {
        &self.sel_ag3_rd_addr_mask1
    }
    #[doc = "0x424 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_addr_mask1(&self) -> &SEL_AG4_RD_ADDR_MASK1 {
        &self.sel_ag4_rd_addr_mask1
    }
    #[doc = "0x428 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_addr_mask1(&self) -> &SEL_AG5_RD_ADDR_MASK1 {
        &self.sel_ag5_rd_addr_mask1
    }
    #[doc = "0x42c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_addr_mask1(&self) -> &SEL_AG6_RD_ADDR_MASK1 {
        &self.sel_ag6_rd_addr_mask1
    }
    #[doc = "0x430 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_addr_mask2(&self) -> &SEL_AG0_RD_ADDR_MASK2 {
        &self.sel_ag0_rd_addr_mask2
    }
    #[doc = "0x434 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_addr_mask2(&self) -> &SEL_AG1_RD_ADDR_MASK2 {
        &self.sel_ag1_rd_addr_mask2
    }
    #[doc = "0x438 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_addr_mask2(&self) -> &SEL_AG2_RD_ADDR_MASK2 {
        &self.sel_ag2_rd_addr_mask2
    }
    #[doc = "0x43c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_addr_mask2(&self) -> &SEL_AG3_RD_ADDR_MASK2 {
        &self.sel_ag3_rd_addr_mask2
    }
    #[doc = "0x440 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_addr_mask2(&self) -> &SEL_AG4_RD_ADDR_MASK2 {
        &self.sel_ag4_rd_addr_mask2
    }
    #[doc = "0x444 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_addr_mask2(&self) -> &SEL_AG5_RD_ADDR_MASK2 {
        &self.sel_ag5_rd_addr_mask2
    }
    #[doc = "0x448 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_addr_mask2(&self) -> &SEL_AG6_RD_ADDR_MASK2 {
        &self.sel_ag6_rd_addr_mask2
    }
    #[doc = "0x44c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_addr_mask0(&self) -> &SEL_AG0_WR_ADDR_MASK0 {
        &self.sel_ag0_wr_addr_mask0
    }
    #[doc = "0x450 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_addr_mask0(&self) -> &SEL_AG1_WR_ADDR_MASK0 {
        &self.sel_ag1_wr_addr_mask0
    }
    #[doc = "0x454 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_addr_mask0(&self) -> &SEL_AG2_WR_ADDR_MASK0 {
        &self.sel_ag2_wr_addr_mask0
    }
    #[doc = "0x458 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_addr_mask0(&self) -> &SEL_AG3_WR_ADDR_MASK0 {
        &self.sel_ag3_wr_addr_mask0
    }
    #[doc = "0x45c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_addr_mask0(&self) -> &SEL_AG4_WR_ADDR_MASK0 {
        &self.sel_ag4_wr_addr_mask0
    }
    #[doc = "0x460 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_addr_mask0(&self) -> &SEL_AG5_WR_ADDR_MASK0 {
        &self.sel_ag5_wr_addr_mask0
    }
    #[doc = "0x464 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_addr_mask0(&self) -> &SEL_AG6_WR_ADDR_MASK0 {
        &self.sel_ag6_wr_addr_mask0
    }
    #[doc = "0x468 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_addr_mask1(&self) -> &SEL_AG0_WR_ADDR_MASK1 {
        &self.sel_ag0_wr_addr_mask1
    }
    #[doc = "0x46c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_addr_mask1(&self) -> &SEL_AG1_WR_ADDR_MASK1 {
        &self.sel_ag1_wr_addr_mask1
    }
    #[doc = "0x470 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_addr_mask1(&self) -> &SEL_AG2_WR_ADDR_MASK1 {
        &self.sel_ag2_wr_addr_mask1
    }
    #[doc = "0x474 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_addr_mask1(&self) -> &SEL_AG3_WR_ADDR_MASK1 {
        &self.sel_ag3_wr_addr_mask1
    }
    #[doc = "0x478 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_addr_mask1(&self) -> &SEL_AG4_WR_ADDR_MASK1 {
        &self.sel_ag4_wr_addr_mask1
    }
    #[doc = "0x47c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_addr_mask1(&self) -> &SEL_AG5_WR_ADDR_MASK1 {
        &self.sel_ag5_wr_addr_mask1
    }
    #[doc = "0x480 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_addr_mask1(&self) -> &SEL_AG6_WR_ADDR_MASK1 {
        &self.sel_ag6_wr_addr_mask1
    }
    #[doc = "0x484 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_addr_mask2(&self) -> &SEL_AG0_WR_ADDR_MASK2 {
        &self.sel_ag0_wr_addr_mask2
    }
    #[doc = "0x488 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_addr_mask2(&self) -> &SEL_AG1_WR_ADDR_MASK2 {
        &self.sel_ag1_wr_addr_mask2
    }
    #[doc = "0x48c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_addr_mask2(&self) -> &SEL_AG2_WR_ADDR_MASK2 {
        &self.sel_ag2_wr_addr_mask2
    }
    #[doc = "0x490 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_addr_mask2(&self) -> &SEL_AG3_WR_ADDR_MASK2 {
        &self.sel_ag3_wr_addr_mask2
    }
    #[doc = "0x494 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_addr_mask2(&self) -> &SEL_AG4_WR_ADDR_MASK2 {
        &self.sel_ag4_wr_addr_mask2
    }
    #[doc = "0x498 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_addr_mask2(&self) -> &SEL_AG5_WR_ADDR_MASK2 {
        &self.sel_ag5_wr_addr_mask2
    }
    #[doc = "0x49c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_addr_mask2(&self) -> &SEL_AG6_WR_ADDR_MASK2 {
        &self.sel_ag6_wr_addr_mask2
    }
    #[doc = "0x4a0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_addr_filter0(&self) -> &SEL_AG0_RD_ADDR_FILTER0 {
        &self.sel_ag0_rd_addr_filter0
    }
    #[doc = "0x4a4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_addr_filter0(&self) -> &SEL_AG1_RD_ADDR_FILTER0 {
        &self.sel_ag1_rd_addr_filter0
    }
    #[doc = "0x4a8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_addr_filter0(&self) -> &SEL_AG2_RD_ADDR_FILTER0 {
        &self.sel_ag2_rd_addr_filter0
    }
    #[doc = "0x4ac - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_addr_filter0(&self) -> &SEL_AG3_RD_ADDR_FILTER0 {
        &self.sel_ag3_rd_addr_filter0
    }
    #[doc = "0x4b0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_addr_filter0(&self) -> &SEL_AG4_RD_ADDR_FILTER0 {
        &self.sel_ag4_rd_addr_filter0
    }
    #[doc = "0x4b4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_addr_filter0(&self) -> &SEL_AG5_RD_ADDR_FILTER0 {
        &self.sel_ag5_rd_addr_filter0
    }
    #[doc = "0x4b8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_addr_filter0(&self) -> &SEL_AG6_RD_ADDR_FILTER0 {
        &self.sel_ag6_rd_addr_filter0
    }
    #[doc = "0x4bc - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_addr_filter1(&self) -> &SEL_AG0_RD_ADDR_FILTER1 {
        &self.sel_ag0_rd_addr_filter1
    }
    #[doc = "0x4c0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_addr_filter1(&self) -> &SEL_AG1_RD_ADDR_FILTER1 {
        &self.sel_ag1_rd_addr_filter1
    }
    #[doc = "0x4c4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_addr_filter1(&self) -> &SEL_AG2_RD_ADDR_FILTER1 {
        &self.sel_ag2_rd_addr_filter1
    }
    #[doc = "0x4c8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_addr_filter1(&self) -> &SEL_AG3_RD_ADDR_FILTER1 {
        &self.sel_ag3_rd_addr_filter1
    }
    #[doc = "0x4cc - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_addr_filter1(&self) -> &SEL_AG4_RD_ADDR_FILTER1 {
        &self.sel_ag4_rd_addr_filter1
    }
    #[doc = "0x4d0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_addr_filter1(&self) -> &SEL_AG5_RD_ADDR_FILTER1 {
        &self.sel_ag5_rd_addr_filter1
    }
    #[doc = "0x4d4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_addr_filter1(&self) -> &SEL_AG6_RD_ADDR_FILTER1 {
        &self.sel_ag6_rd_addr_filter1
    }
    #[doc = "0x4d8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_addr_filter2(&self) -> &SEL_AG0_RD_ADDR_FILTER2 {
        &self.sel_ag0_rd_addr_filter2
    }
    #[doc = "0x4dc - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_addr_filter2(&self) -> &SEL_AG1_RD_ADDR_FILTER2 {
        &self.sel_ag1_rd_addr_filter2
    }
    #[doc = "0x4e0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_addr_filter2(&self) -> &SEL_AG2_RD_ADDR_FILTER2 {
        &self.sel_ag2_rd_addr_filter2
    }
    #[doc = "0x4e4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_addr_filter2(&self) -> &SEL_AG3_RD_ADDR_FILTER2 {
        &self.sel_ag3_rd_addr_filter2
    }
    #[doc = "0x4e8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_addr_filter2(&self) -> &SEL_AG4_RD_ADDR_FILTER2 {
        &self.sel_ag4_rd_addr_filter2
    }
    #[doc = "0x4ec - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_addr_filter2(&self) -> &SEL_AG5_RD_ADDR_FILTER2 {
        &self.sel_ag5_rd_addr_filter2
    }
    #[doc = "0x4f0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_addr_filter2(&self) -> &SEL_AG6_RD_ADDR_FILTER2 {
        &self.sel_ag6_rd_addr_filter2
    }
    #[doc = "0x4f4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_addr_filter0(&self) -> &SEL_AG0_WR_ADDR_FILTER0 {
        &self.sel_ag0_wr_addr_filter0
    }
    #[doc = "0x4f8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_addr_filter0(&self) -> &SEL_AG1_WR_ADDR_FILTER0 {
        &self.sel_ag1_wr_addr_filter0
    }
    #[doc = "0x4fc - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_addr_filter0(&self) -> &SEL_AG2_WR_ADDR_FILTER0 {
        &self.sel_ag2_wr_addr_filter0
    }
    #[doc = "0x500 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_addr_filter0(&self) -> &SEL_AG3_WR_ADDR_FILTER0 {
        &self.sel_ag3_wr_addr_filter0
    }
    #[doc = "0x504 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_addr_filter0(&self) -> &SEL_AG4_WR_ADDR_FILTER0 {
        &self.sel_ag4_wr_addr_filter0
    }
    #[doc = "0x508 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_addr_filter0(&self) -> &SEL_AG5_WR_ADDR_FILTER0 {
        &self.sel_ag5_wr_addr_filter0
    }
    #[doc = "0x50c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_addr_filter0(&self) -> &SEL_AG6_WR_ADDR_FILTER0 {
        &self.sel_ag6_wr_addr_filter0
    }
    #[doc = "0x510 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_addr_filter1(&self) -> &SEL_AG0_WR_ADDR_FILTER1 {
        &self.sel_ag0_wr_addr_filter1
    }
    #[doc = "0x514 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_addr_filter1(&self) -> &SEL_AG1_WR_ADDR_FILTER1 {
        &self.sel_ag1_wr_addr_filter1
    }
    #[doc = "0x518 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_addr_filter1(&self) -> &SEL_AG2_WR_ADDR_FILTER1 {
        &self.sel_ag2_wr_addr_filter1
    }
    #[doc = "0x51c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_addr_filter1(&self) -> &SEL_AG3_WR_ADDR_FILTER1 {
        &self.sel_ag3_wr_addr_filter1
    }
    #[doc = "0x520 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_addr_filter1(&self) -> &SEL_AG4_WR_ADDR_FILTER1 {
        &self.sel_ag4_wr_addr_filter1
    }
    #[doc = "0x524 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_addr_filter1(&self) -> &SEL_AG5_WR_ADDR_FILTER1 {
        &self.sel_ag5_wr_addr_filter1
    }
    #[doc = "0x528 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_addr_filter1(&self) -> &SEL_AG6_WR_ADDR_FILTER1 {
        &self.sel_ag6_wr_addr_filter1
    }
    #[doc = "0x52c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_addr_filter2(&self) -> &SEL_AG0_WR_ADDR_FILTER2 {
        &self.sel_ag0_wr_addr_filter2
    }
    #[doc = "0x530 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_addr_filter2(&self) -> &SEL_AG1_WR_ADDR_FILTER2 {
        &self.sel_ag1_wr_addr_filter2
    }
    #[doc = "0x534 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_addr_filter2(&self) -> &SEL_AG2_WR_ADDR_FILTER2 {
        &self.sel_ag2_wr_addr_filter2
    }
    #[doc = "0x538 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_addr_filter2(&self) -> &SEL_AG3_WR_ADDR_FILTER2 {
        &self.sel_ag3_wr_addr_filter2
    }
    #[doc = "0x53c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_addr_filter2(&self) -> &SEL_AG4_WR_ADDR_FILTER2 {
        &self.sel_ag4_wr_addr_filter2
    }
    #[doc = "0x540 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_addr_filter2(&self) -> &SEL_AG5_WR_ADDR_FILTER2 {
        &self.sel_ag5_wr_addr_filter2
    }
    #[doc = "0x544 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_addr_filter2(&self) -> &SEL_AG6_WR_ADDR_FILTER2 {
        &self.sel_ag6_wr_addr_filter2
    }
    #[doc = "0x548 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_select1(&self) -> &SEL_AG0_METRIC_SELECT1 {
        &self.sel_ag0_metric_select1
    }
    #[doc = "0x54c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_select1(&self) -> &SEL_AG1_METRIC_SELECT1 {
        &self.sel_ag1_metric_select1
    }
    #[doc = "0x550 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_select1(&self) -> &SEL_AG2_METRIC_SELECT1 {
        &self.sel_ag2_metric_select1
    }
    #[doc = "0x554 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_select1(&self) -> &SEL_AG3_METRIC_SELECT1 {
        &self.sel_ag3_metric_select1
    }
    #[doc = "0x558 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_select1(&self) -> &SEL_AG4_METRIC_SELECT1 {
        &self.sel_ag4_metric_select1
    }
    #[doc = "0x55c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_select1(&self) -> &SEL_AG5_METRIC_SELECT1 {
        &self.sel_ag5_metric_select1
    }
    #[doc = "0x560 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_select1(&self) -> &SEL_AG6_METRIC_SELECT1 {
        &self.sel_ag6_metric_select1
    }
    #[doc = "0x564 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_metric_select2(&self) -> &SEL_AG0_METRIC_SELECT2 {
        &self.sel_ag0_metric_select2
    }
    #[doc = "0x568 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_metric_select2(&self) -> &SEL_AG1_METRIC_SELECT2 {
        &self.sel_ag1_metric_select2
    }
    #[doc = "0x56c - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_metric_select2(&self) -> &SEL_AG2_METRIC_SELECT2 {
        &self.sel_ag2_metric_select2
    }
    #[doc = "0x570 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_metric_select2(&self) -> &SEL_AG3_METRIC_SELECT2 {
        &self.sel_ag3_metric_select2
    }
    #[doc = "0x574 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_metric_select2(&self) -> &SEL_AG4_METRIC_SELECT2 {
        &self.sel_ag4_metric_select2
    }
    #[doc = "0x578 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_metric_select2(&self) -> &SEL_AG5_METRIC_SELECT2 {
        &self.sel_ag5_metric_select2
    }
    #[doc = "0x57c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_metric_select2(&self) -> &SEL_AG6_METRIC_SELECT2 {
        &self.sel_ag6_metric_select2
    }
    #[doc = "0x580 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_rd_addr_region_sel(&self) -> &SEL_AG_RD_ADDR_REGION_SEL {
        &self.sel_ag_rd_addr_region_sel
    }
    #[doc = "0x584 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_wr_addr_region_sel(&self) -> &SEL_AG_WR_ADDR_REGION_SEL {
        &self.sel_ag_wr_addr_region_sel
    }
    #[doc = "0x588 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_addr_filter_en(&self) -> &SEL_AG_ADDR_FILTER_EN {
        &self.sel_ag_addr_filter_en
    }
    #[doc = "0x58c - reserved"]
    #[inline(always)]
    pub const fn sel_ag_sw_record_stop_en(&self) -> &SEL_AG_SW_RECORD_STOP_EN {
        &self.sel_ag_sw_record_stop_en
    }
    #[doc = "0x590 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_sw_record_stop_clr(&self) -> &SEL_AG_SW_RECORD_STOP_CLR {
        &self.sel_ag_sw_record_stop_clr
    }
    #[doc = "0x594 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_ins_bandw_test_en(&self) -> &SEL_AG_INS_BANDW_TEST_EN {
        &self.sel_ag_ins_bandw_test_en
    }
    #[doc = "0x598 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_sw_record_stop_data_limit(&self) -> &SEL_AG0_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag0_sw_record_stop_data_limit
    }
    #[doc = "0x59c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_sw_record_stop_data_limit(&self) -> &SEL_AG1_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag1_sw_record_stop_data_limit
    }
    #[doc = "0x5a0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_sw_record_stop_data_limit(&self) -> &SEL_AG2_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag2_sw_record_stop_data_limit
    }
    #[doc = "0x5a4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_sw_record_stop_data_limit(&self) -> &SEL_AG3_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag3_sw_record_stop_data_limit
    }
    #[doc = "0x5a8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_sw_record_stop_data_limit(&self) -> &SEL_AG4_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag4_sw_record_stop_data_limit
    }
    #[doc = "0x5ac - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_sw_record_stop_data_limit(&self) -> &SEL_AG5_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag5_sw_record_stop_data_limit
    }
    #[doc = "0x5b0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_sw_record_stop_data_limit(&self) -> &SEL_AG6_SW_RECORD_STOP_DATA_LIMIT {
        &self.sel_ag6_sw_record_stop_data_limit
    }
    #[doc = "0x5b4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_id_mask(&self) -> &SEL_AG0_ID_MASK {
        &self.sel_ag0_id_mask
    }
    #[doc = "0x5b8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_id_mask(&self) -> &SEL_AG1_ID_MASK {
        &self.sel_ag1_id_mask
    }
    #[doc = "0x5bc - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_id_mask(&self) -> &SEL_AG2_ID_MASK {
        &self.sel_ag2_id_mask
    }
    #[doc = "0x5c0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_id_mask(&self) -> &SEL_AG3_ID_MASK {
        &self.sel_ag3_id_mask
    }
    #[doc = "0x5c4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_id_mask(&self) -> &SEL_AG4_ID_MASK {
        &self.sel_ag4_id_mask
    }
    #[doc = "0x5c8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_id_mask(&self) -> &SEL_AG5_ID_MASK {
        &self.sel_ag5_id_mask
    }
    #[doc = "0x5cc - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_id_mask(&self) -> &SEL_AG6_ID_MASK {
        &self.sel_ag6_id_mask
    }
    #[doc = "0x5d0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_id_filter(&self) -> &SEL_AG0_ID_FILTER {
        &self.sel_ag0_id_filter
    }
    #[doc = "0x5d4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_id_filter(&self) -> &SEL_AG1_ID_FILTER {
        &self.sel_ag1_id_filter
    }
    #[doc = "0x5d8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_id_filter(&self) -> &SEL_AG2_ID_FILTER {
        &self.sel_ag2_id_filter
    }
    #[doc = "0x5dc - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_id_filter(&self) -> &SEL_AG3_ID_FILTER {
        &self.sel_ag3_id_filter
    }
    #[doc = "0x5e0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_id_filter(&self) -> &SEL_AG4_ID_FILTER {
        &self.sel_ag4_id_filter
    }
    #[doc = "0x5e4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_id_filter(&self) -> &SEL_AG5_ID_FILTER {
        &self.sel_ag5_id_filter
    }
    #[doc = "0x5e8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_id_filter(&self) -> &SEL_AG6_ID_FILTER {
        &self.sel_ag6_id_filter
    }
    #[doc = "0x5ec - reserved"]
    #[inline(always)]
    pub const fn sel_ag_bandw_test_en(&self) -> &SEL_AG_BANDW_TEST_EN {
        &self.sel_ag_bandw_test_en
    }
    #[doc = "0x5f0 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_bandw_test_stop(&self) -> &SEL_AG_BANDW_TEST_STOP {
        &self.sel_ag_bandw_test_stop
    }
    #[doc = "0x5f4 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_bandw_trigger_in_sel(&self) -> &SEL_AG0_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag0_bandw_trigger_in_sel
    }
    #[doc = "0x5f8 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_bandw_trigger_in_sel(&self) -> &SEL_AG1_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag1_bandw_trigger_in_sel
    }
    #[doc = "0x5fc - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_bandw_trigger_in_sel(&self) -> &SEL_AG2_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag2_bandw_trigger_in_sel
    }
    #[doc = "0x600 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_bandw_trigger_in_sel(&self) -> &SEL_AG3_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag3_bandw_trigger_in_sel
    }
    #[doc = "0x604 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_bandw_trigger_in_sel(&self) -> &SEL_AG4_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag4_bandw_trigger_in_sel
    }
    #[doc = "0x608 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_bandw_trigger_in_sel(&self) -> &SEL_AG5_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag5_bandw_trigger_in_sel
    }
    #[doc = "0x60c - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_bandw_trigger_in_sel(&self) -> &SEL_AG6_BANDW_TRIGGER_IN_SEL {
        &self.sel_ag6_bandw_trigger_in_sel
    }
    #[doc = "0x610 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG0_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag0_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x614 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag1_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x618 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG2_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag2_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x61c - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG3_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag3_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x620 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG4_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag4_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x624 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG5_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag5_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x628 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_wr_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG6_WR_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag6_wr_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x62c - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG0_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag0_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x630 - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG1_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag1_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x634 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG2_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag2_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x638 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG3_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag3_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x63c - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG4_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag4_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x640 - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG5_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag5_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x644 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_rd_bandw_cnt_valid_strobe_num(
        &self,
    ) -> &SEL_AG6_RD_BANDW_CNT_VALID_STROBE_NUM {
        &self.sel_ag6_rd_bandw_cnt_valid_strobe_num
    }
    #[doc = "0x648 - reserved"]
    #[inline(always)]
    pub const fn sel_ag0_ins_bandw_time_thr(&self) -> &SEL_AG0_INS_BANDW_TIME_THR {
        &self.sel_ag0_ins_bandw_time_thr
    }
    #[doc = "0x64c - reserved"]
    #[inline(always)]
    pub const fn sel_ag1_ins_bandw_time_thr(&self) -> &SEL_AG1_INS_BANDW_TIME_THR {
        &self.sel_ag1_ins_bandw_time_thr
    }
    #[doc = "0x650 - reserved"]
    #[inline(always)]
    pub const fn sel_ag2_ins_bandw_time_thr(&self) -> &SEL_AG2_INS_BANDW_TIME_THR {
        &self.sel_ag2_ins_bandw_time_thr
    }
    #[doc = "0x654 - reserved"]
    #[inline(always)]
    pub const fn sel_ag3_ins_bandw_time_thr(&self) -> &SEL_AG3_INS_BANDW_TIME_THR {
        &self.sel_ag3_ins_bandw_time_thr
    }
    #[doc = "0x658 - reserved"]
    #[inline(always)]
    pub const fn sel_ag4_ins_bandw_time_thr(&self) -> &SEL_AG4_INS_BANDW_TIME_THR {
        &self.sel_ag4_ins_bandw_time_thr
    }
    #[doc = "0x65c - reserved"]
    #[inline(always)]
    pub const fn sel_ag5_ins_bandw_time_thr(&self) -> &SEL_AG5_INS_BANDW_TIME_THR {
        &self.sel_ag5_ins_bandw_time_thr
    }
    #[doc = "0x660 - reserved"]
    #[inline(always)]
    pub const fn sel_ag6_ins_bandw_time_thr(&self) -> &SEL_AG6_INS_BANDW_TIME_THR {
        &self.sel_ag6_ins_bandw_time_thr
    }
    #[doc = "0x664 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_int_raw(&self) -> &SEL_AG_INT_RAW {
        &self.sel_ag_int_raw
    }
    #[doc = "0x668 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_int_st(&self) -> &SEL_AG_INT_ST {
        &self.sel_ag_int_st
    }
    #[doc = "0x66c - reserved"]
    #[inline(always)]
    pub const fn sel_ag_int_ena(&self) -> &SEL_AG_INT_ENA {
        &self.sel_ag_int_ena
    }
    #[doc = "0x670 - reserved"]
    #[inline(always)]
    pub const fn sel_ag_int_clr(&self) -> &SEL_AG_INT_CLR {
        &self.sel_ag_int_clr
    }
    #[doc = "0x674 - reserved"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CLK_EN (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "reserved"]
pub mod clk_en;
#[doc = "AGENT_SELECT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`agent_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agent_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@agent_select`] module"]
pub type AGENT_SELECT = crate::Reg<agent_select::AGENT_SELECT_SPEC>;
#[doc = "reserved"]
pub mod agent_select;
#[doc = "SEL_AG0_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter0`] module"]
pub type SEL_AG0_COUNTER0 = crate::Reg<sel_ag0_counter0::SEL_AG0_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter0;
#[doc = "SEL_AG1_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter0`] module"]
pub type SEL_AG1_COUNTER0 = crate::Reg<sel_ag1_counter0::SEL_AG1_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter0;
#[doc = "SEL_AG2_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter0`] module"]
pub type SEL_AG2_COUNTER0 = crate::Reg<sel_ag2_counter0::SEL_AG2_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter0;
#[doc = "SEL_AG3_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter0`] module"]
pub type SEL_AG3_COUNTER0 = crate::Reg<sel_ag3_counter0::SEL_AG3_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter0;
#[doc = "SEL_AG4_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter0`] module"]
pub type SEL_AG4_COUNTER0 = crate::Reg<sel_ag4_counter0::SEL_AG4_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter0;
#[doc = "SEL_AG5_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter0`] module"]
pub type SEL_AG5_COUNTER0 = crate::Reg<sel_ag5_counter0::SEL_AG5_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter0;
#[doc = "SEL_AG6_COUNTER0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter0`] module"]
pub type SEL_AG6_COUNTER0 = crate::Reg<sel_ag6_counter0::SEL_AG6_COUNTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter0;
#[doc = "SEL_AG0_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter1`] module"]
pub type SEL_AG0_COUNTER1 = crate::Reg<sel_ag0_counter1::SEL_AG0_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter1;
#[doc = "SEL_AG1_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter1`] module"]
pub type SEL_AG1_COUNTER1 = crate::Reg<sel_ag1_counter1::SEL_AG1_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter1;
#[doc = "SEL_AG2_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter1`] module"]
pub type SEL_AG2_COUNTER1 = crate::Reg<sel_ag2_counter1::SEL_AG2_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter1;
#[doc = "SEL_AG3_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter1`] module"]
pub type SEL_AG3_COUNTER1 = crate::Reg<sel_ag3_counter1::SEL_AG3_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter1;
#[doc = "SEL_AG4_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter1`] module"]
pub type SEL_AG4_COUNTER1 = crate::Reg<sel_ag4_counter1::SEL_AG4_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter1;
#[doc = "SEL_AG5_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter1`] module"]
pub type SEL_AG5_COUNTER1 = crate::Reg<sel_ag5_counter1::SEL_AG5_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter1;
#[doc = "SEL_AG6_COUNTER1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter1`] module"]
pub type SEL_AG6_COUNTER1 = crate::Reg<sel_ag6_counter1::SEL_AG6_COUNTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter1;
#[doc = "SEL_AG0_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter2`] module"]
pub type SEL_AG0_COUNTER2 = crate::Reg<sel_ag0_counter2::SEL_AG0_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter2;
#[doc = "SEL_AG1_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter2`] module"]
pub type SEL_AG1_COUNTER2 = crate::Reg<sel_ag1_counter2::SEL_AG1_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter2;
#[doc = "SEL_AG2_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter2`] module"]
pub type SEL_AG2_COUNTER2 = crate::Reg<sel_ag2_counter2::SEL_AG2_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter2;
#[doc = "SEL_AG3_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter2`] module"]
pub type SEL_AG3_COUNTER2 = crate::Reg<sel_ag3_counter2::SEL_AG3_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter2;
#[doc = "SEL_AG4_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter2`] module"]
pub type SEL_AG4_COUNTER2 = crate::Reg<sel_ag4_counter2::SEL_AG4_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter2;
#[doc = "SEL_AG5_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter2`] module"]
pub type SEL_AG5_COUNTER2 = crate::Reg<sel_ag5_counter2::SEL_AG5_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter2;
#[doc = "SEL_AG6_COUNTER2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter2`] module"]
pub type SEL_AG6_COUNTER2 = crate::Reg<sel_ag6_counter2::SEL_AG6_COUNTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter2;
#[doc = "SEL_AG0_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter3`] module"]
pub type SEL_AG0_COUNTER3 = crate::Reg<sel_ag0_counter3::SEL_AG0_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter3;
#[doc = "SEL_AG1_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter3`] module"]
pub type SEL_AG1_COUNTER3 = crate::Reg<sel_ag1_counter3::SEL_AG1_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter3;
#[doc = "SEL_AG2_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter3`] module"]
pub type SEL_AG2_COUNTER3 = crate::Reg<sel_ag2_counter3::SEL_AG2_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter3;
#[doc = "SEL_AG3_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter3`] module"]
pub type SEL_AG3_COUNTER3 = crate::Reg<sel_ag3_counter3::SEL_AG3_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter3;
#[doc = "SEL_AG4_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter3`] module"]
pub type SEL_AG4_COUNTER3 = crate::Reg<sel_ag4_counter3::SEL_AG4_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter3;
#[doc = "SEL_AG5_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter3`] module"]
pub type SEL_AG5_COUNTER3 = crate::Reg<sel_ag5_counter3::SEL_AG5_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter3;
#[doc = "SEL_AG6_COUNTER3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter3`] module"]
pub type SEL_AG6_COUNTER3 = crate::Reg<sel_ag6_counter3::SEL_AG6_COUNTER3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter3;
#[doc = "SEL_AG0_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter4`] module"]
pub type SEL_AG0_COUNTER4 = crate::Reg<sel_ag0_counter4::SEL_AG0_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter4;
#[doc = "SEL_AG1_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter4`] module"]
pub type SEL_AG1_COUNTER4 = crate::Reg<sel_ag1_counter4::SEL_AG1_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter4;
#[doc = "SEL_AG2_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter4`] module"]
pub type SEL_AG2_COUNTER4 = crate::Reg<sel_ag2_counter4::SEL_AG2_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter4;
#[doc = "SEL_AG3_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter4`] module"]
pub type SEL_AG3_COUNTER4 = crate::Reg<sel_ag3_counter4::SEL_AG3_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter4;
#[doc = "SEL_AG4_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter4`] module"]
pub type SEL_AG4_COUNTER4 = crate::Reg<sel_ag4_counter4::SEL_AG4_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter4;
#[doc = "SEL_AG5_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter4`] module"]
pub type SEL_AG5_COUNTER4 = crate::Reg<sel_ag5_counter4::SEL_AG5_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter4;
#[doc = "SEL_AG6_COUNTER4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter4`] module"]
pub type SEL_AG6_COUNTER4 = crate::Reg<sel_ag6_counter4::SEL_AG6_COUNTER4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter4;
#[doc = "SEL_AG0_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter5`] module"]
pub type SEL_AG0_COUNTER5 = crate::Reg<sel_ag0_counter5::SEL_AG0_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter5;
#[doc = "SEL_AG1_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter5`] module"]
pub type SEL_AG1_COUNTER5 = crate::Reg<sel_ag1_counter5::SEL_AG1_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter5;
#[doc = "SEL_AG2_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter5`] module"]
pub type SEL_AG2_COUNTER5 = crate::Reg<sel_ag2_counter5::SEL_AG2_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter5;
#[doc = "SEL_AG3_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter5`] module"]
pub type SEL_AG3_COUNTER5 = crate::Reg<sel_ag3_counter5::SEL_AG3_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter5;
#[doc = "SEL_AG4_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter5`] module"]
pub type SEL_AG4_COUNTER5 = crate::Reg<sel_ag4_counter5::SEL_AG4_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter5;
#[doc = "SEL_AG5_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter5`] module"]
pub type SEL_AG5_COUNTER5 = crate::Reg<sel_ag5_counter5::SEL_AG5_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter5;
#[doc = "SEL_AG6_COUNTER5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter5`] module"]
pub type SEL_AG6_COUNTER5 = crate::Reg<sel_ag6_counter5::SEL_AG6_COUNTER5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter5;
#[doc = "SEL_AG0_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter6`] module"]
pub type SEL_AG0_COUNTER6 = crate::Reg<sel_ag0_counter6::SEL_AG0_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter6;
#[doc = "SEL_AG1_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter6`] module"]
pub type SEL_AG1_COUNTER6 = crate::Reg<sel_ag1_counter6::SEL_AG1_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter6;
#[doc = "SEL_AG2_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter6`] module"]
pub type SEL_AG2_COUNTER6 = crate::Reg<sel_ag2_counter6::SEL_AG2_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter6;
#[doc = "SEL_AG3_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter6`] module"]
pub type SEL_AG3_COUNTER6 = crate::Reg<sel_ag3_counter6::SEL_AG3_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter6;
#[doc = "SEL_AG4_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter6`] module"]
pub type SEL_AG4_COUNTER6 = crate::Reg<sel_ag4_counter6::SEL_AG4_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter6;
#[doc = "SEL_AG5_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter6`] module"]
pub type SEL_AG5_COUNTER6 = crate::Reg<sel_ag5_counter6::SEL_AG5_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter6;
#[doc = "SEL_AG6_COUNTER6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter6`] module"]
pub type SEL_AG6_COUNTER6 = crate::Reg<sel_ag6_counter6::SEL_AG6_COUNTER6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter6;
#[doc = "SEL_AG0_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_counter7`] module"]
pub type SEL_AG0_COUNTER7 = crate::Reg<sel_ag0_counter7::SEL_AG0_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_counter7;
#[doc = "SEL_AG1_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_counter7`] module"]
pub type SEL_AG1_COUNTER7 = crate::Reg<sel_ag1_counter7::SEL_AG1_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_counter7;
#[doc = "SEL_AG2_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_counter7`] module"]
pub type SEL_AG2_COUNTER7 = crate::Reg<sel_ag2_counter7::SEL_AG2_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_counter7;
#[doc = "SEL_AG3_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_counter7`] module"]
pub type SEL_AG3_COUNTER7 = crate::Reg<sel_ag3_counter7::SEL_AG3_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_counter7;
#[doc = "SEL_AG4_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_counter7`] module"]
pub type SEL_AG4_COUNTER7 = crate::Reg<sel_ag4_counter7::SEL_AG4_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_counter7;
#[doc = "SEL_AG5_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_counter7`] module"]
pub type SEL_AG5_COUNTER7 = crate::Reg<sel_ag5_counter7::SEL_AG5_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_counter7;
#[doc = "SEL_AG6_COUNTER7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_counter7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_counter7`] module"]
pub type SEL_AG6_COUNTER7 = crate::Reg<sel_ag6_counter7::SEL_AG6_COUNTER7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_counter7;
#[doc = "SEL_AG0_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range0`] module"]
pub type SEL_AG0_RANGE0 = crate::Reg<sel_ag0_range0::SEL_AG0_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range0;
#[doc = "SEL_AG1_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range0`] module"]
pub type SEL_AG1_RANGE0 = crate::Reg<sel_ag1_range0::SEL_AG1_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range0;
#[doc = "SEL_AG2_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range0`] module"]
pub type SEL_AG2_RANGE0 = crate::Reg<sel_ag2_range0::SEL_AG2_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range0;
#[doc = "SEL_AG3_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range0`] module"]
pub type SEL_AG3_RANGE0 = crate::Reg<sel_ag3_range0::SEL_AG3_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range0;
#[doc = "SEL_AG4_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range0`] module"]
pub type SEL_AG4_RANGE0 = crate::Reg<sel_ag4_range0::SEL_AG4_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range0;
#[doc = "SEL_AG5_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range0`] module"]
pub type SEL_AG5_RANGE0 = crate::Reg<sel_ag5_range0::SEL_AG5_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range0;
#[doc = "SEL_AG6_RANGE0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range0`] module"]
pub type SEL_AG6_RANGE0 = crate::Reg<sel_ag6_range0::SEL_AG6_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range0;
#[doc = "SEL_AG0_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range1`] module"]
pub type SEL_AG0_RANGE1 = crate::Reg<sel_ag0_range1::SEL_AG0_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range1;
#[doc = "SEL_AG1_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range1`] module"]
pub type SEL_AG1_RANGE1 = crate::Reg<sel_ag1_range1::SEL_AG1_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range1;
#[doc = "SEL_AG2_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range1`] module"]
pub type SEL_AG2_RANGE1 = crate::Reg<sel_ag2_range1::SEL_AG2_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range1;
#[doc = "SEL_AG3_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range1`] module"]
pub type SEL_AG3_RANGE1 = crate::Reg<sel_ag3_range1::SEL_AG3_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range1;
#[doc = "SEL_AG4_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range1`] module"]
pub type SEL_AG4_RANGE1 = crate::Reg<sel_ag4_range1::SEL_AG4_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range1;
#[doc = "SEL_AG5_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range1`] module"]
pub type SEL_AG5_RANGE1 = crate::Reg<sel_ag5_range1::SEL_AG5_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range1;
#[doc = "SEL_AG6_RANGE1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range1`] module"]
pub type SEL_AG6_RANGE1 = crate::Reg<sel_ag6_range1::SEL_AG6_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range1;
#[doc = "SEL_AG0_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range2`] module"]
pub type SEL_AG0_RANGE2 = crate::Reg<sel_ag0_range2::SEL_AG0_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range2;
#[doc = "SEL_AG1_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range2`] module"]
pub type SEL_AG1_RANGE2 = crate::Reg<sel_ag1_range2::SEL_AG1_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range2;
#[doc = "SEL_AG2_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range2`] module"]
pub type SEL_AG2_RANGE2 = crate::Reg<sel_ag2_range2::SEL_AG2_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range2;
#[doc = "SEL_AG3_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range2`] module"]
pub type SEL_AG3_RANGE2 = crate::Reg<sel_ag3_range2::SEL_AG3_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range2;
#[doc = "SEL_AG4_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range2`] module"]
pub type SEL_AG4_RANGE2 = crate::Reg<sel_ag4_range2::SEL_AG4_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range2;
#[doc = "SEL_AG5_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range2`] module"]
pub type SEL_AG5_RANGE2 = crate::Reg<sel_ag5_range2::SEL_AG5_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range2;
#[doc = "SEL_AG6_RANGE2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range2`] module"]
pub type SEL_AG6_RANGE2 = crate::Reg<sel_ag6_range2::SEL_AG6_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range2;
#[doc = "SEL_AG0_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range3`] module"]
pub type SEL_AG0_RANGE3 = crate::Reg<sel_ag0_range3::SEL_AG0_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range3;
#[doc = "SEL_AG1_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range3`] module"]
pub type SEL_AG1_RANGE3 = crate::Reg<sel_ag1_range3::SEL_AG1_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range3;
#[doc = "SEL_AG2_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range3`] module"]
pub type SEL_AG2_RANGE3 = crate::Reg<sel_ag2_range3::SEL_AG2_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range3;
#[doc = "SEL_AG3_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range3`] module"]
pub type SEL_AG3_RANGE3 = crate::Reg<sel_ag3_range3::SEL_AG3_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range3;
#[doc = "SEL_AG4_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range3`] module"]
pub type SEL_AG4_RANGE3 = crate::Reg<sel_ag4_range3::SEL_AG4_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range3;
#[doc = "SEL_AG5_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range3`] module"]
pub type SEL_AG5_RANGE3 = crate::Reg<sel_ag5_range3::SEL_AG5_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range3;
#[doc = "SEL_AG6_RANGE3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range3`] module"]
pub type SEL_AG6_RANGE3 = crate::Reg<sel_ag6_range3::SEL_AG6_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range3;
#[doc = "SEL_AG0_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range4`] module"]
pub type SEL_AG0_RANGE4 = crate::Reg<sel_ag0_range4::SEL_AG0_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range4;
#[doc = "SEL_AG1_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range4`] module"]
pub type SEL_AG1_RANGE4 = crate::Reg<sel_ag1_range4::SEL_AG1_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range4;
#[doc = "SEL_AG2_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range4`] module"]
pub type SEL_AG2_RANGE4 = crate::Reg<sel_ag2_range4::SEL_AG2_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range4;
#[doc = "SEL_AG3_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range4`] module"]
pub type SEL_AG3_RANGE4 = crate::Reg<sel_ag3_range4::SEL_AG3_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range4;
#[doc = "SEL_AG4_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range4`] module"]
pub type SEL_AG4_RANGE4 = crate::Reg<sel_ag4_range4::SEL_AG4_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range4;
#[doc = "SEL_AG5_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range4`] module"]
pub type SEL_AG5_RANGE4 = crate::Reg<sel_ag5_range4::SEL_AG5_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range4;
#[doc = "SEL_AG6_RANGE4 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range4`] module"]
pub type SEL_AG6_RANGE4 = crate::Reg<sel_ag6_range4::SEL_AG6_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range4;
#[doc = "SEL_AG0_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range5`] module"]
pub type SEL_AG0_RANGE5 = crate::Reg<sel_ag0_range5::SEL_AG0_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range5;
#[doc = "SEL_AG1_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range5`] module"]
pub type SEL_AG1_RANGE5 = crate::Reg<sel_ag1_range5::SEL_AG1_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range5;
#[doc = "SEL_AG2_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range5`] module"]
pub type SEL_AG2_RANGE5 = crate::Reg<sel_ag2_range5::SEL_AG2_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range5;
#[doc = "SEL_AG3_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range5`] module"]
pub type SEL_AG3_RANGE5 = crate::Reg<sel_ag3_range5::SEL_AG3_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range5;
#[doc = "SEL_AG4_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range5`] module"]
pub type SEL_AG4_RANGE5 = crate::Reg<sel_ag4_range5::SEL_AG4_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range5;
#[doc = "SEL_AG5_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range5`] module"]
pub type SEL_AG5_RANGE5 = crate::Reg<sel_ag5_range5::SEL_AG5_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range5;
#[doc = "SEL_AG6_RANGE5 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range5`] module"]
pub type SEL_AG6_RANGE5 = crate::Reg<sel_ag6_range5::SEL_AG6_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range5;
#[doc = "SEL_AG0_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range6`] module"]
pub type SEL_AG0_RANGE6 = crate::Reg<sel_ag0_range6::SEL_AG0_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range6;
#[doc = "SEL_AG1_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range6`] module"]
pub type SEL_AG1_RANGE6 = crate::Reg<sel_ag1_range6::SEL_AG1_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range6;
#[doc = "SEL_AG2_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range6`] module"]
pub type SEL_AG2_RANGE6 = crate::Reg<sel_ag2_range6::SEL_AG2_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range6;
#[doc = "SEL_AG3_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range6`] module"]
pub type SEL_AG3_RANGE6 = crate::Reg<sel_ag3_range6::SEL_AG3_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range6;
#[doc = "SEL_AG4_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range6`] module"]
pub type SEL_AG4_RANGE6 = crate::Reg<sel_ag4_range6::SEL_AG4_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range6;
#[doc = "SEL_AG5_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range6`] module"]
pub type SEL_AG5_RANGE6 = crate::Reg<sel_ag5_range6::SEL_AG5_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range6;
#[doc = "SEL_AG6_RANGE6 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range6`] module"]
pub type SEL_AG6_RANGE6 = crate::Reg<sel_ag6_range6::SEL_AG6_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range6;
#[doc = "SEL_AG0_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_range7`] module"]
pub type SEL_AG0_RANGE7 = crate::Reg<sel_ag0_range7::SEL_AG0_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_range7;
#[doc = "SEL_AG1_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_range7`] module"]
pub type SEL_AG1_RANGE7 = crate::Reg<sel_ag1_range7::SEL_AG1_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_range7;
#[doc = "SEL_AG2_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_range7`] module"]
pub type SEL_AG2_RANGE7 = crate::Reg<sel_ag2_range7::SEL_AG2_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_range7;
#[doc = "SEL_AG3_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_range7`] module"]
pub type SEL_AG3_RANGE7 = crate::Reg<sel_ag3_range7::SEL_AG3_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_range7;
#[doc = "SEL_AG4_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_range7`] module"]
pub type SEL_AG4_RANGE7 = crate::Reg<sel_ag4_range7::SEL_AG4_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_range7;
#[doc = "SEL_AG5_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_range7`] module"]
pub type SEL_AG5_RANGE7 = crate::Reg<sel_ag5_range7::SEL_AG5_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_range7;
#[doc = "SEL_AG6_RANGE7 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_range7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_range7`] module"]
pub type SEL_AG6_RANGE7 = crate::Reg<sel_ag6_range7::SEL_AG6_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_range7;
#[doc = "SEL_AG0_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_axi_info_record0`] module"]
pub type SEL_AG0_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag0_rd_axi_info_record0::SEL_AG0_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_axi_info_record0;
#[doc = "SEL_AG1_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_axi_info_record0`] module"]
pub type SEL_AG1_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag1_rd_axi_info_record0::SEL_AG1_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_axi_info_record0;
#[doc = "SEL_AG2_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_axi_info_record0`] module"]
pub type SEL_AG2_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag2_rd_axi_info_record0::SEL_AG2_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_axi_info_record0;
#[doc = "SEL_AG3_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_axi_info_record0`] module"]
pub type SEL_AG3_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag3_rd_axi_info_record0::SEL_AG3_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_axi_info_record0;
#[doc = "SEL_AG4_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_axi_info_record0`] module"]
pub type SEL_AG4_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag4_rd_axi_info_record0::SEL_AG4_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_axi_info_record0;
#[doc = "SEL_AG5_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_axi_info_record0`] module"]
pub type SEL_AG5_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag5_rd_axi_info_record0::SEL_AG5_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_axi_info_record0;
#[doc = "SEL_AG6_RD_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_axi_info_record0`] module"]
pub type SEL_AG6_RD_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag6_rd_axi_info_record0::SEL_AG6_RD_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_axi_info_record0;
#[doc = "SEL_AG0_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_axi_info_record1`] module"]
pub type SEL_AG0_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag0_rd_axi_info_record1::SEL_AG0_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_axi_info_record1;
#[doc = "SEL_AG1_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_axi_info_record1`] module"]
pub type SEL_AG1_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag1_rd_axi_info_record1::SEL_AG1_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_axi_info_record1;
#[doc = "SEL_AG2_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_axi_info_record1`] module"]
pub type SEL_AG2_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag2_rd_axi_info_record1::SEL_AG2_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_axi_info_record1;
#[doc = "SEL_AG3_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_axi_info_record1`] module"]
pub type SEL_AG3_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag3_rd_axi_info_record1::SEL_AG3_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_axi_info_record1;
#[doc = "SEL_AG4_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_axi_info_record1`] module"]
pub type SEL_AG4_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag4_rd_axi_info_record1::SEL_AG4_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_axi_info_record1;
#[doc = "SEL_AG5_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_axi_info_record1`] module"]
pub type SEL_AG5_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag5_rd_axi_info_record1::SEL_AG5_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_axi_info_record1;
#[doc = "SEL_AG6_RD_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_axi_info_record1`] module"]
pub type SEL_AG6_RD_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag6_rd_axi_info_record1::SEL_AG6_RD_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_axi_info_record1;
#[doc = "SEL_AG0_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_axi_info_record2`] module"]
pub type SEL_AG0_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag0_rd_axi_info_record2::SEL_AG0_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_axi_info_record2;
#[doc = "SEL_AG1_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_axi_info_record2`] module"]
pub type SEL_AG1_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag1_rd_axi_info_record2::SEL_AG1_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_axi_info_record2;
#[doc = "SEL_AG2_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_axi_info_record2`] module"]
pub type SEL_AG2_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag2_rd_axi_info_record2::SEL_AG2_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_axi_info_record2;
#[doc = "SEL_AG3_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_axi_info_record2`] module"]
pub type SEL_AG3_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag3_rd_axi_info_record2::SEL_AG3_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_axi_info_record2;
#[doc = "SEL_AG4_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_axi_info_record2`] module"]
pub type SEL_AG4_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag4_rd_axi_info_record2::SEL_AG4_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_axi_info_record2;
#[doc = "SEL_AG5_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_axi_info_record2`] module"]
pub type SEL_AG5_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag5_rd_axi_info_record2::SEL_AG5_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_axi_info_record2;
#[doc = "SEL_AG6_RD_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_axi_info_record2`] module"]
pub type SEL_AG6_RD_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag6_rd_axi_info_record2::SEL_AG6_RD_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_axi_info_record2;
#[doc = "SEL_AG0_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_axi_info_record3`] module"]
pub type SEL_AG0_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag0_rd_axi_info_record3::SEL_AG0_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_axi_info_record3;
#[doc = "SEL_AG1_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_axi_info_record3`] module"]
pub type SEL_AG1_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag1_rd_axi_info_record3::SEL_AG1_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_axi_info_record3;
#[doc = "SEL_AG2_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_axi_info_record3`] module"]
pub type SEL_AG2_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag2_rd_axi_info_record3::SEL_AG2_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_axi_info_record3;
#[doc = "SEL_AG3_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_axi_info_record3`] module"]
pub type SEL_AG3_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag3_rd_axi_info_record3::SEL_AG3_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_axi_info_record3;
#[doc = "SEL_AG4_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_axi_info_record3`] module"]
pub type SEL_AG4_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag4_rd_axi_info_record3::SEL_AG4_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_axi_info_record3;
#[doc = "SEL_AG5_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_axi_info_record3`] module"]
pub type SEL_AG5_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag5_rd_axi_info_record3::SEL_AG5_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_axi_info_record3;
#[doc = "SEL_AG6_RD_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_axi_info_record3`] module"]
pub type SEL_AG6_RD_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag6_rd_axi_info_record3::SEL_AG6_RD_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_axi_info_record3;
#[doc = "SEL_AG0_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_axi_info_record0`] module"]
pub type SEL_AG0_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag0_wr_axi_info_record0::SEL_AG0_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_axi_info_record0;
#[doc = "SEL_AG1_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_axi_info_record0`] module"]
pub type SEL_AG1_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag1_wr_axi_info_record0::SEL_AG1_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_axi_info_record0;
#[doc = "SEL_AG2_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_axi_info_record0`] module"]
pub type SEL_AG2_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag2_wr_axi_info_record0::SEL_AG2_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_axi_info_record0;
#[doc = "SEL_AG3_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_axi_info_record0`] module"]
pub type SEL_AG3_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag3_wr_axi_info_record0::SEL_AG3_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_axi_info_record0;
#[doc = "SEL_AG4_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_axi_info_record0`] module"]
pub type SEL_AG4_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag4_wr_axi_info_record0::SEL_AG4_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_axi_info_record0;
#[doc = "SEL_AG5_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_axi_info_record0`] module"]
pub type SEL_AG5_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag5_wr_axi_info_record0::SEL_AG5_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_axi_info_record0;
#[doc = "SEL_AG6_WR_AXI_INFO_RECORD0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_axi_info_record0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_axi_info_record0`] module"]
pub type SEL_AG6_WR_AXI_INFO_RECORD0 =
    crate::Reg<sel_ag6_wr_axi_info_record0::SEL_AG6_WR_AXI_INFO_RECORD0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_axi_info_record0;
#[doc = "SEL_AG0_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_axi_info_record1`] module"]
pub type SEL_AG0_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag0_wr_axi_info_record1::SEL_AG0_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_axi_info_record1;
#[doc = "SEL_AG1_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_axi_info_record1`] module"]
pub type SEL_AG1_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag1_wr_axi_info_record1::SEL_AG1_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_axi_info_record1;
#[doc = "SEL_AG2_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_axi_info_record1`] module"]
pub type SEL_AG2_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag2_wr_axi_info_record1::SEL_AG2_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_axi_info_record1;
#[doc = "SEL_AG3_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_axi_info_record1`] module"]
pub type SEL_AG3_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag3_wr_axi_info_record1::SEL_AG3_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_axi_info_record1;
#[doc = "SEL_AG4_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_axi_info_record1`] module"]
pub type SEL_AG4_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag4_wr_axi_info_record1::SEL_AG4_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_axi_info_record1;
#[doc = "SEL_AG5_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_axi_info_record1`] module"]
pub type SEL_AG5_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag5_wr_axi_info_record1::SEL_AG5_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_axi_info_record1;
#[doc = "SEL_AG6_WR_AXI_INFO_RECORD1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_axi_info_record1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_axi_info_record1`] module"]
pub type SEL_AG6_WR_AXI_INFO_RECORD1 =
    crate::Reg<sel_ag6_wr_axi_info_record1::SEL_AG6_WR_AXI_INFO_RECORD1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_axi_info_record1;
#[doc = "SEL_AG0_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_axi_info_record2`] module"]
pub type SEL_AG0_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag0_wr_axi_info_record2::SEL_AG0_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_axi_info_record2;
#[doc = "SEL_AG1_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_axi_info_record2`] module"]
pub type SEL_AG1_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag1_wr_axi_info_record2::SEL_AG1_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_axi_info_record2;
#[doc = "SEL_AG2_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_axi_info_record2`] module"]
pub type SEL_AG2_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag2_wr_axi_info_record2::SEL_AG2_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_axi_info_record2;
#[doc = "SEL_AG3_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_axi_info_record2`] module"]
pub type SEL_AG3_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag3_wr_axi_info_record2::SEL_AG3_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_axi_info_record2;
#[doc = "SEL_AG4_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_axi_info_record2`] module"]
pub type SEL_AG4_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag4_wr_axi_info_record2::SEL_AG4_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_axi_info_record2;
#[doc = "SEL_AG5_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_axi_info_record2`] module"]
pub type SEL_AG5_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag5_wr_axi_info_record2::SEL_AG5_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_axi_info_record2;
#[doc = "SEL_AG6_WR_AXI_INFO_RECORD2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_axi_info_record2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_axi_info_record2`] module"]
pub type SEL_AG6_WR_AXI_INFO_RECORD2 =
    crate::Reg<sel_ag6_wr_axi_info_record2::SEL_AG6_WR_AXI_INFO_RECORD2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_axi_info_record2;
#[doc = "SEL_AG0_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_axi_info_record3`] module"]
pub type SEL_AG0_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag0_wr_axi_info_record3::SEL_AG0_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_axi_info_record3;
#[doc = "SEL_AG1_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_axi_info_record3`] module"]
pub type SEL_AG1_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag1_wr_axi_info_record3::SEL_AG1_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_axi_info_record3;
#[doc = "SEL_AG2_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_axi_info_record3`] module"]
pub type SEL_AG2_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag2_wr_axi_info_record3::SEL_AG2_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_axi_info_record3;
#[doc = "SEL_AG3_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_axi_info_record3`] module"]
pub type SEL_AG3_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag3_wr_axi_info_record3::SEL_AG3_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_axi_info_record3;
#[doc = "SEL_AG4_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_axi_info_record3`] module"]
pub type SEL_AG4_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag4_wr_axi_info_record3::SEL_AG4_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_axi_info_record3;
#[doc = "SEL_AG5_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_axi_info_record3`] module"]
pub type SEL_AG5_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag5_wr_axi_info_record3::SEL_AG5_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_axi_info_record3;
#[doc = "SEL_AG6_WR_AXI_INFO_RECORD3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_axi_info_record3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_axi_info_record3`] module"]
pub type SEL_AG6_WR_AXI_INFO_RECORD3 =
    crate::Reg<sel_ag6_wr_axi_info_record3::SEL_AG6_WR_AXI_INFO_RECORD3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_axi_info_record3;
#[doc = "SEL_AG0_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_ins_bandw_data0`] module"]
pub type SEL_AG0_INS_BANDW_DATA0 =
    crate::Reg<sel_ag0_ins_bandw_data0::SEL_AG0_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_ins_bandw_data0;
#[doc = "SEL_AG1_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_ins_bandw_data0`] module"]
pub type SEL_AG1_INS_BANDW_DATA0 =
    crate::Reg<sel_ag1_ins_bandw_data0::SEL_AG1_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_ins_bandw_data0;
#[doc = "SEL_AG2_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_ins_bandw_data0`] module"]
pub type SEL_AG2_INS_BANDW_DATA0 =
    crate::Reg<sel_ag2_ins_bandw_data0::SEL_AG2_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_ins_bandw_data0;
#[doc = "SEL_AG3_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_ins_bandw_data0`] module"]
pub type SEL_AG3_INS_BANDW_DATA0 =
    crate::Reg<sel_ag3_ins_bandw_data0::SEL_AG3_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_ins_bandw_data0;
#[doc = "SEL_AG4_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_ins_bandw_data0`] module"]
pub type SEL_AG4_INS_BANDW_DATA0 =
    crate::Reg<sel_ag4_ins_bandw_data0::SEL_AG4_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_ins_bandw_data0;
#[doc = "SEL_AG5_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_ins_bandw_data0`] module"]
pub type SEL_AG5_INS_BANDW_DATA0 =
    crate::Reg<sel_ag5_ins_bandw_data0::SEL_AG5_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_ins_bandw_data0;
#[doc = "SEL_AG6_INS_BANDW_DATA0 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_ins_bandw_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_ins_bandw_data0`] module"]
pub type SEL_AG6_INS_BANDW_DATA0 =
    crate::Reg<sel_ag6_ins_bandw_data0::SEL_AG6_INS_BANDW_DATA0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_ins_bandw_data0;
#[doc = "SEL_AG0_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_ins_bandw_data1`] module"]
pub type SEL_AG0_INS_BANDW_DATA1 =
    crate::Reg<sel_ag0_ins_bandw_data1::SEL_AG0_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_ins_bandw_data1;
#[doc = "SEL_AG1_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_ins_bandw_data1`] module"]
pub type SEL_AG1_INS_BANDW_DATA1 =
    crate::Reg<sel_ag1_ins_bandw_data1::SEL_AG1_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_ins_bandw_data1;
#[doc = "SEL_AG2_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_ins_bandw_data1`] module"]
pub type SEL_AG2_INS_BANDW_DATA1 =
    crate::Reg<sel_ag2_ins_bandw_data1::SEL_AG2_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_ins_bandw_data1;
#[doc = "SEL_AG3_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_ins_bandw_data1`] module"]
pub type SEL_AG3_INS_BANDW_DATA1 =
    crate::Reg<sel_ag3_ins_bandw_data1::SEL_AG3_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_ins_bandw_data1;
#[doc = "SEL_AG4_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_ins_bandw_data1`] module"]
pub type SEL_AG4_INS_BANDW_DATA1 =
    crate::Reg<sel_ag4_ins_bandw_data1::SEL_AG4_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_ins_bandw_data1;
#[doc = "SEL_AG5_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_ins_bandw_data1`] module"]
pub type SEL_AG5_INS_BANDW_DATA1 =
    crate::Reg<sel_ag5_ins_bandw_data1::SEL_AG5_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_ins_bandw_data1;
#[doc = "SEL_AG6_INS_BANDW_DATA1 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_ins_bandw_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_ins_bandw_data1`] module"]
pub type SEL_AG6_INS_BANDW_DATA1 =
    crate::Reg<sel_ag6_ins_bandw_data1::SEL_AG6_INS_BANDW_DATA1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_ins_bandw_data1;
#[doc = "SEL_AG0_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_ins_bandw_data2`] module"]
pub type SEL_AG0_INS_BANDW_DATA2 =
    crate::Reg<sel_ag0_ins_bandw_data2::SEL_AG0_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_ins_bandw_data2;
#[doc = "SEL_AG1_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_ins_bandw_data2`] module"]
pub type SEL_AG1_INS_BANDW_DATA2 =
    crate::Reg<sel_ag1_ins_bandw_data2::SEL_AG1_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_ins_bandw_data2;
#[doc = "SEL_AG2_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_ins_bandw_data2`] module"]
pub type SEL_AG2_INS_BANDW_DATA2 =
    crate::Reg<sel_ag2_ins_bandw_data2::SEL_AG2_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_ins_bandw_data2;
#[doc = "SEL_AG3_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_ins_bandw_data2`] module"]
pub type SEL_AG3_INS_BANDW_DATA2 =
    crate::Reg<sel_ag3_ins_bandw_data2::SEL_AG3_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_ins_bandw_data2;
#[doc = "SEL_AG4_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_ins_bandw_data2`] module"]
pub type SEL_AG4_INS_BANDW_DATA2 =
    crate::Reg<sel_ag4_ins_bandw_data2::SEL_AG4_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_ins_bandw_data2;
#[doc = "SEL_AG5_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_ins_bandw_data2`] module"]
pub type SEL_AG5_INS_BANDW_DATA2 =
    crate::Reg<sel_ag5_ins_bandw_data2::SEL_AG5_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_ins_bandw_data2;
#[doc = "SEL_AG6_INS_BANDW_DATA2 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_ins_bandw_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_ins_bandw_data2`] module"]
pub type SEL_AG6_INS_BANDW_DATA2 =
    crate::Reg<sel_ag6_ins_bandw_data2::SEL_AG6_INS_BANDW_DATA2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_ins_bandw_data2;
#[doc = "SEL_AG0_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_ins_bandw_data3`] module"]
pub type SEL_AG0_INS_BANDW_DATA3 =
    crate::Reg<sel_ag0_ins_bandw_data3::SEL_AG0_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_ins_bandw_data3;
#[doc = "SEL_AG1_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_ins_bandw_data3`] module"]
pub type SEL_AG1_INS_BANDW_DATA3 =
    crate::Reg<sel_ag1_ins_bandw_data3::SEL_AG1_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_ins_bandw_data3;
#[doc = "SEL_AG2_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_ins_bandw_data3`] module"]
pub type SEL_AG2_INS_BANDW_DATA3 =
    crate::Reg<sel_ag2_ins_bandw_data3::SEL_AG2_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_ins_bandw_data3;
#[doc = "SEL_AG3_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_ins_bandw_data3`] module"]
pub type SEL_AG3_INS_BANDW_DATA3 =
    crate::Reg<sel_ag3_ins_bandw_data3::SEL_AG3_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_ins_bandw_data3;
#[doc = "SEL_AG4_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_ins_bandw_data3`] module"]
pub type SEL_AG4_INS_BANDW_DATA3 =
    crate::Reg<sel_ag4_ins_bandw_data3::SEL_AG4_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_ins_bandw_data3;
#[doc = "SEL_AG5_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_ins_bandw_data3`] module"]
pub type SEL_AG5_INS_BANDW_DATA3 =
    crate::Reg<sel_ag5_ins_bandw_data3::SEL_AG5_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_ins_bandw_data3;
#[doc = "SEL_AG6_INS_BANDW_DATA3 (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_ins_bandw_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_ins_bandw_data3`] module"]
pub type SEL_AG6_INS_BANDW_DATA3 =
    crate::Reg<sel_ag6_ins_bandw_data3::SEL_AG6_INS_BANDW_DATA3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_ins_bandw_data3;
#[doc = "SEL_AG0_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range0`] module"]
pub type SEL_AG0_METRIC_RANGE0 = crate::Reg<sel_ag0_metric_range0::SEL_AG0_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range0;
#[doc = "SEL_AG1_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range0`] module"]
pub type SEL_AG1_METRIC_RANGE0 = crate::Reg<sel_ag1_metric_range0::SEL_AG1_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range0;
#[doc = "SEL_AG2_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range0`] module"]
pub type SEL_AG2_METRIC_RANGE0 = crate::Reg<sel_ag2_metric_range0::SEL_AG2_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range0;
#[doc = "SEL_AG3_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range0`] module"]
pub type SEL_AG3_METRIC_RANGE0 = crate::Reg<sel_ag3_metric_range0::SEL_AG3_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range0;
#[doc = "SEL_AG4_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range0`] module"]
pub type SEL_AG4_METRIC_RANGE0 = crate::Reg<sel_ag4_metric_range0::SEL_AG4_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range0;
#[doc = "SEL_AG5_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range0`] module"]
pub type SEL_AG5_METRIC_RANGE0 = crate::Reg<sel_ag5_metric_range0::SEL_AG5_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range0;
#[doc = "SEL_AG6_METRIC_RANGE0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range0`] module"]
pub type SEL_AG6_METRIC_RANGE0 = crate::Reg<sel_ag6_metric_range0::SEL_AG6_METRIC_RANGE0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range0;
#[doc = "SEL_AG0_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range1`] module"]
pub type SEL_AG0_METRIC_RANGE1 = crate::Reg<sel_ag0_metric_range1::SEL_AG0_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range1;
#[doc = "SEL_AG1_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range1`] module"]
pub type SEL_AG1_METRIC_RANGE1 = crate::Reg<sel_ag1_metric_range1::SEL_AG1_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range1;
#[doc = "SEL_AG2_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range1`] module"]
pub type SEL_AG2_METRIC_RANGE1 = crate::Reg<sel_ag2_metric_range1::SEL_AG2_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range1;
#[doc = "SEL_AG3_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range1`] module"]
pub type SEL_AG3_METRIC_RANGE1 = crate::Reg<sel_ag3_metric_range1::SEL_AG3_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range1;
#[doc = "SEL_AG4_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range1`] module"]
pub type SEL_AG4_METRIC_RANGE1 = crate::Reg<sel_ag4_metric_range1::SEL_AG4_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range1;
#[doc = "SEL_AG5_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range1`] module"]
pub type SEL_AG5_METRIC_RANGE1 = crate::Reg<sel_ag5_metric_range1::SEL_AG5_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range1;
#[doc = "SEL_AG6_METRIC_RANGE1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range1`] module"]
pub type SEL_AG6_METRIC_RANGE1 = crate::Reg<sel_ag6_metric_range1::SEL_AG6_METRIC_RANGE1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range1;
#[doc = "SEL_AG0_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range2`] module"]
pub type SEL_AG0_METRIC_RANGE2 = crate::Reg<sel_ag0_metric_range2::SEL_AG0_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range2;
#[doc = "SEL_AG1_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range2`] module"]
pub type SEL_AG1_METRIC_RANGE2 = crate::Reg<sel_ag1_metric_range2::SEL_AG1_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range2;
#[doc = "SEL_AG2_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range2`] module"]
pub type SEL_AG2_METRIC_RANGE2 = crate::Reg<sel_ag2_metric_range2::SEL_AG2_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range2;
#[doc = "SEL_AG3_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range2`] module"]
pub type SEL_AG3_METRIC_RANGE2 = crate::Reg<sel_ag3_metric_range2::SEL_AG3_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range2;
#[doc = "SEL_AG4_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range2`] module"]
pub type SEL_AG4_METRIC_RANGE2 = crate::Reg<sel_ag4_metric_range2::SEL_AG4_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range2;
#[doc = "SEL_AG5_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range2`] module"]
pub type SEL_AG5_METRIC_RANGE2 = crate::Reg<sel_ag5_metric_range2::SEL_AG5_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range2;
#[doc = "SEL_AG6_METRIC_RANGE2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range2`] module"]
pub type SEL_AG6_METRIC_RANGE2 = crate::Reg<sel_ag6_metric_range2::SEL_AG6_METRIC_RANGE2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range2;
#[doc = "SEL_AG0_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range3`] module"]
pub type SEL_AG0_METRIC_RANGE3 = crate::Reg<sel_ag0_metric_range3::SEL_AG0_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range3;
#[doc = "SEL_AG1_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range3`] module"]
pub type SEL_AG1_METRIC_RANGE3 = crate::Reg<sel_ag1_metric_range3::SEL_AG1_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range3;
#[doc = "SEL_AG2_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range3`] module"]
pub type SEL_AG2_METRIC_RANGE3 = crate::Reg<sel_ag2_metric_range3::SEL_AG2_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range3;
#[doc = "SEL_AG3_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range3`] module"]
pub type SEL_AG3_METRIC_RANGE3 = crate::Reg<sel_ag3_metric_range3::SEL_AG3_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range3;
#[doc = "SEL_AG4_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range3`] module"]
pub type SEL_AG4_METRIC_RANGE3 = crate::Reg<sel_ag4_metric_range3::SEL_AG4_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range3;
#[doc = "SEL_AG5_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range3`] module"]
pub type SEL_AG5_METRIC_RANGE3 = crate::Reg<sel_ag5_metric_range3::SEL_AG5_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range3;
#[doc = "SEL_AG6_METRIC_RANGE3 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range3`] module"]
pub type SEL_AG6_METRIC_RANGE3 = crate::Reg<sel_ag6_metric_range3::SEL_AG6_METRIC_RANGE3_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range3;
#[doc = "SEL_AG0_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range4`] module"]
pub type SEL_AG0_METRIC_RANGE4 = crate::Reg<sel_ag0_metric_range4::SEL_AG0_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range4;
#[doc = "SEL_AG1_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range4`] module"]
pub type SEL_AG1_METRIC_RANGE4 = crate::Reg<sel_ag1_metric_range4::SEL_AG1_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range4;
#[doc = "SEL_AG2_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range4`] module"]
pub type SEL_AG2_METRIC_RANGE4 = crate::Reg<sel_ag2_metric_range4::SEL_AG2_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range4;
#[doc = "SEL_AG3_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range4`] module"]
pub type SEL_AG3_METRIC_RANGE4 = crate::Reg<sel_ag3_metric_range4::SEL_AG3_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range4;
#[doc = "SEL_AG4_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range4`] module"]
pub type SEL_AG4_METRIC_RANGE4 = crate::Reg<sel_ag4_metric_range4::SEL_AG4_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range4;
#[doc = "SEL_AG5_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range4`] module"]
pub type SEL_AG5_METRIC_RANGE4 = crate::Reg<sel_ag5_metric_range4::SEL_AG5_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range4;
#[doc = "SEL_AG6_METRIC_RANGE4 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range4`] module"]
pub type SEL_AG6_METRIC_RANGE4 = crate::Reg<sel_ag6_metric_range4::SEL_AG6_METRIC_RANGE4_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range4;
#[doc = "SEL_AG0_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range5`] module"]
pub type SEL_AG0_METRIC_RANGE5 = crate::Reg<sel_ag0_metric_range5::SEL_AG0_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range5;
#[doc = "SEL_AG1_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range5`] module"]
pub type SEL_AG1_METRIC_RANGE5 = crate::Reg<sel_ag1_metric_range5::SEL_AG1_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range5;
#[doc = "SEL_AG2_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range5`] module"]
pub type SEL_AG2_METRIC_RANGE5 = crate::Reg<sel_ag2_metric_range5::SEL_AG2_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range5;
#[doc = "SEL_AG3_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range5`] module"]
pub type SEL_AG3_METRIC_RANGE5 = crate::Reg<sel_ag3_metric_range5::SEL_AG3_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range5;
#[doc = "SEL_AG4_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range5`] module"]
pub type SEL_AG4_METRIC_RANGE5 = crate::Reg<sel_ag4_metric_range5::SEL_AG4_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range5;
#[doc = "SEL_AG5_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range5`] module"]
pub type SEL_AG5_METRIC_RANGE5 = crate::Reg<sel_ag5_metric_range5::SEL_AG5_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range5;
#[doc = "SEL_AG6_METRIC_RANGE5 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range5`] module"]
pub type SEL_AG6_METRIC_RANGE5 = crate::Reg<sel_ag6_metric_range5::SEL_AG6_METRIC_RANGE5_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range5;
#[doc = "SEL_AG0_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range6`] module"]
pub type SEL_AG0_METRIC_RANGE6 = crate::Reg<sel_ag0_metric_range6::SEL_AG0_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range6;
#[doc = "SEL_AG1_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range6`] module"]
pub type SEL_AG1_METRIC_RANGE6 = crate::Reg<sel_ag1_metric_range6::SEL_AG1_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range6;
#[doc = "SEL_AG2_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range6`] module"]
pub type SEL_AG2_METRIC_RANGE6 = crate::Reg<sel_ag2_metric_range6::SEL_AG2_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range6;
#[doc = "SEL_AG3_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range6`] module"]
pub type SEL_AG3_METRIC_RANGE6 = crate::Reg<sel_ag3_metric_range6::SEL_AG3_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range6;
#[doc = "SEL_AG4_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range6`] module"]
pub type SEL_AG4_METRIC_RANGE6 = crate::Reg<sel_ag4_metric_range6::SEL_AG4_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range6;
#[doc = "SEL_AG5_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range6`] module"]
pub type SEL_AG5_METRIC_RANGE6 = crate::Reg<sel_ag5_metric_range6::SEL_AG5_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range6;
#[doc = "SEL_AG6_METRIC_RANGE6 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range6`] module"]
pub type SEL_AG6_METRIC_RANGE6 = crate::Reg<sel_ag6_metric_range6::SEL_AG6_METRIC_RANGE6_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range6;
#[doc = "SEL_AG0_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_range7`] module"]
pub type SEL_AG0_METRIC_RANGE7 = crate::Reg<sel_ag0_metric_range7::SEL_AG0_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_range7;
#[doc = "SEL_AG1_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_range7`] module"]
pub type SEL_AG1_METRIC_RANGE7 = crate::Reg<sel_ag1_metric_range7::SEL_AG1_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_range7;
#[doc = "SEL_AG2_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_range7`] module"]
pub type SEL_AG2_METRIC_RANGE7 = crate::Reg<sel_ag2_metric_range7::SEL_AG2_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_range7;
#[doc = "SEL_AG3_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_range7`] module"]
pub type SEL_AG3_METRIC_RANGE7 = crate::Reg<sel_ag3_metric_range7::SEL_AG3_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_range7;
#[doc = "SEL_AG4_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_range7`] module"]
pub type SEL_AG4_METRIC_RANGE7 = crate::Reg<sel_ag4_metric_range7::SEL_AG4_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_range7;
#[doc = "SEL_AG5_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_range7`] module"]
pub type SEL_AG5_METRIC_RANGE7 = crate::Reg<sel_ag5_metric_range7::SEL_AG5_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_range7;
#[doc = "SEL_AG6_METRIC_RANGE7 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_range7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_range7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_range7`] module"]
pub type SEL_AG6_METRIC_RANGE7 = crate::Reg<sel_ag6_metric_range7::SEL_AG6_METRIC_RANGE7_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_range7;
#[doc = "SEL_AG0_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_addr_mask0`] module"]
pub type SEL_AG0_RD_ADDR_MASK0 = crate::Reg<sel_ag0_rd_addr_mask0::SEL_AG0_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_addr_mask0;
#[doc = "SEL_AG1_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_addr_mask0`] module"]
pub type SEL_AG1_RD_ADDR_MASK0 = crate::Reg<sel_ag1_rd_addr_mask0::SEL_AG1_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_addr_mask0;
#[doc = "SEL_AG2_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_addr_mask0`] module"]
pub type SEL_AG2_RD_ADDR_MASK0 = crate::Reg<sel_ag2_rd_addr_mask0::SEL_AG2_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_addr_mask0;
#[doc = "SEL_AG3_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_addr_mask0`] module"]
pub type SEL_AG3_RD_ADDR_MASK0 = crate::Reg<sel_ag3_rd_addr_mask0::SEL_AG3_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_addr_mask0;
#[doc = "SEL_AG4_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_addr_mask0`] module"]
pub type SEL_AG4_RD_ADDR_MASK0 = crate::Reg<sel_ag4_rd_addr_mask0::SEL_AG4_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_addr_mask0;
#[doc = "SEL_AG5_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_addr_mask0`] module"]
pub type SEL_AG5_RD_ADDR_MASK0 = crate::Reg<sel_ag5_rd_addr_mask0::SEL_AG5_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_addr_mask0;
#[doc = "SEL_AG6_RD_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_addr_mask0`] module"]
pub type SEL_AG6_RD_ADDR_MASK0 = crate::Reg<sel_ag6_rd_addr_mask0::SEL_AG6_RD_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_addr_mask0;
#[doc = "SEL_AG0_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_addr_mask1`] module"]
pub type SEL_AG0_RD_ADDR_MASK1 = crate::Reg<sel_ag0_rd_addr_mask1::SEL_AG0_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_addr_mask1;
#[doc = "SEL_AG1_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_addr_mask1`] module"]
pub type SEL_AG1_RD_ADDR_MASK1 = crate::Reg<sel_ag1_rd_addr_mask1::SEL_AG1_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_addr_mask1;
#[doc = "SEL_AG2_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_addr_mask1`] module"]
pub type SEL_AG2_RD_ADDR_MASK1 = crate::Reg<sel_ag2_rd_addr_mask1::SEL_AG2_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_addr_mask1;
#[doc = "SEL_AG3_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_addr_mask1`] module"]
pub type SEL_AG3_RD_ADDR_MASK1 = crate::Reg<sel_ag3_rd_addr_mask1::SEL_AG3_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_addr_mask1;
#[doc = "SEL_AG4_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_addr_mask1`] module"]
pub type SEL_AG4_RD_ADDR_MASK1 = crate::Reg<sel_ag4_rd_addr_mask1::SEL_AG4_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_addr_mask1;
#[doc = "SEL_AG5_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_addr_mask1`] module"]
pub type SEL_AG5_RD_ADDR_MASK1 = crate::Reg<sel_ag5_rd_addr_mask1::SEL_AG5_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_addr_mask1;
#[doc = "SEL_AG6_RD_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_addr_mask1`] module"]
pub type SEL_AG6_RD_ADDR_MASK1 = crate::Reg<sel_ag6_rd_addr_mask1::SEL_AG6_RD_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_addr_mask1;
#[doc = "SEL_AG0_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_addr_mask2`] module"]
pub type SEL_AG0_RD_ADDR_MASK2 = crate::Reg<sel_ag0_rd_addr_mask2::SEL_AG0_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_addr_mask2;
#[doc = "SEL_AG1_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_addr_mask2`] module"]
pub type SEL_AG1_RD_ADDR_MASK2 = crate::Reg<sel_ag1_rd_addr_mask2::SEL_AG1_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_addr_mask2;
#[doc = "SEL_AG2_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_addr_mask2`] module"]
pub type SEL_AG2_RD_ADDR_MASK2 = crate::Reg<sel_ag2_rd_addr_mask2::SEL_AG2_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_addr_mask2;
#[doc = "SEL_AG3_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_addr_mask2`] module"]
pub type SEL_AG3_RD_ADDR_MASK2 = crate::Reg<sel_ag3_rd_addr_mask2::SEL_AG3_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_addr_mask2;
#[doc = "SEL_AG4_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_addr_mask2`] module"]
pub type SEL_AG4_RD_ADDR_MASK2 = crate::Reg<sel_ag4_rd_addr_mask2::SEL_AG4_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_addr_mask2;
#[doc = "SEL_AG5_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_addr_mask2`] module"]
pub type SEL_AG5_RD_ADDR_MASK2 = crate::Reg<sel_ag5_rd_addr_mask2::SEL_AG5_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_addr_mask2;
#[doc = "SEL_AG6_RD_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_addr_mask2`] module"]
pub type SEL_AG6_RD_ADDR_MASK2 = crate::Reg<sel_ag6_rd_addr_mask2::SEL_AG6_RD_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_addr_mask2;
#[doc = "SEL_AG0_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_addr_mask0`] module"]
pub type SEL_AG0_WR_ADDR_MASK0 = crate::Reg<sel_ag0_wr_addr_mask0::SEL_AG0_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_addr_mask0;
#[doc = "SEL_AG1_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_addr_mask0`] module"]
pub type SEL_AG1_WR_ADDR_MASK0 = crate::Reg<sel_ag1_wr_addr_mask0::SEL_AG1_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_addr_mask0;
#[doc = "SEL_AG2_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_addr_mask0`] module"]
pub type SEL_AG2_WR_ADDR_MASK0 = crate::Reg<sel_ag2_wr_addr_mask0::SEL_AG2_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_addr_mask0;
#[doc = "SEL_AG3_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_addr_mask0`] module"]
pub type SEL_AG3_WR_ADDR_MASK0 = crate::Reg<sel_ag3_wr_addr_mask0::SEL_AG3_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_addr_mask0;
#[doc = "SEL_AG4_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_addr_mask0`] module"]
pub type SEL_AG4_WR_ADDR_MASK0 = crate::Reg<sel_ag4_wr_addr_mask0::SEL_AG4_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_addr_mask0;
#[doc = "SEL_AG5_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_addr_mask0`] module"]
pub type SEL_AG5_WR_ADDR_MASK0 = crate::Reg<sel_ag5_wr_addr_mask0::SEL_AG5_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_addr_mask0;
#[doc = "SEL_AG6_WR_ADDR_MASK0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_addr_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_addr_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_addr_mask0`] module"]
pub type SEL_AG6_WR_ADDR_MASK0 = crate::Reg<sel_ag6_wr_addr_mask0::SEL_AG6_WR_ADDR_MASK0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_addr_mask0;
#[doc = "SEL_AG0_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_addr_mask1`] module"]
pub type SEL_AG0_WR_ADDR_MASK1 = crate::Reg<sel_ag0_wr_addr_mask1::SEL_AG0_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_addr_mask1;
#[doc = "SEL_AG1_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_addr_mask1`] module"]
pub type SEL_AG1_WR_ADDR_MASK1 = crate::Reg<sel_ag1_wr_addr_mask1::SEL_AG1_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_addr_mask1;
#[doc = "SEL_AG2_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_addr_mask1`] module"]
pub type SEL_AG2_WR_ADDR_MASK1 = crate::Reg<sel_ag2_wr_addr_mask1::SEL_AG2_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_addr_mask1;
#[doc = "SEL_AG3_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_addr_mask1`] module"]
pub type SEL_AG3_WR_ADDR_MASK1 = crate::Reg<sel_ag3_wr_addr_mask1::SEL_AG3_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_addr_mask1;
#[doc = "SEL_AG4_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_addr_mask1`] module"]
pub type SEL_AG4_WR_ADDR_MASK1 = crate::Reg<sel_ag4_wr_addr_mask1::SEL_AG4_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_addr_mask1;
#[doc = "SEL_AG5_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_addr_mask1`] module"]
pub type SEL_AG5_WR_ADDR_MASK1 = crate::Reg<sel_ag5_wr_addr_mask1::SEL_AG5_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_addr_mask1;
#[doc = "SEL_AG6_WR_ADDR_MASK1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_addr_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_addr_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_addr_mask1`] module"]
pub type SEL_AG6_WR_ADDR_MASK1 = crate::Reg<sel_ag6_wr_addr_mask1::SEL_AG6_WR_ADDR_MASK1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_addr_mask1;
#[doc = "SEL_AG0_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_addr_mask2`] module"]
pub type SEL_AG0_WR_ADDR_MASK2 = crate::Reg<sel_ag0_wr_addr_mask2::SEL_AG0_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_addr_mask2;
#[doc = "SEL_AG1_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_addr_mask2`] module"]
pub type SEL_AG1_WR_ADDR_MASK2 = crate::Reg<sel_ag1_wr_addr_mask2::SEL_AG1_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_addr_mask2;
#[doc = "SEL_AG2_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_addr_mask2`] module"]
pub type SEL_AG2_WR_ADDR_MASK2 = crate::Reg<sel_ag2_wr_addr_mask2::SEL_AG2_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_addr_mask2;
#[doc = "SEL_AG3_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_addr_mask2`] module"]
pub type SEL_AG3_WR_ADDR_MASK2 = crate::Reg<sel_ag3_wr_addr_mask2::SEL_AG3_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_addr_mask2;
#[doc = "SEL_AG4_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_addr_mask2`] module"]
pub type SEL_AG4_WR_ADDR_MASK2 = crate::Reg<sel_ag4_wr_addr_mask2::SEL_AG4_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_addr_mask2;
#[doc = "SEL_AG5_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_addr_mask2`] module"]
pub type SEL_AG5_WR_ADDR_MASK2 = crate::Reg<sel_ag5_wr_addr_mask2::SEL_AG5_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_addr_mask2;
#[doc = "SEL_AG6_WR_ADDR_MASK2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_addr_mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_addr_mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_addr_mask2`] module"]
pub type SEL_AG6_WR_ADDR_MASK2 = crate::Reg<sel_ag6_wr_addr_mask2::SEL_AG6_WR_ADDR_MASK2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_addr_mask2;
#[doc = "SEL_AG0_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_addr_filter0`] module"]
pub type SEL_AG0_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag0_rd_addr_filter0::SEL_AG0_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_addr_filter0;
#[doc = "SEL_AG1_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_addr_filter0`] module"]
pub type SEL_AG1_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag1_rd_addr_filter0::SEL_AG1_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_addr_filter0;
#[doc = "SEL_AG2_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_addr_filter0`] module"]
pub type SEL_AG2_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag2_rd_addr_filter0::SEL_AG2_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_addr_filter0;
#[doc = "SEL_AG3_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_addr_filter0`] module"]
pub type SEL_AG3_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag3_rd_addr_filter0::SEL_AG3_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_addr_filter0;
#[doc = "SEL_AG4_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_addr_filter0`] module"]
pub type SEL_AG4_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag4_rd_addr_filter0::SEL_AG4_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_addr_filter0;
#[doc = "SEL_AG5_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_addr_filter0`] module"]
pub type SEL_AG5_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag5_rd_addr_filter0::SEL_AG5_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_addr_filter0;
#[doc = "SEL_AG6_RD_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_addr_filter0`] module"]
pub type SEL_AG6_RD_ADDR_FILTER0 =
    crate::Reg<sel_ag6_rd_addr_filter0::SEL_AG6_RD_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_addr_filter0;
#[doc = "SEL_AG0_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_addr_filter1`] module"]
pub type SEL_AG0_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag0_rd_addr_filter1::SEL_AG0_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_addr_filter1;
#[doc = "SEL_AG1_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_addr_filter1`] module"]
pub type SEL_AG1_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag1_rd_addr_filter1::SEL_AG1_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_addr_filter1;
#[doc = "SEL_AG2_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_addr_filter1`] module"]
pub type SEL_AG2_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag2_rd_addr_filter1::SEL_AG2_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_addr_filter1;
#[doc = "SEL_AG3_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_addr_filter1`] module"]
pub type SEL_AG3_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag3_rd_addr_filter1::SEL_AG3_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_addr_filter1;
#[doc = "SEL_AG4_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_addr_filter1`] module"]
pub type SEL_AG4_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag4_rd_addr_filter1::SEL_AG4_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_addr_filter1;
#[doc = "SEL_AG5_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_addr_filter1`] module"]
pub type SEL_AG5_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag5_rd_addr_filter1::SEL_AG5_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_addr_filter1;
#[doc = "SEL_AG6_RD_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_addr_filter1`] module"]
pub type SEL_AG6_RD_ADDR_FILTER1 =
    crate::Reg<sel_ag6_rd_addr_filter1::SEL_AG6_RD_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_addr_filter1;
#[doc = "SEL_AG0_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_addr_filter2`] module"]
pub type SEL_AG0_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag0_rd_addr_filter2::SEL_AG0_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_addr_filter2;
#[doc = "SEL_AG1_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_addr_filter2`] module"]
pub type SEL_AG1_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag1_rd_addr_filter2::SEL_AG1_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_addr_filter2;
#[doc = "SEL_AG2_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_addr_filter2`] module"]
pub type SEL_AG2_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag2_rd_addr_filter2::SEL_AG2_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_addr_filter2;
#[doc = "SEL_AG3_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_addr_filter2`] module"]
pub type SEL_AG3_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag3_rd_addr_filter2::SEL_AG3_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_addr_filter2;
#[doc = "SEL_AG4_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_addr_filter2`] module"]
pub type SEL_AG4_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag4_rd_addr_filter2::SEL_AG4_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_addr_filter2;
#[doc = "SEL_AG5_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_addr_filter2`] module"]
pub type SEL_AG5_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag5_rd_addr_filter2::SEL_AG5_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_addr_filter2;
#[doc = "SEL_AG6_RD_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_addr_filter2`] module"]
pub type SEL_AG6_RD_ADDR_FILTER2 =
    crate::Reg<sel_ag6_rd_addr_filter2::SEL_AG6_RD_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_addr_filter2;
#[doc = "SEL_AG0_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_addr_filter0`] module"]
pub type SEL_AG0_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag0_wr_addr_filter0::SEL_AG0_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_addr_filter0;
#[doc = "SEL_AG1_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_addr_filter0`] module"]
pub type SEL_AG1_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag1_wr_addr_filter0::SEL_AG1_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_addr_filter0;
#[doc = "SEL_AG2_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_addr_filter0`] module"]
pub type SEL_AG2_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag2_wr_addr_filter0::SEL_AG2_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_addr_filter0;
#[doc = "SEL_AG3_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_addr_filter0`] module"]
pub type SEL_AG3_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag3_wr_addr_filter0::SEL_AG3_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_addr_filter0;
#[doc = "SEL_AG4_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_addr_filter0`] module"]
pub type SEL_AG4_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag4_wr_addr_filter0::SEL_AG4_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_addr_filter0;
#[doc = "SEL_AG5_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_addr_filter0`] module"]
pub type SEL_AG5_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag5_wr_addr_filter0::SEL_AG5_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_addr_filter0;
#[doc = "SEL_AG6_WR_ADDR_FILTER0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_addr_filter0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_addr_filter0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_addr_filter0`] module"]
pub type SEL_AG6_WR_ADDR_FILTER0 =
    crate::Reg<sel_ag6_wr_addr_filter0::SEL_AG6_WR_ADDR_FILTER0_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_addr_filter0;
#[doc = "SEL_AG0_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_addr_filter1`] module"]
pub type SEL_AG0_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag0_wr_addr_filter1::SEL_AG0_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_addr_filter1;
#[doc = "SEL_AG1_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_addr_filter1`] module"]
pub type SEL_AG1_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag1_wr_addr_filter1::SEL_AG1_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_addr_filter1;
#[doc = "SEL_AG2_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_addr_filter1`] module"]
pub type SEL_AG2_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag2_wr_addr_filter1::SEL_AG2_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_addr_filter1;
#[doc = "SEL_AG3_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_addr_filter1`] module"]
pub type SEL_AG3_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag3_wr_addr_filter1::SEL_AG3_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_addr_filter1;
#[doc = "SEL_AG4_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_addr_filter1`] module"]
pub type SEL_AG4_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag4_wr_addr_filter1::SEL_AG4_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_addr_filter1;
#[doc = "SEL_AG5_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_addr_filter1`] module"]
pub type SEL_AG5_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag5_wr_addr_filter1::SEL_AG5_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_addr_filter1;
#[doc = "SEL_AG6_WR_ADDR_FILTER1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_addr_filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_addr_filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_addr_filter1`] module"]
pub type SEL_AG6_WR_ADDR_FILTER1 =
    crate::Reg<sel_ag6_wr_addr_filter1::SEL_AG6_WR_ADDR_FILTER1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_addr_filter1;
#[doc = "SEL_AG0_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_addr_filter2`] module"]
pub type SEL_AG0_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag0_wr_addr_filter2::SEL_AG0_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_addr_filter2;
#[doc = "SEL_AG1_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_addr_filter2`] module"]
pub type SEL_AG1_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag1_wr_addr_filter2::SEL_AG1_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_addr_filter2;
#[doc = "SEL_AG2_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_addr_filter2`] module"]
pub type SEL_AG2_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag2_wr_addr_filter2::SEL_AG2_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_addr_filter2;
#[doc = "SEL_AG3_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_addr_filter2`] module"]
pub type SEL_AG3_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag3_wr_addr_filter2::SEL_AG3_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_addr_filter2;
#[doc = "SEL_AG4_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_addr_filter2`] module"]
pub type SEL_AG4_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag4_wr_addr_filter2::SEL_AG4_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_addr_filter2;
#[doc = "SEL_AG5_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_addr_filter2`] module"]
pub type SEL_AG5_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag5_wr_addr_filter2::SEL_AG5_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_addr_filter2;
#[doc = "SEL_AG6_WR_ADDR_FILTER2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_addr_filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_addr_filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_addr_filter2`] module"]
pub type SEL_AG6_WR_ADDR_FILTER2 =
    crate::Reg<sel_ag6_wr_addr_filter2::SEL_AG6_WR_ADDR_FILTER2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_addr_filter2;
#[doc = "SEL_AG0_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_select1`] module"]
pub type SEL_AG0_METRIC_SELECT1 = crate::Reg<sel_ag0_metric_select1::SEL_AG0_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_select1;
#[doc = "SEL_AG1_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_select1`] module"]
pub type SEL_AG1_METRIC_SELECT1 = crate::Reg<sel_ag1_metric_select1::SEL_AG1_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_select1;
#[doc = "SEL_AG2_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_select1`] module"]
pub type SEL_AG2_METRIC_SELECT1 = crate::Reg<sel_ag2_metric_select1::SEL_AG2_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_select1;
#[doc = "SEL_AG3_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_select1`] module"]
pub type SEL_AG3_METRIC_SELECT1 = crate::Reg<sel_ag3_metric_select1::SEL_AG3_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_select1;
#[doc = "SEL_AG4_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_select1`] module"]
pub type SEL_AG4_METRIC_SELECT1 = crate::Reg<sel_ag4_metric_select1::SEL_AG4_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_select1;
#[doc = "SEL_AG5_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_select1`] module"]
pub type SEL_AG5_METRIC_SELECT1 = crate::Reg<sel_ag5_metric_select1::SEL_AG5_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_select1;
#[doc = "SEL_AG6_METRIC_SELECT1 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_select1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_select1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_select1`] module"]
pub type SEL_AG6_METRIC_SELECT1 = crate::Reg<sel_ag6_metric_select1::SEL_AG6_METRIC_SELECT1_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_select1;
#[doc = "SEL_AG0_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_metric_select2`] module"]
pub type SEL_AG0_METRIC_SELECT2 = crate::Reg<sel_ag0_metric_select2::SEL_AG0_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_metric_select2;
#[doc = "SEL_AG1_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_metric_select2`] module"]
pub type SEL_AG1_METRIC_SELECT2 = crate::Reg<sel_ag1_metric_select2::SEL_AG1_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_metric_select2;
#[doc = "SEL_AG2_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_metric_select2`] module"]
pub type SEL_AG2_METRIC_SELECT2 = crate::Reg<sel_ag2_metric_select2::SEL_AG2_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_metric_select2;
#[doc = "SEL_AG3_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_metric_select2`] module"]
pub type SEL_AG3_METRIC_SELECT2 = crate::Reg<sel_ag3_metric_select2::SEL_AG3_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_metric_select2;
#[doc = "SEL_AG4_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_metric_select2`] module"]
pub type SEL_AG4_METRIC_SELECT2 = crate::Reg<sel_ag4_metric_select2::SEL_AG4_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_metric_select2;
#[doc = "SEL_AG5_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_metric_select2`] module"]
pub type SEL_AG5_METRIC_SELECT2 = crate::Reg<sel_ag5_metric_select2::SEL_AG5_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_metric_select2;
#[doc = "SEL_AG6_METRIC_SELECT2 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_metric_select2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_metric_select2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_metric_select2`] module"]
pub type SEL_AG6_METRIC_SELECT2 = crate::Reg<sel_ag6_metric_select2::SEL_AG6_METRIC_SELECT2_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_metric_select2;
#[doc = "SEL_AG_RD_ADDR_REGION_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_rd_addr_region_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_rd_addr_region_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_rd_addr_region_sel`] module"]
pub type SEL_AG_RD_ADDR_REGION_SEL =
    crate::Reg<sel_ag_rd_addr_region_sel::SEL_AG_RD_ADDR_REGION_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_rd_addr_region_sel;
#[doc = "SEL_AG_WR_ADDR_REGION_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_wr_addr_region_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_wr_addr_region_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_wr_addr_region_sel`] module"]
pub type SEL_AG_WR_ADDR_REGION_SEL =
    crate::Reg<sel_ag_wr_addr_region_sel::SEL_AG_WR_ADDR_REGION_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_wr_addr_region_sel;
#[doc = "SEL_AG_ADDR_FILTER_EN (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_addr_filter_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_addr_filter_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_addr_filter_en`] module"]
pub type SEL_AG_ADDR_FILTER_EN = crate::Reg<sel_ag_addr_filter_en::SEL_AG_ADDR_FILTER_EN_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_addr_filter_en;
#[doc = "SEL_AG_SW_RECORD_STOP_EN (w) register accessor: reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_sw_record_stop_en::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_sw_record_stop_en`] module"]
pub type SEL_AG_SW_RECORD_STOP_EN =
    crate::Reg<sel_ag_sw_record_stop_en::SEL_AG_SW_RECORD_STOP_EN_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_sw_record_stop_en;
#[doc = "SEL_AG_SW_RECORD_STOP_CLR (w) register accessor: reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_sw_record_stop_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_sw_record_stop_clr`] module"]
pub type SEL_AG_SW_RECORD_STOP_CLR =
    crate::Reg<sel_ag_sw_record_stop_clr::SEL_AG_SW_RECORD_STOP_CLR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_sw_record_stop_clr;
#[doc = "SEL_AG_INS_BANDW_TEST_EN (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_ins_bandw_test_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_ins_bandw_test_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_ins_bandw_test_en`] module"]
pub type SEL_AG_INS_BANDW_TEST_EN =
    crate::Reg<sel_ag_ins_bandw_test_en::SEL_AG_INS_BANDW_TEST_EN_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_ins_bandw_test_en;
#[doc = "SEL_AG0_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_sw_record_stop_data_limit`] module"]
pub type SEL_AG0_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag0_sw_record_stop_data_limit::SEL_AG0_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_sw_record_stop_data_limit;
#[doc = "SEL_AG1_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_sw_record_stop_data_limit`] module"]
pub type SEL_AG1_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag1_sw_record_stop_data_limit::SEL_AG1_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_sw_record_stop_data_limit;
#[doc = "SEL_AG2_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_sw_record_stop_data_limit`] module"]
pub type SEL_AG2_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag2_sw_record_stop_data_limit::SEL_AG2_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_sw_record_stop_data_limit;
#[doc = "SEL_AG3_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_sw_record_stop_data_limit`] module"]
pub type SEL_AG3_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag3_sw_record_stop_data_limit::SEL_AG3_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_sw_record_stop_data_limit;
#[doc = "SEL_AG4_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_sw_record_stop_data_limit`] module"]
pub type SEL_AG4_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag4_sw_record_stop_data_limit::SEL_AG4_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_sw_record_stop_data_limit;
#[doc = "SEL_AG5_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_sw_record_stop_data_limit`] module"]
pub type SEL_AG5_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag5_sw_record_stop_data_limit::SEL_AG5_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_sw_record_stop_data_limit;
#[doc = "SEL_AG6_SW_RECORD_STOP_DATA_LIMIT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_sw_record_stop_data_limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_sw_record_stop_data_limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_sw_record_stop_data_limit`] module"]
pub type SEL_AG6_SW_RECORD_STOP_DATA_LIMIT =
    crate::Reg<sel_ag6_sw_record_stop_data_limit::SEL_AG6_SW_RECORD_STOP_DATA_LIMIT_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_sw_record_stop_data_limit;
#[doc = "SEL_AG0_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_id_mask`] module"]
pub type SEL_AG0_ID_MASK = crate::Reg<sel_ag0_id_mask::SEL_AG0_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_id_mask;
#[doc = "SEL_AG1_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_id_mask`] module"]
pub type SEL_AG1_ID_MASK = crate::Reg<sel_ag1_id_mask::SEL_AG1_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_id_mask;
#[doc = "SEL_AG2_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_id_mask`] module"]
pub type SEL_AG2_ID_MASK = crate::Reg<sel_ag2_id_mask::SEL_AG2_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_id_mask;
#[doc = "SEL_AG3_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_id_mask`] module"]
pub type SEL_AG3_ID_MASK = crate::Reg<sel_ag3_id_mask::SEL_AG3_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_id_mask;
#[doc = "SEL_AG4_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_id_mask`] module"]
pub type SEL_AG4_ID_MASK = crate::Reg<sel_ag4_id_mask::SEL_AG4_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_id_mask;
#[doc = "SEL_AG5_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_id_mask`] module"]
pub type SEL_AG5_ID_MASK = crate::Reg<sel_ag5_id_mask::SEL_AG5_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_id_mask;
#[doc = "SEL_AG6_ID_MASK (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_id_mask`] module"]
pub type SEL_AG6_ID_MASK = crate::Reg<sel_ag6_id_mask::SEL_AG6_ID_MASK_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_id_mask;
#[doc = "SEL_AG0_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_id_filter`] module"]
pub type SEL_AG0_ID_FILTER = crate::Reg<sel_ag0_id_filter::SEL_AG0_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_id_filter;
#[doc = "SEL_AG1_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_id_filter`] module"]
pub type SEL_AG1_ID_FILTER = crate::Reg<sel_ag1_id_filter::SEL_AG1_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_id_filter;
#[doc = "SEL_AG2_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_id_filter`] module"]
pub type SEL_AG2_ID_FILTER = crate::Reg<sel_ag2_id_filter::SEL_AG2_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_id_filter;
#[doc = "SEL_AG3_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_id_filter`] module"]
pub type SEL_AG3_ID_FILTER = crate::Reg<sel_ag3_id_filter::SEL_AG3_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_id_filter;
#[doc = "SEL_AG4_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_id_filter`] module"]
pub type SEL_AG4_ID_FILTER = crate::Reg<sel_ag4_id_filter::SEL_AG4_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_id_filter;
#[doc = "SEL_AG5_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_id_filter`] module"]
pub type SEL_AG5_ID_FILTER = crate::Reg<sel_ag5_id_filter::SEL_AG5_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_id_filter;
#[doc = "SEL_AG6_ID_FILTER (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_id_filter`] module"]
pub type SEL_AG6_ID_FILTER = crate::Reg<sel_ag6_id_filter::SEL_AG6_ID_FILTER_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_id_filter;
#[doc = "SEL_AG_BANDW_TEST_EN (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_bandw_test_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_bandw_test_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_bandw_test_en`] module"]
pub type SEL_AG_BANDW_TEST_EN = crate::Reg<sel_ag_bandw_test_en::SEL_AG_BANDW_TEST_EN_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_bandw_test_en;
#[doc = "SEL_AG_BANDW_TEST_STOP (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_bandw_test_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_bandw_test_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_bandw_test_stop`] module"]
pub type SEL_AG_BANDW_TEST_STOP = crate::Reg<sel_ag_bandw_test_stop::SEL_AG_BANDW_TEST_STOP_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_bandw_test_stop;
#[doc = "SEL_AG0_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_bandw_trigger_in_sel`] module"]
pub type SEL_AG0_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag0_bandw_trigger_in_sel::SEL_AG0_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_bandw_trigger_in_sel;
#[doc = "SEL_AG1_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_bandw_trigger_in_sel`] module"]
pub type SEL_AG1_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag1_bandw_trigger_in_sel::SEL_AG1_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_bandw_trigger_in_sel;
#[doc = "SEL_AG2_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_bandw_trigger_in_sel`] module"]
pub type SEL_AG2_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag2_bandw_trigger_in_sel::SEL_AG2_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_bandw_trigger_in_sel;
#[doc = "SEL_AG3_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_bandw_trigger_in_sel`] module"]
pub type SEL_AG3_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag3_bandw_trigger_in_sel::SEL_AG3_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_bandw_trigger_in_sel;
#[doc = "SEL_AG4_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_bandw_trigger_in_sel`] module"]
pub type SEL_AG4_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag4_bandw_trigger_in_sel::SEL_AG4_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_bandw_trigger_in_sel;
#[doc = "SEL_AG5_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_bandw_trigger_in_sel`] module"]
pub type SEL_AG5_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag5_bandw_trigger_in_sel::SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_bandw_trigger_in_sel;
#[doc = "SEL_AG6_BANDW_TRIGGER_IN_SEL (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_bandw_trigger_in_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_bandw_trigger_in_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_bandw_trigger_in_sel`] module"]
pub type SEL_AG6_BANDW_TRIGGER_IN_SEL =
    crate::Reg<sel_ag6_bandw_trigger_in_sel::SEL_AG6_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_bandw_trigger_in_sel;
#[doc = "SEL_AG0_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG0_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag0_wr_bandw_cnt_valid_strobe_num::SEL_AG0_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag1_wr_bandw_cnt_valid_strobe_num::SEL_AG1_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG2_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG2_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag2_wr_bandw_cnt_valid_strobe_num::SEL_AG2_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG3_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG3_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag3_wr_bandw_cnt_valid_strobe_num::SEL_AG3_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG4_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG4_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag4_wr_bandw_cnt_valid_strobe_num::SEL_AG4_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG5_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG5_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag5_wr_bandw_cnt_valid_strobe_num::SEL_AG5_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG6_WR_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_wr_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_wr_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_wr_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG6_WR_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag6_wr_bandw_cnt_valid_strobe_num::SEL_AG6_WR_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_wr_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG0_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG0_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag0_rd_bandw_cnt_valid_strobe_num::SEL_AG0_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG1_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG1_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag1_rd_bandw_cnt_valid_strobe_num::SEL_AG1_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG2_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG2_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag2_rd_bandw_cnt_valid_strobe_num::SEL_AG2_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG3_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG3_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag3_rd_bandw_cnt_valid_strobe_num::SEL_AG3_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG4_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG4_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag4_rd_bandw_cnt_valid_strobe_num::SEL_AG4_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG5_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG5_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag5_rd_bandw_cnt_valid_strobe_num::SEL_AG5_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG6_RD_BANDW_CNT_VALID_STROBE_NUM (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_rd_bandw_cnt_valid_strobe_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_rd_bandw_cnt_valid_strobe_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_rd_bandw_cnt_valid_strobe_num`] module"]
pub type SEL_AG6_RD_BANDW_CNT_VALID_STROBE_NUM =
    crate::Reg<sel_ag6_rd_bandw_cnt_valid_strobe_num::SEL_AG6_RD_BANDW_CNT_VALID_STROBE_NUM_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_rd_bandw_cnt_valid_strobe_num;
#[doc = "SEL_AG0_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag0_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag0_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag0_ins_bandw_time_thr`] module"]
pub type SEL_AG0_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag0_ins_bandw_time_thr::SEL_AG0_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag0_ins_bandw_time_thr;
#[doc = "SEL_AG1_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag1_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag1_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag1_ins_bandw_time_thr`] module"]
pub type SEL_AG1_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag1_ins_bandw_time_thr::SEL_AG1_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag1_ins_bandw_time_thr;
#[doc = "SEL_AG2_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag2_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag2_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag2_ins_bandw_time_thr`] module"]
pub type SEL_AG2_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag2_ins_bandw_time_thr::SEL_AG2_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag2_ins_bandw_time_thr;
#[doc = "SEL_AG3_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag3_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag3_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag3_ins_bandw_time_thr`] module"]
pub type SEL_AG3_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag3_ins_bandw_time_thr::SEL_AG3_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag3_ins_bandw_time_thr;
#[doc = "SEL_AG4_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag4_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag4_ins_bandw_time_thr`] module"]
pub type SEL_AG4_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag4_ins_bandw_time_thr::SEL_AG4_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag4_ins_bandw_time_thr;
#[doc = "SEL_AG5_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag5_ins_bandw_time_thr`] module"]
pub type SEL_AG5_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag5_ins_bandw_time_thr::SEL_AG5_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag5_ins_bandw_time_thr;
#[doc = "SEL_AG6_INS_BANDW_TIME_THR (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag6_ins_bandw_time_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag6_ins_bandw_time_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag6_ins_bandw_time_thr`] module"]
pub type SEL_AG6_INS_BANDW_TIME_THR =
    crate::Reg<sel_ag6_ins_bandw_time_thr::SEL_AG6_INS_BANDW_TIME_THR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag6_ins_bandw_time_thr;
#[doc = "SEL_AG_INT_RAW (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_int_raw`] module"]
pub type SEL_AG_INT_RAW = crate::Reg<sel_ag_int_raw::SEL_AG_INT_RAW_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_int_raw;
#[doc = "SEL_AG_INT_ST (r) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_int_st`] module"]
pub type SEL_AG_INT_ST = crate::Reg<sel_ag_int_st::SEL_AG_INT_ST_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_int_st;
#[doc = "SEL_AG_INT_ENA (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_int_ena`] module"]
pub type SEL_AG_INT_ENA = crate::Reg<sel_ag_int_ena::SEL_AG_INT_ENA_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_int_ena;
#[doc = "SEL_AG_INT_CLR (w) register accessor: reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sel_ag_int_clr`] module"]
pub type SEL_AG_INT_CLR = crate::Reg<sel_ag_int_clr::SEL_AG_INT_CLR_SPEC>;
#[doc = "reserved"]
pub mod sel_ag_int_clr;
#[doc = "DATE (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "reserved"]
pub mod date;
